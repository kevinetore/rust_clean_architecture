use diesel::pg::PgConnection;
use std::env;
use diesel::r2d2::ConnectionManager;

pub fn establish_connection() -> ConnectionManager<PgConnection> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    ConnectionManager::<PgConnection>::new(database_url)
}