use actix_web::{web, App, HttpServer, Responder};

async fn aggregate() -> impl Responder {
    let nouns = reqwest::get("http://nouns:8081")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let verbs = reqwest::get("http://verbs:8082")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let hostname = hostname::get().unwrap().into_string().unwrap();
    format!("{nouns} {verbs} from {hostname}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(aggregate)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
