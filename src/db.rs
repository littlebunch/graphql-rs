extern crate diesel;
use std::env;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn connect() -> MysqlPool {
    let database_url = env::var("DATABASE_URL").expect("Bad url");
    init(&database_url).expect("Error")
}
