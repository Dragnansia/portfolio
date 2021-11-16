#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::collections::BTreeMap;

#[get("/")]
fn home() -> Template {
    let mut context = BTreeMap::from([("title", "Home")]);
    Template::render("home", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![home])
        .attach(Template::custom(|_engines| {}))
}
