use actix_web::{HttpResponse, get, web, App, HttpServer, Responder,Result};
use serde::Deserialize;
use sled;

#[derive(Deserialize)]
struct Url {
    url: String,
}

fn itostr(vec: sled::IVec) -> String {
    match String::from_utf8(vec.as_ref().to_vec()) {
        Ok(x) => x,
        Err(_) => String::from("")
    }
}

async fn set(db: web::Data<sled::Db>, form: web::Form<Url>) -> std::result::Result<impl Responder,Box<dyn std::error::Error>> {
    db.insert("last",form.url.as_bytes())?;
    Ok(HttpResponse::Ok().finish())
}

async fn get(db: web::Data<sled::Db>) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let reply = match db.get("last")?{
        Some(x) => x,
        None => sled::IVec::from("")
    };
    Ok(itostr(reply))
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let db: sled::Db = sled::open("database.sled").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/set", web::post().to(set))
            .route("/get", web::get().to(get))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
