# graphql-rs
A graphql server for the [USDA Branded Food Products](https://fdc.nal.usda.gov) dataset implemented with [Rust](https://www.rust-lang.org) using [actix](https://actix.rs), [juniper](https://docs.rs/juniper) and [diesel](https://diesel.rs).  The data store is [mariadb](mariadb.com).  This project is an excercise in learning a Rust, the learning curve for which has been fairly steep for me but more than worthwhile.  Please share your suggestions for improving my Rust as well as the query functionality.   

A running instance of the server is available at [rs.littlebunch.com](https://rs.littlebunch.com/).  A docker deployment is forthcoming.

Feel free to take this project as a starting point for writing your own graphql service.
## Building
### Step 1: Set-up your environment: 
If you haven't already, install the Rust [toolchain](https://www.rust-lang.org/tools/install) in your work environment as well as a recent version of [Mariadb](https://go.mariadb.com/download-mariadb-server-community.html?utm_source=google&utm_medium=ppc&utm_campaign=MKG-Search-Google-Branded-DL-NA-Server-DL&gclid=Cj0KCQjwvIT5BRCqARIsAAwwD-T-NRStQ4_3Ci8FyhdSYrsJWofpjOO5yKLxZ6NOGRqRHvdQxIAIjREaAtGWEALw_wcB)/[Mysql](https://www.mysql.com/downloads/)  
### Step 2: Clone this repo
```
git clone git@github.com:littlebunch/graphql-rs.git
```
### Step 3: Set-up the database
You can build the schema from the ground-up using the [Diesel CLI](https://diesel.rs) or save yourself some time and use the dump of a recent version of the Branded Food Products database available on [https://go.littlebunch.com](https://go.littlebunch.com/bfpd-2020-07-27.sql.gz) which you can download and create the database in your environment.
### Step 4: Start the service
You need to set a couple of environment variables.  It generally makes sense to put them in an .env file in the root path of your project which gets loaded start-up:

```
DATABASE_URL=mysql://user:userpassword@localhost/bfpd
GRAPHIQL_URL=http://localhost:8080/graphql
```
Then run the server from the project root (the path where cargo.toml is located):
```
cargo run
```
## Sample Queries
The nice thing about graphql is that it's self-documenting as illustrated by the client's "Documentation Explorer".  To get you started, here are some sample queries: 
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
#### Food UPC 000000018753 with nutrient data for Calcium (nutrient nbr = 301):
```
{
  food(fid:"000000018753", nids: ["301"]) {
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
#### Browse foods, ordered by food name:
```
{
  foods(max: 150, offset: 0, sort: "description", nids: []) {
    upc
    description
    manufacturer
    food
    ingredients
    foodGroup
  }
}
