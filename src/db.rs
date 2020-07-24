extern crate diesel;
use dotenv::dotenv;
use std::env;

use crate::diesel::Connection;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub const MAX_RECS: i64 = 250;
pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn connect() -> MysqlPool {
    let database_url = env::var("DATABASE_URL").expect("Bad url");
    init(&database_url).expect("Error")
}

pub struct BrowseRequest {
    pub max: i64,
    pub offset: i64,
    pub sort: String,
    pub order: String,
}
impl BrowseRequest {
    pub fn new() -> Self {
        Self {
            max: 0,
            offset: 0,
            sort: String::from("id"),
            order: String::from("asc"),
        }
    }
}
