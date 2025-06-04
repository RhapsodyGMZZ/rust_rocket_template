use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn home() -> Template {
    return Template::render("home", context! {
        description:"This is Rust's Rocket template that will be maintained and updated often. Feel free to make issues, 
        give some ideas to enhance this little project.",
        username: "RhapsodyGMZZ"
    })
}