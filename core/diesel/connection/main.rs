use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use std::env;

pub fn establish_connection() -> ConnectionManager<PgConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    ConnectionManager::<PgConnection>::new(database_url)
}
