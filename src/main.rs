mod database;
mod html;
mod http_error;

#[macro_use]
extern crate rocket;

use crate::{database::new_connection, html::render_html_file, http_error::err_404};
use mongodb::bson::doc;
use mongodb::Client;
use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::Template;
use std::env::var;

struct Database(Client);

#[get("/")]
fn home(state: &State<Database>) -> Template {
    render_html_file("home", Some(&[("title", "Home")]))
}

#[launch]
async fn rocket() -> _ {
    let mongodb = new_connection(&var("DB_URL").unwrap(), "Portfolio").await;

    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![home])
        .register("/", catchers![err_404])
        .manage(Database(mongodb))
        .attach(Template::custom(|_engines| {}))
}
