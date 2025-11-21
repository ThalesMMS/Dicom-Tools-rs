use axum::{
    extract::{Multipart, Path},
    routing::{get, post},
    Router, Json, response::Html,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub async fn start_server(host: &str, port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/metadata/:filename", get(get_metadata))
        .route("/api/upload", post(upload_handler))
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    println!("Server running at http://{}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root_handler() -> Html<&'static str> {
    // Relative path resolution: move from web.rs into the templates folder
    Html(include_str!("templates/index.html")) 
}

async fn upload_handler(mut multipart: Multipart) -> Json<Value> {
    // Minimal upload handler for demo purposes
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Received file: {} ({} bytes)", name, data.len());
        // Placeholder: here we would persist to /tmp and process
    }
    Json(json!({ "success": true }))
}

async fn get_metadata(Path(filename): Path<String>) -> Json<Value> {
    // Mock response; real implementation would invoke metadata extraction
    Json(json!({
        "patient": { "name": "DOE^JOHN", "id": "12345" },
        "filename": filename
    }))
}
