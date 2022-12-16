use std::sync::Mutex;
use std::env;
use actix_web::{ HttpServer,
                 App,
                 HttpResponse,
                 web };
use serde::{ Serialize, Deserialize };
use sqlx::mysql::{ MySqlPool, MySqlPoolOptions };

struct AppState {
    pool: Mutex<MySqlPool>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url: String = env::var("DATABASE_URL").unwrap();

    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://user:password@127.0.0.1:3306/dbconn")
        .await
        .unwrap();

    let mut app_state: web::Data<AppState> = web::Data::new(AppState {
        pool: Mutex::new(pool)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(root))
            .route("/get", web::get().to(get_user))
            .route("/get-all", web::get().to(get_all_users))
            .route("/create", web::post().to(create_user))
            .route("/patch", web::patch().to(patch_user))
            .route("/delete", web::delete().to(delete_user))

    }).bind(("127.0.0.1", 4000))?
        .run()
        .await
}

async fn root() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Server is up and running.".to_string(),
    })
}

async fn get_user(app_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Got user.".to_string(),
    })
}

async fn get_all_users() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Got all users.".to_string(),
    })
}

async fn create_user() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Created a user.".to_string(),
    })
}

async fn patch_user() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Updated a user.".to_string(),
    })
}

async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Deleted a user.".to_string(),
    })
}
