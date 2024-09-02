use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").unwrap());
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub mod schema;
