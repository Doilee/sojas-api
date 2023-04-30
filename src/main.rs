mod users;
mod events;
mod jwt;
mod pinkpolitiek_api;

use std::env;
use actix_web::{HttpServer, App, HttpResponse, get, web};
use serde::{ Serialize, Deserialize };
use sqlx::mysql::{ MySqlPool, MySqlPoolOptions };

#[derive(Clone)]
pub struct AppState {
    pool: MySqlPool,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    sojas_api::set_env();

    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(root)
            .service(users::get)
            .service(users::get_all)
            .service(events::all_events)
            .service(events::participate)
            .service(events::stop_participating)
            //.service(vents::)
    }).bind(("127.0.0.1", 4000))?
        .run()
        .await
}

#[get("/")]
async fn root() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Server is up and running.".to_string(),
    })
}
