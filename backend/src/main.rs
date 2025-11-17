use backend::{cors, database, routes::home::home};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use sqlx::PgPool;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    dotenvy::from_path("../.env").expect("Can't load .env file.");

    // Initialising database object AND applying migrations
    let db: PgPool = database::open()
        .await
        .unwrap_or_else(|e| panic!("Couldn't open database: {e}"));
    database::init_db().unwrap_or_else(|e| panic!("Migration could not be performed: {e}"));

    // Building the app with the routes mounted
    rocket::build()
        .mount("/public", FileServer::from("./static"))
        .mount("/", routes![home])
        .manage(db) // General context, it can be called mutltiple times while the type is different for each method call
        .attach(cors::CORS) //setting the proper CORS headers as a `fairing` which is triggered at each response sent by the server to send also this CORS headers.
        .attach(Template::fairing())
}
