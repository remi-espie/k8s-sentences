use actix_web::{web, App, HttpServer, Responder};

async fn aggregate() -> impl Responder {
    let nouns = reqwest::get("http://words:8081/noun")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let verbs = reqwest::get("http://words:8081/verb")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let hostname = hostname::get().unwrap().into_string().unwrap();
    format!("{} {} from {}", nouns, verbs, hostname)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(aggregate)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
