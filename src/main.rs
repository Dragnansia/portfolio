mod database;
mod html;
mod http_error;
mod lang;
mod project;

#[macro_use]
extern crate rocket;

use crate::lang::Language;
use crate::project::Image;
use crate::{
    database::new_connection, html::render_html_file, http_error::err_404, project::Project,
};
use mongodb::{bson::doc, Database};
use rocket::futures::{StreamExt, TryFutureExt, TryStreamExt};
use rocket::{fs::FileServer, Build, Rocket, State};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::env::var;

#[get("/")]
fn home(state: &State<Database>) -> Template {
    render_html_file("home", Some(&[("title", "Home")]))
}

async fn add_project_example(database: &Database) {
    let projects_list = database.collection::<Project>("Projects");
    projects_list
        .insert_one(
            Project {
                name: "MrBombe".to_string(),
                desc: "MrBombe Description".to_string(),
                images: [Image {
                    desc: "sdjfsdf".to_string(),
                    url: "qsdqsd".to_string(),
                }]
                .to_vec(),
                languages: [Language {
                    data: HashMap::new(),
                    name: "fr".to_string(),
                }]
                .to_vec(),
            },
            None,
        )
        .await;
}

// TODO: get project and languages and need to remove typing
async fn init_database_project(database: &Database) -> Option<Vec<Project>> {
    let collections = database.collection::<Project>("Projects");
    let mut lists = collections.find(None, None).await.ok()?;
    let projects = lists.try_collect().await.ok()?;

    for project in projects {
        println!("-> {:?}", project);
    }

    Some(projects)
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
