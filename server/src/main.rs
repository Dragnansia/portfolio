pub mod api;

use actix_files::Files;
use actix_web::{middleware, web::Data, App, HttpServer};
use api::project::{all_project, project_info};
use mongodb::{error::Error, options::ClientOptions, Client, Database};
use std::{env, io};

/// Connect to mongodb database
///
/// Need to set `DB_URL` env variable to try connection
async fn db_connection() -> Result<Client, Error> {
    let db_url = env::var("DB_URL").unwrap_or("mongodb://localhost:27017".into());
    let options = ClientOptions::parse(db_url).await?;

    Client::with_options(options)
}

pub struct AppState {
    db: Database,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let host = env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port = env::var("PORT")
        .unwrap_or_default()
        .parse::<u16>()
        .unwrap_or(8000);

    let db = match db_connection().await {
        Ok(db) => db.database("portfolio"),
        Err(error) => {
            return Err(std::io::Error::new(io::ErrorKind::Other, error.to_string()));
        }
    };

    println!("Server start at port: {}", port);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(AppState { db: db.clone() }))
            .service(all_project)
            .service(project_info)
            .service(Files::new("/", "./client/dist/").index_file("index.html"))
            .service(Files::new("/", "./client/dist/").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
