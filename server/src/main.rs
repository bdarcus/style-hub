mod auth;

use axum::{
    extract::{State, Query},
    routing::{get, post},
    Router,
    Json,
    response::{Redirect, IntoResponse},
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::{Value, json};
use csln_core::Style;
use csln_processor::{Processor, Reference, Bibliography, Citation, CitationItem};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use oauth2::{AuthorizationCode, TokenResponse};
use uuid::Uuid;

struct AppState {
    references: HashMap<String, Reference>,
    db: sqlx::PgPool,
    oauth_client: oauth2::basic::BasicClient,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let oauth_client = auth::create_oauth_client();

    // Load references from YAML
    let ref_path = "resources/comprehensive.yaml";
    // Try explicit path if running from root
    let ref_path = if std::path::Path::new("server/resources/comprehensive.yaml").exists() {
        "server/resources/comprehensive.yaml"
    } else {
        ref_path
    };

    let f = std::fs::File::open(ref_path).expect("Failed to open comprehensive.yaml references");
    let references: HashMap<String, Reference> = serde_yaml::from_reader(f).expect("Failed to parse comprehensive.yaml");

    let state = Arc::new(AppState {
        references,
        db: pool,
        oauth_client,
    });

    let app = Router::new()
        .route("/", get(health_check))
        .route("/version", get(version))
        .route("/auth/github", get(github_auth))
        .route("/auth/github/callback", get(github_callback))
        .route("/references", get(get_references))
        .route("/preview/citation", post(preview_citation))
        .route("/preview/bibliography", post(preview_bibliography))
        .route("/api/styles", get(list_styles).post(save_style))
        .route("/api/styles/:id", get(get_style))
        .route("/api/styles/:id/fork", post(fork_style))
        .route("/api/styles/:id/bookmark", post(add_bookmark).delete(remove_bookmark))
        .route("/api/bookmarks", get(list_bookmarks))
        .route("/api/hub", get(list_public_styles))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct AuthQuery {
    code: String,
    state: String,
}

async fn github_auth(State(state): State<Arc<AppState>>) -> Redirect {
    let (auth_url, _csrf_token) = state.oauth_client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("user:email".to_string()))
        .url();

    Redirect::to(auth_url.as_str())
}

async fn github_callback(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AuthQuery>,
) -> impl IntoResponse {
    let token_result = state.oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .expect("Failed to exchange token");

    let client = reqwest::Client::new();
    let github_user: auth::GithubUser = client
        .get("https://api.github.com/user")
        .header("User-Agent", "style-editor-server")
        .bearer_auth(token_result.access_token().secret())
        .send()
        .await
        .expect("Failed to fetch GitHub user")
        .json()
        .await
        .expect("Failed to parse GitHub user");

    let email = github_user.email.unwrap_or_else(|| format!("{}@github.com", github_user.login));

    // Upsert user
    let user = sqlx::query_as!(
        auth::User,
        r#"
        INSERT INTO users (email, github_id)
        VALUES ($1, $2)
        ON CONFLICT (github_id) DO UPDATE SET email = EXCLUDED.email
        RETURNING id, email, role
        "#,
        email,
        github_user.id.to_string()
    )
    .fetch_one(&state.db)
    .await
    .expect("Failed to upsert user");

    let jwt = auth::create_jwt(user.id, &user.role);
    
    // Redirect to frontend with token
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    Redirect::to(&format!("{}/auth/callback?token={}", frontend_url, jwt))
}

async fn health_check() -> &'static str {
    "OK"
}

async fn version() -> Json<Value> {
    Json(json!({
        "service": "style-editor-server",
        "csln_core_version": "git-latest"
    }))
}

async fn get_references(State(state): State<Arc<AppState>>) -> Json<HashMap<String, Reference>> {
    Json(state.references.clone())
}

#[derive(Deserialize)]
struct PreviewRequest {
    style: Style,
    references: Vec<Reference>,
}

#[derive(Serialize)]
struct PreviewResponse {
    result: String,
}

async fn preview_citation(Json(payload): Json<PreviewRequest>) -> Json<PreviewResponse> {
    // 1. Convert Vec<Reference> to Bibliography (IndexMap)
    let bib: Bibliography = payload.references
        .into_iter()
        .map(|r| (r.id().clone().unwrap_or_default(), r))
        .collect();

    // 2. Identify IDs to cite (for now just cite them all)
    let cite_ids: Vec<String> = bib.keys().cloned().collect();

    // 3. Initialize Processor
    let processor = Processor::new(payload.style, bib);

    // 4. Create Citation object
    let citation = Citation {
        id: Some("preview-1".to_string()),
        items: cite_ids.into_iter().map(|id| CitationItem { id, ..Default::default() }).collect(),
        ..Default::default()
    };

    // 5. Render
    let result = match processor.process_citation(&citation) {
        Ok(res) => res,
        Err(e) => format!("Error: {}", e),
    };

    Json(PreviewResponse { result })
}

