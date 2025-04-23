use actix_web::{web, App, HttpServer, Responder};
use rand::prelude::IndexedRandom;
use std::env;
use tokio_postgres::{Client, NoTls};

async fn get_noun(client: web::Data<Client>) -> impl Responder {
    // Query the database for nouns
    let rows = client
        .query("SELECT noun FROM nouns", &[])
        .await
        .expect("Failed to execute query");

    // Collect the nouns into a vector
    let nouns: Vec<String> = rows.iter().map(|row| row.get("noun")).collect();

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
    let nouns: Vec<String> = rows.iter().map(|row| row.get("verb")).collect();

    let default = "default_verb".to_string();

    // Pick a random verb
    let verb = nouns.choose(&mut rand::rng()).unwrap_or(&default);

    verb.to_string()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables for database connection
    let user = env::var("PG_USER").unwrap_or("postgres".to_string());
    let passwd = env::var("PG_PASSWORD").unwrap_or("password".to_string());
    let host = env::var("PG_HOST").unwrap_or("postgres".to_string());
    let port = env::var("PG_PORT").unwrap_or("5432".to_string());
    let dbname = env::var("PG_NAME").unwrap_or("words".to_string());
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        host, port, user, passwd, dbname
    );

    // Connect to the PostgreSQL database
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .expect("Failed to connect to the database");

    // Spawn the connection in a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let client_data = web::Data::new(client);

    HttpServer::new(move || {
        App::new()
            .app_data(client_data.clone())
            .route("/noun", web::get().to(get_noun))
            .route("/verb", web::get().to(get_verb))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
