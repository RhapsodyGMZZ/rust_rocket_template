use std::{env::var, path::Path};

use futures::executor;

use rocket::futures;
use sqlx::{
    migrate::Migrator,
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySql, Pool,
};

/// Applies migrations of our database
/// 
/// # Returns:
/// - `Result<(), String>` used then to log the error and panic with the given error String
pub fn init_db() -> Result<(), String> {
    let mig = executor::block_on(Migrator::new(Path::new("./migrations")));
    let pool = executor::block_on(open());

    match (mig, pool) {
        (Ok(m), Ok(p)) => match executor::block_on(m.run(&p)) {
            Err(e) => Err(format!("Migration failed: {e}")),
            Ok(()) => Ok(()),
        },
        (Err(e), _) => Err(format!("Error reading the migrations: {e}")),
        (_, Err(e)) => Err(format!("Database connection failed: {e}")),
    }
}

/// Opening database and returning its connector
/// 
/// # Returns:
/// - `Result<Pool<MySql>, sqlx::Error>` (the same as MySqlPool which is an alias for this type)
/// if it throws an Error, it panics and doesn't launch the whole app.
pub async fn open() -> Result<Pool<MySql>, sqlx::Error> {
    
    // Securely using MYSQL environment variable to obfuscate the credentials of our database
    let conn_cfg = MySqlConnectOptions::new()
        .host("localhost")
        .port(3306)
        .username(&var("MYSQL_USER").unwrap_or_default())
        .password(&var("MYSQL_PASSWORD").unwrap_or_default())
        .database(&var("MYSQL_DATABASE").unwrap_or_default());

    MySqlPoolOptions::new()
        .max_connections(10) // There is a max of 10 users (MYSQL users, not Framehub's users) simultaneously connected
        .connect_with(conn_cfg)
        .await
}