async fn preview_bibliography(Json(payload): Json<PreviewRequest>) -> Json<PreviewResponse> {
    let bib: Bibliography = payload.references
        .into_iter()
        .map(|r| (r.id().clone().unwrap_or_default(), r))
        .collect();

    let processor = Processor::new(payload.style, bib);
    let output = processor.process_references();
    
    // Simple join of entries for now, typically this would be a list
    let result = match output.bibliography.is_empty() {
        true => String::new(),
        false => output.bibliography.iter()
            .map(|entry| csln_processor::citation_to_string(entry, None, None, None, None))
            .collect::<Vec<String>>()
            .join("\n")
    };

    Json(PreviewResponse { result })
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct StyleRow {
    id: Uuid,
    user_id: Uuid,
    title: String,
    intent: Value,
    csln: Option<String>,
    is_public: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct SaveStyleRequest {
    id: Option<Uuid>,
    title: String,
    intent: Value,
    csln: Option<String>,
    is_public: Option<bool>,
}

async fn list_styles(
    State(state): State<Arc<AppState>>,
    user: auth::AuthenticatedUser,
) -> Json<Vec<StyleRow>> {
    let styles = sqlx::query_as!(
        StyleRow,
        "SELECT id, user_id, title, intent, csln, is_public, created_at, updated_at FROM styles WHERE user_id = $1 ORDER BY updated_at DESC",
        user.id
    )
    .fetch_all(&state.db)
    .await
    .expect("Failed to fetch styles");

    Json(styles)
}

async fn list_public_styles(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<StyleRow>> {
    let styles = sqlx::query_as!(
        StyleRow,
        "SELECT id, user_id, title, intent, csln, is_public, created_at, updated_at FROM styles WHERE is_public = true ORDER BY updated_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .expect("Failed to fetch public styles");

    Json(styles)
}

async fn get_style(
    State(state): State<Arc<AppState>>,
    user: auth::AuthenticatedUser,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    let style = sqlx::query_as!(
        StyleRow,
        "SELECT id, user_id, title, intent, csln, is_public, created_at, updated_at FROM styles WHERE id = $1 AND (user_id = $2 OR is_public = true)",
        id,
        user.id
    )
    .fetch_optional(&state.db)
    .await
    .expect("Failed to fetch style");

    match style {
        Some(s) => Json(s).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "Style not found").into_response(),
    }
}

async fn save_style(
    State(state): State<Arc<AppState>>,
    user: auth::AuthenticatedUser,
    Json(payload): Json<SaveStyleRequest>,
) -> Json<StyleRow> {
    let style = if let Some(id) = payload.id {
        // Update
        sqlx::query_as!(
            StyleRow,
            r#"
            UPDATE styles 
            SET title = $1, intent = $2, csln = $3, is_public = $4
            WHERE id = $5 AND user_id = $6
            RETURNING id, user_id, title, intent, csln, is_public, created_at, updated_at
            "#,
            payload.title,
            payload.intent,
            payload.csln,
            payload.is_public.unwrap_or(false),
            id,
            user.id
        )
        .fetch_one(&state.db)
        .await
        .expect("Failed to update style")
    } else {
        // Create
        sqlx::query_as!(
            StyleRow,
            r#"
            INSERT INTO styles (user_id, title, intent, csln, is_public)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, title, intent, csln, is_public, created_at, updated_at
            "#,
            user.id,
            payload.title,
            payload.intent,
            payload.csln,
            payload.is_public.unwrap_or(false)
        )
        .fetch_one(&state.db)
        .await
        .expect("Failed to create style")
    };

    // Add to history
    let _ = sqlx::query!(
        "INSERT INTO history (style_id, intent_snapshot) VALUES ($1, $2)",
        style.id,
        style.intent
    )
    .execute(&state.db)
    .await;

    Json(style)
}

async fn fork_style(
    State(state): State<Arc<AppState>>,
    user: auth::AuthenticatedUser,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    // 1. Fetch the original style (must be public or owned by user)
    let original = sqlx::query!(
        "SELECT title, intent, csln FROM styles WHERE id = $1 AND (is_public = true OR user_id = $2)",
        id,
        user.id
    )
    .fetch_optional(&state.db)
    .await
    .expect("Database error");

    let original = match original {
        Some(s) => s,
        None => return (axum::http::StatusCode::NOT_FOUND, "Source style not found").into_response(),
    };

    // 2. Create the copy
    let forked_title = format!("{} (Fork)", original.title);
    let style = sqlx::query_as!(
        StyleRow,
        r#"
        INSERT INTO styles (user_id, title, intent, csln, is_public)
        VALUES ($1, $2, $3, $4, false)
        RETURNING id, user_id, title, intent, csln, is_public, created_at, updated_at
        "#,
        user.id,
        forked_title,
        original.intent,
        original.csln,
    )
    .fetch_one(&state.db)
    .await
    .expect("Failed to fork style");

    Json(style).into_response()
}

async fn add_bookmark(
    State(state): State<Arc<AppState>>,
    user: auth::AuthenticatedUser,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "INSERT INTO bookmarks (user_id, style_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        user.id,
        id
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => axum::http::StatusCode::CREATED.into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn remove_bookmark(
    State(state): State<Arc<AppState>>,
    user: auth::AuthenticatedUser,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> impl IntoResponse {
    let _ = sqlx::query!(
        "DELETE FROM bookmarks WHERE user_id = $1 AND style_id = $2",
        user.id,
        id
    )
    .execute(&state.db)
    .await;

    axum::http::StatusCode::NO_CONTENT
}

async fn list_bookmarks(
    State(state): State<Arc<AppState>>,
    user: auth::AuthenticatedUser,
) -> Json<Vec<StyleRow>> {
    let styles = sqlx::query_as!(
        StyleRow,
        r#"
        SELECT s.id, s.user_id, s.title, s.intent, s.csln, s.is_public, s.created_at, s.updated_at 
        FROM styles s
        JOIN bookmarks b ON s.id = b.style_id
        WHERE b.user_id = $1
        ORDER BY b.created_at DESC
        "#,
        user.id
    )
    .fetch_all(&state.db)
    .await
    .expect("Failed to fetch bookmarks");

    Json(styles)
}
