use actix_web::{web, App, HttpServer, Responder};
use rand::seq::IndexedRandom;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Verbs {
    verbs: Vec<String>,
}

async fn get_verbs() -> impl Responder {
    let data = fs::read_to_string("verbs.json").expect("Unable to read file");
    let verbs: Verbs = serde_json::from_str(&data).expect("Unable to parse JSON");
    let verb = verbs.verbs.choose(&mut rand::rng()).unwrap();
    verb.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(get_verbs)))
        .bind("0.0.0.0:8082")?
        .run()
        .await
}
