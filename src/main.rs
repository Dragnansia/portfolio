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
    page::{home::home, project::proj},
};
use rocket::{fs::FileServer, Build, Rocket};
use rocket_dyn_templates::Template;
use std::env::var;

#[launch]
async fn rocket() -> Rocket<Build> {
    let mongodb = new_connection(&var("DB_URL").unwrap_or_default(), "Portfolio").await;
    let database = mongodb.database("portfolio");

    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![home, proj])
        .register("/", catchers![err_404])
        .manage(database)
        .attach(Template::custom(|_engines| {}))
}
