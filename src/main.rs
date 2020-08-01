#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate juniper;
extern crate serde_derive;
use actix_web::{http, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use dotenv::dotenv;
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::env;
use std::io;
use std::sync::Arc;

mod db;
mod graphql_schema;
mod schema;
use crate::db::connect;
use crate::graphql_schema::{create_schema, Context, Schema};

fn graphiql() -> HttpResponse {
    let url = match env::var("GRAPHIQL_URL") {
        Ok(x) => x,
        Err(e) => "http://localhost:8080/graphql".to_string(),
    };
    //let html = graphiql_source("http://localhost:8080/graphql");
    let html = graphiql_source(&url);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn main() -> io::Result<()> {
    dotenv().ok();
    let pool = connect();
    let schema_context = Context { db: pool.clone() };
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(schema_context.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("0.0.0.0:8080")?
    .run()
}
