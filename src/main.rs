use actix_web::{App, HttpServer, web};
use sqlx::{FromRow, PgPool};
use serde::{Serialize, Deserialize};
use actix_cors::Cors;

// Import all poll-related functions from your models::poll module
use models::poll::{
    create_poll, 
    get_poll, 
    update_poll, 
    delete_poll, 
    vote, 
    get_stats, 
    get_expiring_polls,
    get_all_polls
};
use models::login::login;
use models::register::register;

mod models;
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

// Added #[derive(Clone)] to make Db clonable for actix-web's web::Data
#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Set your actual database URL here
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file or environment");
    
    // Connect to the database pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    // Create an instance of your Db struct
    let db_instance = Db { pool: pool };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            // Pass the Db struct as application data.
            // .clone() is required here because the closure moves into the HttpServer.
            // Each worker thread gets a clone of this web::Data.
            .app_data(web::Data::new(db_instance.clone()))
            .service(login)
            .service(register)
            // Register all poll services
            .service(create_poll)
            .service(get_poll)
            .service(update_poll)
            .service(delete_poll)
            .service(vote)
            .service(get_stats)
            .service(get_expiring_polls)
            .service(get_all_polls)
    })
    // Ensure this binding address is accessible by your clients
    .bind("0.0.0.0:8000")?
    .run()
    .await
}