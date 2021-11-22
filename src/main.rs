mod arguments;
mod database;
mod html;
mod http_error;
mod lang;
mod project;

#[macro_use]
extern crate rocket;

use crate::project::Project;
use crate::{
    arguments::array_to_hashmap, database::new_connection, html::render_html_file,
    http_error::err_404, project::init_database_project,
};
use mongodb::bson::doc;
use rocket::{fs::FileServer, response::Redirect, Build, Rocket, State};
use rocket_dyn_templates::Template;
use std::env::var;

#[get("/")]
fn home(_projects: &State<Vec<Project>>) -> Template {
    render_html_file("home", Some(array_to_hashmap(&[("title", "Home")])))
}

#[get("/project/<name>")]
fn proj(projects: &State<Vec<Project>>, name: &str) -> Result<Template, Redirect> {
    // Todo: Render page 404 or a new page for project not found ?
    // check for the better options but actually just redirect to 404 page
    let res = projects.iter().rfind(|p| p.title == name);
    if res.is_none() {
        Err(Redirect::to("/404"))
    } else {
        let proj = res.unwrap().clone();
        Ok(render_html_file("proj", Some(proj)))
    }
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let mongodb = new_connection(&var("DB_URL").unwrap(), "Portfolio").await;
    let database = mongodb.database("portfolio");

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
