use actix_web::{web, App};
use dotenv::dotenv;
use errors::EzyTutorError;
use sqlx::postgres::PgPool;
use state::AppState;
use std::{env, sync::Mutex};

use routes::*;

// ______________________________________________________________________
#[path = "../iter5/dbaccess/mod.rs"]
mod dbaccess;
#[path = "../iter5/errors.rs"]
mod errors;
#[path = "../iter5/handlers/mod.rs"]
mod handlers;
#[path = "../iter5/models/mod.rs"]
mod models;
#[path = "../iter5/routes.rs"]
mod routes;
#[path = "../iter5/state.rs"]
mod state;

// ╾────────────────────────────────────────────────────────────────────╼
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You have already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        let shared_data = shared_data.clone();
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                EzyTutorError::InvalidInput("Please provide valid JSON input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };

    // * INFO: Start HTTP Server
    println!("Starting HTTP server: 127.0.0.1:3002");

    let host_port = env::var("HOST_PORT").expect("HOST:PORT is not set in .env file");

    actix_web::HttpServer::new(app)
        .bind(&host_port)?
        .run()
        .await
}
