mod users;
mod events;

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
struct DataResponse<Data> {
    data: Data,
    message: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let _database_url: String = env::var("DATABASE_URL").unwrap();
    const DATABASE_URL: &str = "mysql://root:localhost@127.0.0.1:3306/sojas_api";
    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(root)
            .service(users::get)
            .service(users::get_all)
            .service(events::index)
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
