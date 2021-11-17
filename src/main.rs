mod html;
mod http_error;

#[macro_use]
extern crate rocket;

use crate::{html::render_html_file, http_error::err_404};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[get("/")]
fn home() -> Template {
    render_html_file("Home", "home", None)
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![home])
        .register("/", catchers![err_404])
        .attach(Template::custom(|_engines| {}))
}
