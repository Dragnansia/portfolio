mod database;
mod html;
mod http_error;
mod lang;
mod project;

#[macro_use]
extern crate rocket;

use crate::project::Project;
use crate::{
    database::new_connection, html::render_html_file, http_error::err_404,
    project::init_database_project,
};
use mongodb::bson::doc;
use rocket::{fs::FileServer, response::Redirect, Build, Rocket, State};
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::collections::HashMap;
use std::env::var;

// Todo: find a better way to convert array to Hashmap
// Todo: move this function on a generic function file
fn array_to_hashmap<'a>(args: &'a [(&str, &str)]) -> HashMap<&'a str, &'a str> {
    let mut hmp = HashMap::new();
    hmp.extend(args.iter().map(|a| a.clone()));
    hmp
}

#[get("/")]
fn home(_projects: &State<Vec<Project>>) -> Template {
    render_html_file("home", Some(array_to_hashmap(&[("title", "Home")])))
}

#[get("/project/<name>")]
fn proj(projects: &State<Vec<Project>>, name: &str) -> Template {
    // Todo: Render page 404 or a new page for project not found ?
    // check for the better options but actually just redirect to 404 page
    println!("-> {}", name);
    let res = projects.iter().find(|p| p.name == name);
    if res.is_none() {
        Redirect::to("404");
    }

    render_html_file("proj", Some(res.unwrap()))
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let mongodb = new_connection(&var("DB_URL").unwrap(), "Portfolio").await;
    let database = mongodb.database("projects");

    let projects = init_database_project(&database).await.unwrap_or_default();
    assert!(!projects.is_empty(), "No project found");

    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![home, proj])
        .register("/", catchers![err_404])
        .manage(database)
        .manage(projects)
        .attach(Template::custom(|_engines| {}))
}
