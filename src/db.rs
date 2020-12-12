extern crate diesel;
use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn init(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn connect() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("Bad url");
    init(&database_url).expect("Error")
}
