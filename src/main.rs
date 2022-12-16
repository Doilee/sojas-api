use std::sync::Mutex;
use std::env;
use actix_web::{ HttpServer,
                 App,
                 HttpResponse,
                 web };
use serde::{ Serialize, Deserialize };
use sqlx::mysql::{ MySqlPool, MySqlPoolOptions, MySqlRow };
use sqlx;
// use futures::TryStreamExt; // row.try_next()

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
    const DATABASE_URL: &str = "mysql://user:password@127.0.0.1:3306/sqlx_demo";

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
    /* Queries:
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
     * .execute   <- returns number of affected rows, drops result 
     * receive results:
     * .fetch           <- the fetch query finalizer returns a stream-like
     *                     type that iterates through the rows in the result sets.
     * .fetch_one       <- fetch_one & fetch_optional to request one required or
     *                     optional result from the database
     * .fetch_optional  <-
     * .fetch_all
     * 
     * sqlx::query  <- will return Row<'conn>
     *   Column values can be accessed with row.get()
     */
    let mut pool: MySqlPool = *app_state.pool.lock().unwrap();

    // TODO what is difference between fetch && fetch_all
    // TODO fetch(&mut conn)
    // TODO fetch_all(&pool)
    // TODO what is fetch & and what is fetch_all
    // TODO why does fetch need connection and why does fetch_all need pooled connection
    // TODO fetch & fetch_all accepts &mut conn or &pool??

    let users_0: Vec<MySqlRow> = sqlx::query("SELECT * FROM users")
        .map(|row: MySqlRow| {
            // ...
        })
        .fetch(&pool);

    let users_1: Vec<MySqlRow> = sqlx::query("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    // query_as is like query but with typing casting
    #[derive(sqlx::FromRow)]
    struct User {
        id: u64,
        username: String,
        email: String,
    }

    let users_2: Vec<User> = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch(&pool);

    /* query! macro
     * to achieve compile-time syntactic and
     * semantic verification of the SQL
     * 
     * DATABASE_URL environment variable must be set at build time
     */
    let users_3 = sqlx::query!(
        "SELECT * FROM users"
        // , binded values
    ).fetch_all(&pool) // -> Vec<{ id: i64, username: String, email: String }>
        .await             //    The output type is an anonymous record.
        .unwrap();

    // named output type
    let users_4: Vec<User> = sqlx::query_as!(
        User,
        "SELECT * FROM users",
        // , binded values
    ).fetch_all(&pool) // -> Vec<User>
        .await
        .unwrap();

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
