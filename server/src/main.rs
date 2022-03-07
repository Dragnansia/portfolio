use actix_files::Files;
use actix_web::{App, HttpServer};
use std::{io, sync::Mutex};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Mutex::new(0))
            .service(Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
