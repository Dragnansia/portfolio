use crate::{arguments::array_to_hashmap, render_html_file};
use rocket_dyn_templates::Template;

#[catch(404)]
pub fn err_404() -> Template {
    render_html_file("404", Some(array_to_hashmap(&[("title", "404")])))
}
