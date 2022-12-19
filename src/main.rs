use std::sync::Mutex;
use std::env;
use actix_web::{ HttpServer,
                 App,
                 HttpResponse,
                 web };
use serde::{ Serialize, Deserialize };
use sqlx::mysql::{ MySqlPool, MySqlPoolOptions, MySqlQueryResult };

struct AppState {
    pool: Mutex<MySqlPool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct UserResponse {
    user: User,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct UsersResponse {
    users: Vec<User>,
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let _database_url: String = env::var("DATABASE_URL").unwrap();
    const DATABASE_URL: &str = "mysql://user:password@127.0.0.1:3306/sqlxdemo";

    /* Connecting to a database
     * for single connection:
     * [MySql|Sqlite|PgConnection...]Connection::connect()
     * 
     * for pool connection:
     * [MysqlPool|...]::connect()
     *
     * custom pool connection:
     * [MysqlPool|...]Options::new().connect()
     */
    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        // .connect("mysql://user:password@localhost:3306/sqlx_demo")
        .await
        .unwrap();

    let app_state: web::Data<AppState> = web::Data::new(AppState {
        pool: Mutex::new(pool)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(root))
            .route("/get/{user_id}", web::get().to(get_user))
            .route("/get-all", web::get().to(get_all_users))
            .route("/create", web::post().to(create_user))
            .route("/patch", web::patch().to(patch_user))
            .route("/delete/{user_id}", web::delete().to(delete_user))
    }).bind(("127.0.0.1", 4000))?
        .run()
        .await
}

async fn root() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "Server is up and running.".to_string(),
    })
}

async fn get_user(path: web::Path<i32>, app_state: web::Data<AppState>) -> HttpResponse {
    let pool: &MySqlPool = &*app_state.pool.lock().unwrap();
    let user_id: i32 = path.into_inner(); 
    let user: Result<User, sqlx::Error> = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id=?",
        user_id
    ).fetch_one(pool).await;

    if user.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "No user found with given id.".to_string()
        });
    }

    HttpResponse::Ok().json(UserResponse {
        user: user.unwrap(), 
        message: "Got user.".to_string(),
    })
}

async fn get_all_users(app_state: web::Data<AppState>) -> HttpResponse {
    let pool: &MySqlPool = &*app_state.pool.lock().unwrap();
    let users: Vec<User> = sqlx::query_as!(
        User,
        "SELECT * FROM users",
    ).fetch_all(pool).await.unwrap();

    HttpResponse::Ok().json(UsersResponse {
        users,
        message: "Got all users.".to_string(),
    })
}

#[derive(Serialize, Deserialize)]
struct CreateUserBody {
    username: String,
    email: String
}

async fn create_user(body: web::Json<CreateUserBody>, app_state: web::Data<AppState>) -> HttpResponse {
    let pool: &MySqlPool = &*app_state.pool.lock().unwrap();

    let created: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        "INSERT INTO users(username, email) VALUES(?, ?)",
        body.username,
        body.email,
    ).execute(pool).await;

    if created.is_err() {
        println!("{}", created.unwrap_err());
        return HttpResponse::InternalServerError().json(Response {
            message: "Couldn't create a new user.".to_string(),
        });
    }

    HttpResponse::Ok().json(Response {
        message: "Created a user.".to_string(),
    })
}

#[derive(Serialize, Deserialize)]
struct PatchUserBody {
    username: Option<String>,
    email: Option<String>,
}

async fn patch_user(body: web::Json<PatchUserBody>, app_state: web::Data<AppState>) -> HttpResponse {
    let pool: &MySqlPool = &*app_state.pool.lock().unwrap();

    /* Patch username */
    if body.username.is_some() {
        let patch_username: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
            "",
            body.username.unwrap(),
        ).execute(pool).await;

        if patch_username.is_err() {
            return HttpResponse::InternalServerError().json(Response {
                message: "Couldn't patch username.".to_string(),
            });
        }
    }

    /* Patch email */
    if body.email.is_some() {
        let patch_email: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
            "",
            body.email.unwrap(),
        ).execute(pool).await;

        if patch_email.is_err() {
            return HttpResponse::InternalServerError().json(Response {
                message: "Couldn't patch email.".to_string(),
            });
        }
    }

    HttpResponse::Ok().json(Response {
        message: "Updated the user.".to_string(),
    })
}

async fn delete_user(path: web::Path<i32>, app_state: web::Data<AppState>) -> HttpResponse {
    let pool: &MySqlPool = &*app_state.pool.lock().unwrap();
    let user_id: i32 = path.into_inner();

    let deleted: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        "DELETE FROM users WHERE id=?",
        user_id,
    ).execute(pool).await;

    if deleted.is_err() {
        println!("{}", deleted.unwrap_err());
        return HttpResponse::InternalServerError().json(Response {
            message: "Couldn't delete the user.".to_string(),
        });
    }

    HttpResponse::Ok().json(Response {
        message: "Deleted the user.".to_string(),
    })
}
