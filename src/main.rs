mod users;
mod events;
mod jwt;
mod pinkpolitiek_api;

use std::env;
use actix_web::{HttpServer, App, HttpResponse, get, web, http};
use serde::{ Serialize, Deserialize };
use sqlx::mysql::{ MySqlPool, MySqlPoolOptions };
use actix_cors::Cors;

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
        let cors = Cors::default()
              .allowed_origin("http://localhost:5714")
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .service(root)
            .service(users::login)
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
