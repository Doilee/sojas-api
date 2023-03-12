use actix_web::{ HttpServer,
                 App,
                 HttpResponse,
                 web };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(root))
            // .route("/users", web::get().to(get_all_users))
    }).bind(("127.0.0.1", 4000))?
        .run()
        .await
}

async fn root() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Server is up and running.".to_string(),
    })
}