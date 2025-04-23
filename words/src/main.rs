use std::env;
use actix_web::{web, App, HttpServer, Responder};
use rand::prelude::IndexedRandom;
use tokio_postgres::{NoTls, Client};

async fn get_noun(client: web::Data<Client>) -> impl Responder {

    // Query the database for nouns
    let rows = client
        .query("SELECT noun FROM nouns", &[])
        .await
        .expect("Failed to execute query");

    // Collect the nouns into a vector
    let nouns: Vec<String> = rows
        .iter()
        .map(|row| row.get("noun"))
        .collect();
    
    let default = "default_noun".to_string();

    // Pick a random noun
    let noun = nouns.choose(&mut rand::rng()).unwrap_or(&default);

    noun.to_string()
}

async fn get_verb(client: web::Data<Client>) -> impl Responder {
    // Query the database for nouns
    let rows = client
        .query("SELECT verb FROM verbs", &[])
        .await
        .expect("Failed to execute query");

    // Collect the words into a vector
    let nouns: Vec<String> = rows
        .iter()
        .map(|row| row.get("verb"))
        .collect();

    let default = "default_verb".to_string();

    // Pick a random verb
    let verb = nouns.choose(&mut rand::rng()).unwrap_or(&default);

    verb.to_string()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load the database url from environment variables
    let database_url: String  = env::var("DATABASE_URL").unwrap_or("postgres://postgres:postgres@postgres:5432/words".to_string());
    
    // Connect to the PostgreSQL database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to the database");

    // Spawn the connection in a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let client_data = web::Data::new(client);


    HttpServer::new(move || 
        App::new()
            .app_data(client_data.clone())
            .route("/noun", web::get().to(get_noun))
            .route("/verb", web::get().to(get_verb))
    )
        .bind("0.0.0.0:8081")?
        .run()
        .await
}