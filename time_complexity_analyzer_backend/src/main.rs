use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct CodeInput {
    code: String,
}

#[post("/analyze")]
async fn analyze_code(code_input: web::Json<CodeInput>) -> impl Responder {
    // For simplicity, let's assume we are analyzing a simple loop.
    let code = &code_input.code;
    let complexity = if code.contains("for") || code.contains("while") {
        "O(n)".to_string()
    } else {
        "O(1)".to_string()
    };
    
    HttpResponse::Ok().json(complexity)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(analyze_code)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
