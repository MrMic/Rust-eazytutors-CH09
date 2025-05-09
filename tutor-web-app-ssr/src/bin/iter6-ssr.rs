#[path = "../iter6/mod.rs"]
mod iter6;
use std::env;

use actix_web::{
    App, HttpServer,
    web::{self, Data},
};
use dotenv::dotenv;
use iter6::{routes::app_config, state::AppState};
use sqlx::postgres::PgPool;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // * INFO: Start HTTP Server
    let host_port = env::var("HOST_PORT").expect("HOST:HOST_PORT is not set in .env file");
    println!("Listenig on http://{}", &host_port);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // * INFO: Contruct APP State
    let shared_data = web::Data::new(AppState {
        db: db_pool.clone(),
    });

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter6/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .app_data(shared_data.clone())
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
