mod database;
mod html;
mod http_error;
mod lang;
mod project;

#[macro_use]
extern crate rocket;

use crate::{
    database::new_connection, html::render_html_file, http_error::err_404,
    project::init_database_project,
};
use mongodb::{bson::doc, Database};
use rocket::{fs::FileServer, Build, Rocket, State};
use rocket_dyn_templates::Template;
use std::env::var;

#[get("/")]
fn home(state: &State<Database>) -> Template {
    render_html_file("home", Some(&[("title", "Home")]))
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let mongodb = new_connection(&var("DB_URL").unwrap(), "Portfolio").await;
    let database = mongodb.database("projects");

    let projects = init_database_project(&database).await.unwrap_or_default();
    assert!(!projects.is_empty(), "No project found");

    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![home])
        .register("/", catchers![err_404])
        .manage(database)
        .attach(Template::custom(|_engines| {}))
}
