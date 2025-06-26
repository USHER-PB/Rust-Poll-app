use actix_web::{App, HttpRequest, HttpServer};
use sqlx::Postgres
struct User{
  name: String, 
  password: String 
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {

  let connection = sqlx::postgres::Pgpool::connect("DATBASE URL")
  .connect();
  HttpServer::new(||{
    App::new()
    .route()
    .route( )
  })
  .bind("192.168.2.90:8000")?
  .run
  .await
}
mod models;