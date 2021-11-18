use rocket_dyn_templates::Template;
use std::collections::HashMap;

pub fn render_html_file(html_file: &'static str, args: Option<&[(&str, &str)]>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    if !args.is_none() {
        context.extend(args.unwrap().iter().map(|arg| arg.clone()));
    }

    for core in &context {
        println!("{:?}", core);
    }

    Template::render(html_file, context)
}
