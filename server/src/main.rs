use axum::{
    routing::{get, post},
    Router,
    Json,
};
use std::net::SocketAddr;
use serde_json::{Value, json};
use csln_core::Style;
use csln_processor::{Processor, Reference, Bibliography, Citation, CitationItem};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/version", get(version))
        .route("/preview/citation", post(preview_citation))
        .route("/preview/bibliography", post(preview_bibliography));

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
        .map(|r| (r.id.clone(), r))
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
        .map(|r| (r.id.clone(), r))
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
