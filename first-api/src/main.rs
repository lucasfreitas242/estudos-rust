#![allow(unused)]

use std::net::SocketAddr;

use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello", 
        get(handler_hello));


    //region: ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap()

        //endregion ---Start Server
}

//region ---Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello, <strong>{name}!!!</strong>"))       
}

//endregion region ---Handler Hello