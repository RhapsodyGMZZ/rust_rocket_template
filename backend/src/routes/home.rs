use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn home() -> Template {
    return Template::render("home", context! {
        test:"Testing context"
    })
}