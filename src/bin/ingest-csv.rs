extern crate diesel;
extern crate serde;
#[macro_use]
extern crate clap;


use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

use clap::App;
use graphql_rs::csv::{process_derivations, process_foods, process_nutdata, process_nutrients};
use diesel::mysql::MysqlConnection;
use crate::diesel::Connection;
use std::process;

use graphql_rs::models::Food;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Bad url");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
//#[derive(Debug, Serialize, Deserialize)]
///
///  rudimentary implementation of query
pub fn main() {
    let conn = establish_connection();
    let cli = load_yaml!("clap.yml");
    let matches = App::from_yaml(cli).get_matches();
    let csvtype = matches.value_of("type").unwrap();
    let path = matches.value_of("path").unwrap();
    match csvtype {
        "FOOD" => {
            let mut count: usize = 0;
            println!("Loading foods");
            count = process_foods(path.to_string(), &conn);
            println!("Finished. {} foods loaded.", count);
            println!("Now loading nutrient data.");
            count = process_nutdata(path.to_string(), &conn);
            println!("Finished. {} nutrient data.", count)
        }
        "NUT" => {
            let count = process_nutrients(path.to_string(), &conn);
            println!("Finished.  {} nutrients loaded", count);
        }
        "DERV" => {
            let count = process_derivations(path.to_string(), &conn);
            println!("Finished.  {} derivations loaded", count);
        }
        _ => {
            println!("invalid input type");
            process::exit(1)
        }
    }

    process::exit(0)
}
