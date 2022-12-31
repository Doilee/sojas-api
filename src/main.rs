use actix_web::{ HttpServer,
                 App,
                 HttpResponse,
                 web };
use serde::{ Serialize, Deserialize };
use sqlx::mysql::{ MySqlPool, MySqlPoolOptions, MySqlQueryResult };

#[derive(Clone)]
struct AppState {
    pool: MySqlPool,
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

    // let _database_url: String = env::var("DATABASE_URL").unwrap();
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
        .await
        .unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(root))
            .route("/get/{user_id}", web::get().to(get_user))
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

async fn get_user(path: web::Path<i32>, app_state: web::Data<AppState>) -> HttpResponse {
    let user_id: i32 = path.into_inner(); 
    /* Queries
     * prepared (parameterized):
     *   have their quey plan cached, use a
     *   binary mode of communication (lower
     *   bandwith and faster decoding), and
     *   utilize parameters to avoid SQL
     *   Injection
     * unprepared (simple):
     *   intended only for use case where
     *   prepared  statement will not work
     * 
     * &str is treated as an unprepared query
     * Query or QueryAs struct is treated as
     * prepared query
     *
     *  conn.execute("BEGIN").await                            <- unprepared
     *  conn.execute(sqlx::query("DELETE FROM table")).await   <- prepared
     * 
     * All methods accept one of &mut {connection type}, &mut Transaction or &Pool
     * 
     * sqlx::query(""); // Query
     * sqlx::query_as(""); // QueryAs
     * sqlx::query("QUERY").fetch_one(&pool).await // <- sqlx::Result<MySqlRow>
     * sqlx::query_as("QUERY").fetch_one(&pool).await // <- sqlx::Result<User> which derives FromRow
     */

    /* sqlx::query Example:
     * let user: sqlx::Result<MySqlRow> = sqlx::query("").bind().fetch().await
     */

    /* sqlx::query_as Example:
     * #[derive(sqlx::FromRow)]
     * struct UserRow {
     *     id: i32,
     *     email: String,
     *     username: String,
     * }
     * 
     * let user_0: sqlx::Result<UserRow> = sqlx::query_as("SELECT * FROM users WHERE id=?")
     *    .bind(user_id)
     *    .fetch_one(&app_state.pool)
     *    .await;
     */

    let user: Result<User, sqlx::Error> = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id=?",
        user_id
    ).fetch_one(&app_state.pool).await;
    // fetch                   Stream                               call .try_nex()
    // fetch_optional  .await  sqlx::Result<Option<T>>              extra rows are ignored
    // fetch_all       .await  sqlx::Result<Vec<T>>
    // fetch_one       .await  sqlx::Result<T>                      error if no rows, extra rows are ignored
    // execute         .await  sqlx::Result<Database::QueryResult>  for INSERT/UPDATE/DELETE without returning

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
    let users: Vec<User> = sqlx::query_as!(
        User,
        "SELECT * FROM users",
    ).fetch_all(&app_state.pool).await.unwrap();

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
    let created: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        "INSERT INTO users(username, email) VALUES(?, ?)",
        body.username,
        body.email,
    ).execute(&app_state.pool).await;

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
    id: i32,
    username: Option<String>,
    email: Option<String>,
}

async fn patch_user(body: web::Json<PatchUserBody>, app_state: web::Data<AppState>) -> HttpResponse {
    /* Patch username */
    if body.username.is_some() {
        let patch_username: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
            "UPDATE users SET username = ? WHERE id = ?",
            body.username.as_ref().unwrap(),
            body.id,
        ).execute(&app_state.pool).await;

        if patch_username.is_err() {
            return HttpResponse::InternalServerError().json(Response {
                message: "Couldn't patch username.".to_string(),
            });
        }
    }

    /* Patch email */
    if body.email.is_some() {
        let patch_email: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
            "UPDATE users SET email = ? WHERE id = ?",
            body.email.as_ref().unwrap(),
            body.id,
        ).execute(&app_state.pool).await;

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

#[derive(Serialize, Deserialize)]
struct DeleteUserBody {
    id: i32,
}

async fn delete_user(body: web::Json<DeleteUserBody>, app_state: web::Data<AppState>) -> HttpResponse {
    let deleted: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        "DELETE FROM users WHERE id=?",
        body.id,
    ).execute(&app_state.pool).await;

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
