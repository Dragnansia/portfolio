use actix_files::Files;
use actix_web::{App, HttpServer};
use mongodb::{error::Error, options::ClientOptions, Client};
use std::{env, io, sync::Mutex};

async fn db_connection() -> Result<Client, Error> {
    let db_url = env::var("DB_URL").unwrap_or_default();
    let options = ClientOptions::parse(db_url).await?;

    Client::with_options(options)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let host = env::var("HOST").unwrap_or("0.0.0.0".into());
    let port = env::var("PORT")
        .unwrap_or("8000".into())
        .parse::<u16>()
        .unwrap();

    let db = match db_connection().await {
        Ok(db) => db,
        Err(error) => {
            return Err(std::io::Error::new(
                io::ErrorKind::Unsupported,
                error.to_string(),
            ));
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(Mutex::new(0))
            .service(Files::new("/", "./dist/").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
