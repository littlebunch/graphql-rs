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
    let mut csvtype = matches.value_of("type").unwrap_or_default();
    let path = matches.value_of("path").unwrap();
    if csvtype.len() == 0 {
        csvtype="ALL"
    }
    let mut count: usize=0;
    match csvtype {
        "FOOD" => {
            println!("Loading foods");
            count = process_foods(path.to_string(), &conn);
            println!("Finished. {} foods loaded.", count);
            println!("Now loading nutrient data.");
            count = process_nutdata(path.to_string(), &conn);
            println!("Finished. {} nutrient data.", count)
        }
        "NUT" => {
            count = process_nutrients(path.to_string(), &conn);
            println!("Finished.  {} nutrients loaded", count);
        }
        "DERV" => {
            count = process_derivations(path.to_string(), &conn);
            println!("Finished.  {} derivations loaded", count);
        }
        "ALL" => {
            println!("Starting csv load");
            count = process_nutrients(path.to_string(), &conn);
            println!("Finished.  {} nutrients loaded", count);
            count = process_derivations(path.to_string(), &conn);
            println!("Finished.  {} derivations loaded", count);
            println!("Loading foods");
            count = process_foods(path.to_string(), &conn);
            println!("Finished. {} foods loaded.", count);
            println!("Now loading nutrient data.");
            count = process_nutdata(path.to_string(), &conn);
            println!("Finished. {} nutrient data.", count)
        }
        _ => {
            println!("invalid input type");
            process::exit(1)
        }
    }

    process::exit(0)
}
