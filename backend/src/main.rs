use std::thread;

use backend::{cors, database, routes::home::home, routes::websocket::ws_rs,};
use futures::executor::block_on;
use rocket::data::{Limits, ToByteUnit};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use sqlx::PgPool;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    dotenvy::from_path("../.env").expect("Can't load .env file.");

    // Custom config to set some settings for the server
    let figment = rocket::Config::figment().merge((
        "limits",
        Limits::new()
            .limit("data-form", 5.megabytes())
            .limit("form", 2.megabytes())
            .limit("string", 5.megabytes())
            .limit("bytes", 5.megabytes()),
    ));

    // Initialising database object AND applying migrations
    let db: PgPool = database::open()
        .await
        .unwrap_or_else(|e| panic!("Couldn't open database: {e}"));
    database::init_db().unwrap_or_else(|e| panic!("Migration could not be performed: {e}"));

    // Spawning a thread to start asynchronously the websocket server
    thread::Builder::new()
        .name("Thread for Rust Chat with ws crate".into())
        .spawn(move || {
            block_on(ws_rs::websocket());
        })
        .map_err(|e|dbg!(format!("ERROR {e}"))).expect("ERROR WEBSOCKET OUT");

    // Building the app with the routes mounted
    rocket::custom(figment)
        .mount("/public", FileServer::from("./static"))
        .mount("/", routes![home])
        .manage(db) // General context, it can be called mutltiple times while the type is different for each method call
        .attach(cors::CORS) //setting the proper CORS headers as a `fairing` which is triggered at each response sent by the server to send also this CORS headers.
        .attach(Template::fairing())
}
