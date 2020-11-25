use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

// Errorを返すenumを定義
#[derive(Error, Debug)]
enum MyError {}

// ResponseErrorをMyErrorに実装
impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello, Rust!";

    // HttpResponse::Ok()を返す
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    // 鯖建て
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}