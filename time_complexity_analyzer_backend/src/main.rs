use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use regex::Regex;

#[derive(Deserialize)]
struct CodeInput {
    code: String,
}

fn detect_language(code: &str) -> &str {
    if code.contains("fn ") && code.contains("let ") {
        "rust"
    } else if code.contains("def ") && code.contains("import ") {
        "python"
    } else if code.contains("function ") || code.contains("let ") || code.contains("const ") {
        "javascript"
    } else {
        "unknown"
    }
}

#[post("/analyze")]
async fn analyze_code(code_input: web::Json<CodeInput>) -> impl Responder {
    let code = &code_input.code;
    println!("Received code: {}", code); // Debug print

    let language = detect_language(code);
    println!("Detected language: {}", language); // Debug print

    let mut complexity = String::from("O(1)");

    let (loop_regex, nested_loop_regex, recursive_regex) = match language {
        "rust" => (
            Regex::new(r"\b(for|while)\b").expect("Failed to compile loop regex"),
            Regex::new(r"\b(for|while)[\s\S]*?\b(for|while)\b").expect("Failed to compile nested loop regex"),
            Regex::new(r"\bfn\b[\s\S]*?\b\1\b").expect("Failed to compile recursive regex"),
        ),
        "python" => (
            Regex::new(r"\b(for|while)\b").expect("Failed to compile loop regex"),
            Regex::new(r"\b(for|while)[\s\S]*?\b(for|while)\b").expect("Failed to compile nested loop regex"),
            Regex::new(r"\bdef\b[\s\S]*?\b\1\b").expect("Failed to compile recursive regex"),
        ),
        "javascript" => (
            Regex::new(r"\b(for|while)\b").expect("Failed to compile loop regex"),
            Regex::new(r"\b(for|while)[\s\S]*?\b(for|while)\b").expect("Failed to compile nested loop regex"),
            Regex::new(r"\bfunction\b[\s\S]*?\bfunction\b|\bconst\b[\s\S]*?=\s*function\b").expect("Failed to compile recursive regex"),
        ),
        _ => (
            Regex::new("").expect("Failed to compile loop regex"),
            Regex::new("").expect("Failed to compile nested loop regex"),
            Regex::new("").expect("Failed to compile recursive regex"),
        ),
    };

    if nested_loop_regex.is_match(code) {
        complexity = String::from("O(n^2)");
    } else if loop_regex.is_match(code) {
        complexity = String::from("O(n)");
    } else if recursive_regex.is_match(code) {
        complexity = String::from("O(2^n)");
    }

    println!("Determined complexity: {}", complexity); // Debug print

    HttpResponse::Ok().json(complexity)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8000"); // Log server start
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(analyze_code)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
