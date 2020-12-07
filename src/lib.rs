pub mod csv;
pub mod db;
pub mod models;
pub mod schema;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate regex;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use std::error::Error;
pub trait Get {
    type Item;
    type Conn;
    fn get(&self, c: &Self::Conn) -> Result<Vec<Self::Item>, Box<dyn Error>>;
}
pub trait Browse {
    type Item;
    type Conn;
    fn browse(
        &self,
        max: i64,
        off: i64,
        sort: String,
        order: String,
        c: &Self::Conn,
    ) -> Result<Vec<Self::Item>, Box<dyn Error>>;
   
}
pub trait Count {
    type Item;
    type Conn;
    fn query_count(&self,c: &Self::Conn) -> Result<i64,Box<dyn Error>>;
}
