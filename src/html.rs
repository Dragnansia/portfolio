use rocket_dyn_templates::Template;
use serde::Serialize;

pub fn render_html_file<C>(html_file: &'static str, args: Option<C>) -> Template
where
    C: Serialize,
{
    match args {
        Some(args) => Template::render(html_file, args),
        None => Template::render(html_file, &[("Error", "No arguments for render")]),
    }
}
