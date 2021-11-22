mod arguments;
mod database;
mod html;
mod http_error;
mod lang;
mod page;

#[macro_use]
extern crate rocket;

use crate::{
    database::new_connection,
    html::render_html_file,
    http_error::err_404,
    page::{
        home::home,
        project::{init_database_project, proj},
    },
};
use rocket::{fs::FileServer, Build, Rocket};
use rocket_dyn_templates::Template;
use std::env::var;

#[launch]
async fn rocket() -> Rocket<Build> {
    let mongodb = new_connection(&var("DB_URL").unwrap_or_default(), "Portfolio").await;
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
