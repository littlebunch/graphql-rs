# graphql-rs
A graphql server for the [USDA Branded Food Products](https://fdc.nal.usda.gov) dataset implemented with [Rust](https://www.rust-lang.org) using [Actix](https://actix.rs), [Juniper](https://docs.rs/juniper) and [Diesel](https://diesel.rs).  The data store is [mariadb](mariadb.com). (A preliminary [postgresql](https://www.postgresql.org) version is also available on the pg branch.)  This project is an exercise in learning Rust. I used https://github.com/iwilsonq/rust-graphql-example and https://github.com/andrewleverette/rust_csv_examples as starting points for the server and csv processing respectively.  The Rust learning curve has been fairly steep for me but more than worthwhile.  Please share your suggestions for improving my Rust as well as the query functionality.   

A running instance of the server is available at [rs.littlebunch.com](https://rs.littlebunch.com/).  A docker image is available on [docker hub](https://hub.docker.com/repository/docker/littlebunch/graphql-rs).  A recent dump of the database is available at [https://go.littlebunch.com](https://go.littlebunch.com/bfpd-2020-07-27.sql.gz).

Feel free to take this project as a starting point for writing your own graphql service.
## What's here
[./src/cvs.rs](https://github.com/littlebunch/graphql-rs/blob/master/src/csv.rs) -- module used by the ingest utility for importing the UDSA csv files into the database.     
[./src/db.rs](https://github.com/littlebunch/graphql-rs/blob/master/src/db.rs) -- wrapper for connecting to the database; configured for Mysql/Mariadb     
[./src/graphql_schema.rs](https://github.com/littlebunch/graphql-rs/blob/master/src/graphql_schema.rs) -- graphql schema     
[./src/lib.rs](https://github.com/littlebunch/graphql-rs/blob/master/src/lib.rs) -- things to build a crate   
[./src/main.rs](https://github.com/littlebunch/graphql-rs/blob/master/src/main.rs) -- actix web server init and run      
[./src/models.rs](https://github.com/littlebunch/graphql-rs/blob/master/src/models.rs) -- all the stuff for accessing the database using Diesel ORM     
[./src/schema.rs](https://github.com/littlebunch/graphql-rs/blob/master/src/schema.rs) -- database schema derived from Diesel CLI and used by Diesel calls     
[./src/bin/ingest-csv.rs](https://github.com/littlebunch/graphql-rs/blob/master/src/bin/ingest-csv.rs) -- cli utility for importing the USDA csv files into the database    
[./database/](https://github.com/littlebunch/graphql-rs/tree/master/database) -- Diesel migration scripts for mariadb and postgres to create the database and schema.rs

## How to Build
### Step 1: Set-up your environment: 
If you haven't already, install the Rust [toolchain](https://www.rust-lang.org/tools/install) in your work environment as well as a recent version of [Mariadb](https://go.mariadb.com/download-mariadb-server-community.html?utm_source=google&utm_medium=ppc&utm_campaign=MKG-Search-Google-Branded-DL-NA-Server-DL&gclid=Cj0KCQjwvIT5BRCqARIsAAwwD-T-NRStQ4_3Ci8FyhdSYrsJWofpjOO5yKLxZ6NOGRqRHvdQxIAIjREaAtGWEALw_wcB)/[Mysql](https://www.mysql.com/downloads/)  
### Step 2: Clone this repo
```
git clone git@github.com:littlebunch/graphql-rs.git
```
### Step 3: Build the binaries
```
cargo build --release
```
This will create the graphql-rs server in the ./target/release directory.  If you are importing USDA csv, then build the cli utility for doing that:

```
cargo build --release --bin ingest-csv
```

## How to run
### Step 1: Set-up the database
A couple of options:  1) You can build the database from the ground-up by importing the USDA csv files using the provided ingest-csv command line utility or 2) download a dump of a recent version of the Branded Food Products database from [https://go.littlebunch.com](https://go.littlebunch.com/bfpd-2020-07-27.sql.gz) and create the database in your environment.    

#### How to use the ingest-csv utility 
This assumes you have access to a working instance of mariadb or mysql.  The utility is a first draft and assumes you are importing into an empty database.   

1. Download and unzip the latest csv from the [FDC website](https://fdc.nal.usda.gov/download-datasets.html) into a directory of your choice.  You will need the Branded Foods and Supporting data for All Downloads zip files:
```
wget https://fdc.nal.usda.gov/fdc-datasets/FoodData_Central_branded_food_csv_2020-04-29.zip
```
```
wget https://fdc.nal.usda.gov/fdc-datasets/FoodData_Central_Supporting_Data_csv_2020-04-29.zip
```

2. Create an empty schema using the schema provided in database/bfpd-schema.sql. 
```
mysql -u user -p -e"create schema bfpd;"
```

3. Use the Diesel migration script to create an empty database.
```
mysql -u user -p bfpd < database/mariadb/up.sql
```
Note: You can use the up.sql and down.sql scripts to create a [diesel migration](https://diesel.rs/guides/getting-started/).  This is probably more trouble than it's worth unless you need to change the schema or just want to learn a bit more about diesel migrations.

4. Load the data by pointing the program to the full path containing the csv:
```
./target/release/ingest-cvs -p /path/to/csv/
```
The load takes about 3-10 minutes depending on your hardware.  Note:  you need to set a DATABASE_URL variable as described in Step 2 below before running the ingest-csv program.

### Step 2: Start the service
You need to set a couple of environment variables.  It generally makes sense to put them in an .env file in the root path of your project which gets loaded at start-up:

```
DATABASE_URL=mysql://user:userpassword@localhost/bfpd
GRAPHIQL_URL=http://localhost:8080/graphql
```
Then run the server from the project root (the path where cargo.toml is located):
```
./target/release/graphql-rs
```
or start a Docker instance:
```
docker run --rm -it -p 8080:8080 --env-file=/full/path/to/.env littlebunch/graphql-rs
```
The client will be available at  http://localhost:8080/graphiql.
## Sample Queries
To get you started, here are some sample queries you can paste into the client of your choice, e.g. Insomnia, Postman or the local graphiql playground.  Use either http://localhost:8080/graphql or https://rs.littlebunch.com/graphql.

#### Food UPC 000000018753 with all nutrient data:
```
{
  food(fid:"000000018753", nids: []) {
    upc
    description
    servingSize
    servingDescription
    servingUnit
    nutrientData {
      value
      portionValue
      nutrientNo
      nutrient
      unit
    }
  }
}
```
#### Food UPC 000000018753 with nutrient data for Energy (Calories) (nutrient nbr = 208):
```
{
  food(fid:"000000018753", nids: ["208"]) {
    upc
    description
    servingSize
    servingDescription
    servingUnit
    nutrientData {
      value
      portionValue
      nutrientNo
      nutrient
      unit
    }
  }
}
```
#### Browse foods, sorted ascending by upc filtered on publication date of 2020-02-01 and food group "Biscuits/Cookies":
```
{
  foods(browse: {max: 150, offset: 0, sort: "upc", order: "asc", filters: {pubdate: "20200201", fg: "Biscuits/Cookies", manu: ""}}, nids: []) {
    upc
    description
    manufacturer
    food
    ingredients
    foodGroup
    nutrientData {
      portionValue
      nutrientNo
      nutrient
      unit
    }
  }
}
```
#### Browse foods, sorted ascending by upc filtered on publication date range from 2020-02-01 through 2020-05-31 and manufacturer "GENERAL MILLS SALES INC.":
```
{
  foods(browse: {max: 150, offset: 0, sort: "description", order: "asc", filters: {pubdate: "20200201:20200531", fg: "", manu: "GENERAL MILLS SALES INC."}}, nids: []) {
    upc
    description
    manufacturer
    food
    ingredients
    foodGroup
    nutrientData {
      portionValue
      nutrientNo
      nutrient
      unit
    }
  }
}
```
#### List nutrients sorted ascending by name:
```
{
  nutrients(max: 100, offset: 0, sort: "name", order: "asc", nids: []) {
    nbr
    name
    unit
  }
}
```
### List food groups sorted ascending by group:
```
{
  foodGroups(max:125,offset:0,sort:"group",order:"asc") {
    id
    group
  }
}
```
### List food manufacturers (owners) sorted ascending by name:
```
{
  foodGroups(max:150,offset:0,sort:"name",order:"asc") {
    id
    name
  }
}
```
