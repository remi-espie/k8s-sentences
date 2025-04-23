use actix_web::{web, App, HttpServer, Responder};
use rand::seq::IndexedRandom;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Nouns {
    nouns: Vec<String>,
}

async fn get_noun() -> impl Responder {
    let data = fs::read_to_string("nouns.json").expect("Unable to read file");
    let nouns: Nouns = serde_json::from_str(&data).expect("Unable to parse JSON");
    let noun = nouns.nouns.choose(&mut rand::rng()).unwrap();
    noun.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(get_noun)))
        .bind("0.0.0.0:8081")?
        .run()
        .await
}
