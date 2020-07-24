pub mod db;
pub mod models;
pub mod schema;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use self::diesel::prelude::*;
use std::error::Error;
