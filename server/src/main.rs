use actix_files::Files;
use actix_web::{App, HttpServer};
use std::{env, io, sync::Mutex};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let host = env::var("HOST").unwrap_or("0.0.0.0".into());
    let port = env::var("PORT")
        .unwrap_or("8000".into())
        .parse::<u16>()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Mutex::new(0))
            .service(Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
