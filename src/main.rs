mod users;
mod events;
mod jwt;

use std::{env, fs};
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

#[derive(Serialize, Deserialize)]
struct DataResponse<T> {
    data: T,
    message: Option<String>,
}
fn set_env() {
    let file = fs::read_to_string(".env").unwrap();

    for var in file.split("\n").into_iter() {
        let key_value : Vec<&str> = var.split("=").collect();

        env::set_var(key_value[0], key_value[1]);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_env();

    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&*env::var("DATABASE_URL").unwrap())
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
