use crate::{arguments::array_to_hashmap, html::render_html_file, page::project::Project};
use rocket::State;
use rocket_dyn_templates::Template;

#[get("/")]
pub fn home(_projects: &State<Vec<Project>>) -> Template {
    render_html_file("home", Some(array_to_hashmap(&[("title", "Home")])))
}
