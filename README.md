# graphql-rs
A graphql server for the [USDA Branded Food Products](https://fdc.nal.usda.gov) dataset implemented with [Rust](https://www.rust-lang.org) using [actix](https://actix.rs), [juniper](https://docs.rs/juniper) and [diesel](https://diesel.rs).  The data store is [mariadb](mariadb.com).  This project is an excercise in learning a Rust, the learning curve for which, for me, has been fairly steep but worthwhile.  Please share your suggestions for improving my Rust as well as the query functionality.   

Also, feel free to take this project as a starting point for writing your own graphql service.
## Building
### Step 1: Set-up your environment: 
If you haven't already, install the Rust [toolchain](https://www.rust-lang.org/tools/install) in your work environment as well as a recent version of [Mariadb](https://go.mariadb.com/download-mariadb-server-community.html?utm_source=google&utm_medium=ppc&utm_campaign=MKG-Search-Google-Branded-DL-NA-Server-DL&gclid=Cj0KCQjwvIT5BRCqARIsAAwwD-T-NRStQ4_3Ci8FyhdSYrsJWofpjOO5yKLxZ6NOGRqRHvdQxIAIjREaAtGWEALw_wcB)/[Mysql](https://www.mysql.com/downloads/)  
### Step 2: Clone this repo
```
git clone git@github.com:littlebunch/graphql-rs.git
```
### Step 3: Set-up the database
You can build the schema from the ground-up using the [Diesel CLI](https://diesel.rs) or save yourself some time and use the dump of a recent version of the Branded Food Products database available [here](https://go.littlebunch.com/bfpd/download) to create the database in your environment.
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
