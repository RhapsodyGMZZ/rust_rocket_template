use std::{env::var, path::Path};

use futures::executor;

use rocket::futures;
use sqlx::{
    Pool, Postgres,
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
};

/// Applies migrations of our database
///
/// # Returns:
/// - `Result<(), String>` used then to log the error and panic with the given error String
pub fn init_db() -> Result<(), String> {
    let mig: Result<Migrator, sqlx::migrate::MigrateError> =
        executor::block_on(Migrator::new(Path::new("./migrations")));
    let pool: Result<Pool<Postgres>, sqlx::Error> = executor::block_on(open());

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
/// - `Result<Pool<Postgres>, sqlx::Error>` (the same as PostgresPool which is an alias for this type)
/// if it throws an Error, it panics and doesn't launch the whole app.
pub async fn open() -> Result<Pool<Postgres>, sqlx::Error> {
    // Securely using Postgres environment variable to obfuscate the credentials of our database
    let conn_cfg = PgConnectOptions::new()
        .host("localhost")
        .port(
            var("POSTGRES_PORT")
                .ok()
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(5432),
        )
        .username(&var("POSTGRES_USER").unwrap_or_default())
        .password(&var("POSTGRES_PASSWORD").unwrap_or_default())
        .database(&var("POSTGRES_DATABASE").unwrap_or_default());
    PgPoolOptions::new()
        .max_connections(10) // There is a max of 10 users (Postgres users, not Framehub's users) simultaneously connected
        .connect_with(conn_cfg)
        .await
}
