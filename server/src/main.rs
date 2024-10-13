use std::net::SocketAddr;

static WASM_BG: &[u8] = include_bytes!("../static/wasm_bg.wasm");

use axum::{body::{Body, Bytes}, http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router};

async fn serve_file(path: &str) -> impl IntoResponse {
    match std::fs::read_to_string(path) {
        Ok(content) => (StatusCode::OK, Html(content)),
        Err(_) => (StatusCode::NOT_FOUND, Html("404 Not Found".to_string())),
    }
}

async fn serve_file_ad(path: &str, mime_type: &str) -> Response<Body> {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let body: Body = Body::new(content);
            let mut response = Response::new(body);

            response.headers_mut().insert(
                "Content-Type", mime_type.parse().unwrap()
            );

            response
        },
        Err(_) => {
            let body = Body::new("404 not found".to_string());
            let mut response = Response::new(body);
            *response.status_mut() = StatusCode::NOT_FOUND;

            response
        },
    }
}

async fn index() -> impl IntoResponse {
    serve_file("index.html").await
}

async fn wasm_main() -> Response<Body> {
    serve_file_ad("./static/wasm.js", "application/javascript").await
}

async fn wasm_bg() -> Response<Body> {
    let body: Body = Bytes::from(WASM_BG).into();
    let mut response = Response::new(body);

    response.headers_mut().insert(
        "Content-Type", "application/wasm".parse().unwrap()
    );

    response
}

#[tokio::main]
async fn main() {
   let app = Router::new()
       .route("/", get(index))
       .route("/static/wasm_bg.wasm", get(wasm_bg))
       .route("/static/wasm.js", get(wasm_main));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
