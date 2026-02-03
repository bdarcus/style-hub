use axum::{
    extract::State,
    routing::{get, post},
    Router,
    Json,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::{Value, json};
use csln_core::Style;
use csln_processor::{Processor, Reference, Bibliography, Citation, CitationItem};
use serde::{Deserialize, Serialize};
use intent_engine::{StyleIntent, DecisionPackage};

struct AppState {
    references: HashMap<String, Reference>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load references from YAML
    // Check multiple potential locations for the file
    let possible_paths = [
        "server/resources/comprehensive.yaml",
        "resources/comprehensive.yaml",
        "../resources/comprehensive.yaml",
    ];

    let mut ref_path = "server/resources/comprehensive.yaml";
    for path in possible_paths {
        if std::path::Path::new(path).exists() {
            ref_path = path;
            break;
        }
    }
    
    println!("Loading references from: {}", ref_path);

    let f = std::fs::File::open(ref_path).expect("Failed to open comprehensive.yaml references");
    let mut references: HashMap<String, Reference> = serde_yaml::from_reader(f).expect("Failed to parse comprehensive.yaml");
    
    // Ensure each reference has its ID set from the map key
    for (id, reference) in references.iter_mut() {
        reference.set_id(id.clone());
    }

    let state = Arc::new(AppState {
        references: references.clone()
    });
    
    println!("Loaded {} references.", references.len());

    let app = Router::new()
        .route("/", get(health_check))
        .route("/version", get(version))
        .route("/references", get(get_references))
        .route("/preview/citation", post(preview_citation))
        .route("/preview/bibliography", post(preview_bibliography))
        .route("/api/v1/decide", post(decide_handler))
        .route("/api/v1/generate", post(generate_handler))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
    println!("Handling preview_citation request");
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
        Err(e) => {
            println!("preview_citation error: {}", e);
            format!("Error: {}", e)
        },
    };

    Json(PreviewResponse { result })
}

async fn preview_bibliography(Json(payload): Json<PreviewRequest>) -> Json<PreviewResponse> {
    println!("Handling preview_bibliography request");
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

/// Handler for the `/api/v1/decide` endpoint.
/// 
/// Receives the current `StyleIntent` from the frontend and determines:
/// 1. What is missing?
/// 2. What is the next logical question to ask?
/// 3. What are the preview options for that question?
async fn decide_handler(
    State(state): State<Arc<AppState>>,
    Json(intent): Json<StyleIntent>
) -> Json<DecisionPackage> {
    println!("Handling decide request: {:?}", intent);
    // Call the engine to determine the next decision based on current intent
    let mut package = intent.decide();

    // Generate real preview using the processor
    let style = intent.to_style();
    println!("Generated style: {:?}", style);
    
    // Convert HashMap to Bibliography (IndexMap)
    let bib: Bibliography = state.references.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    // Pick a few diverse references to cite for the preview
    let mut cite_ids = Vec::new();
    
    // Prioritize specific references that show off style features
    let candidates = ["vaswani_attention", "foucault_discipline", "brown_v_board"];
    for id in candidates {
        if bib.contains_key(id) {
            cite_ids.push(id.to_string());
        }
    }
    
    // Fallback to random ones if none found
    if cite_ids.is_empty() {
        cite_ids = bib.keys().take(3).cloned().collect();
    }

    if !cite_ids.is_empty() {
        let processor = Processor::new(style, bib);
        let citation = Citation {
            id: Some("preview-1".to_string()),
            items: cite_ids.into_iter().map(|id| CitationItem { id, ..Default::default() }).collect(),
            ..Default::default()
        };

        println!("Processing citation for {} items", citation.items.len());

        match processor.process_citation(&citation) {
            Ok(res) => {
                println!("Preview result: {}", res);
                if !res.trim().is_empty() {
                    let mut html = format!("<div class='live-preview-content'><div class='preview-citation'>{}</div>", res);
                    
                    if intent.has_bibliography.unwrap_or(false) {
                         let bib_output = processor.process_references();
                         if !bib_output.bibliography.is_empty() {
                             html.push_str("<div class='preview-bibliography'><h4>Example Bibliography</h4>");
                             for entry in bib_output.bibliography {
                                 let bib_str = csln_processor::citation_to_string(&entry, None, None, None, None);
                                 html.push_str(&format!("<div class='bib-entry'>{}</div>", bib_str));
                             }
                             html.push_str("</div>");
                         }
                    }
                    
                    html.push_str("</div>");
                    package.preview_html = html;
                }
            },
            Err(e) => {
                println!("Preview generation error: {}", e);
                // Fallback to the hardcoded preview if generation fails (or show error)
            }
        }
    }

    Json(package)
}

/// Handler for the `/api/v1/generate` endpoint.
/// 
/// Receives the final `StyleIntent` and returns the complete CSLN YAML.
async fn generate_handler(Json(intent): Json<StyleIntent>) -> (axum::http::HeaderMap, String) {
    let csln = intent.generate_csln();
    
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/x-yaml"),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        axum::http::HeaderValue::from_static("attachment; filename=\"custom-style.yaml\""),
    );

    (headers, csln)
}