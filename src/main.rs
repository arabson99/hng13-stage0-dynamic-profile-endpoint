use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use chrono::{Utc, SecondsFormat};
use reqwest::Client;

#[derive(Serialize)]
struct User {
    email: String,
    name: String,
    stack: String,
}

#[derive(Serialize)]
struct ApiResponse {
    status: String,
    user: User,
    timestamp: String,
    fact: String,
}

#[derive(serde::Deserialize)]
struct CatFactResponse {
    fact: String,
}

#[get("/me")]
async fn get_profile() -> impl Responder {
    let client = Client::new();
    let cat_fact_url = "https://catfact.ninja/fact";

    let fact = match client.get(cat_fact_url).send().await {
        Ok(resp) => match resp.json::<CatFactResponse>().await {
            Ok(json) => json.fact,
            Err(_) => String::from("Could not parse cat fact."),
        },
        Err(_) => String::from("Could not fetch cat fact at this time."),
    };

    let response = ApiResponse {
        status: "success".to_string(),
        user: User {
            email: "arabiusman99@gmail.com".to_string(),
            name: "Abubakar Abdulazeez Usman".to_string(),
            stack: "Rust/Actix Web".to_string(),
        },
        timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true), // ISO 8601 UTC format
        fact,
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running ");

    HttpServer::new(|| App::new().service(get_profile))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
