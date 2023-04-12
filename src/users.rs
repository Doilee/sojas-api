use actix_web::{delete, get, HttpResponse, post, put, web};
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryResult;
use crate::{AppState, DataResponse, Response};

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    id: u32,
    soy_balance: Option<i32>,
    is_admin: Option<i8>,
}

#[get("/users/{id}")]
pub async fn get(path: web::Path<u32>, app_state: web::Data<AppState>) -> HttpResponse {
    let id: u32 = path.into_inner();

    let user: Result<User, sqlx::Error> = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id=?",
        id
    ).fetch_one(&app_state.pool).await;

    if user.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "No user found with given id.".to_string()
        });
    }

    HttpResponse::Ok().json(DataResponse {
        data: user.unwrap(),
        message: Option::from("Got user.".to_string()),
    })
}

#[get("/users")]
pub async fn get_all(app_state: web::Data<AppState>) -> HttpResponse {
    let users: Vec<User> = sqlx::query_as!(
        User,
        "SELECT * FROM users",
    ).fetch_all(&app_state.pool).await.unwrap();

    HttpResponse::Ok().json(DataResponse {
        data: users,
        message: Option::from("Got all users.".to_string()),
    })
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserBody {
    username: String,
    email: String
}

// #[post("/users")]
// pub async fn create(body: web::Json<CreateUserBody>, app_state: web::Data<AppState>) -> HttpResponse {
//     let created: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
//         "INSERT INTO users(id, soy_balance, is_admin) VALUES(?, ?, ?)",
//         body.id,
//         body.soy_balance,
//         body.is_admin,
//     ).execute(&app_state.pool).await;
//
//     if created.is_err() {
//         println!("{}", created.unwrap_err());
//         return HttpResponse::InternalServerError().json(Response {
//             message: "Couldn't create a new user.".to_string(),
//         });
//     }
//
//     HttpResponse::Ok().json(Response {
//         message: "Created a user.".to_string(),
//     })
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct PutUserBody {
//     id: i32,
//     username: Option<String>,
//     email: Option<String>,
// }
//
// #[put("/users/{id}")]
// pub async fn update(body: web::Json<PutUserBody>, app_state: web::Data<AppState>) -> HttpResponse {
//     /* Put username */
//     if body.username.is_some() {
//         let patch_username: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
//             "UPDATE users SET username = ? WHERE id = ?",
//             body.username.as_ref().unwrap(),
//             body.id,
//         ).execute(&app_state.pool).await;
//
//
//         if patch_username.is_err() {
//             return HttpResponse::InternalServerError().json(Response {
//                 message: "Couldn't patch username.".to_string(),
//             });
//         }
//     }
//
//     /* Patch email */
//     if body.email.is_some() {
//         let patch_email: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
//             "UPDATE users SET email = ? WHERE id = ?",
//             body.email.as_ref().unwrap(),
//             body.id,
//         ).execute(&app_state.pool).await;
//
//         if patch_email.is_err() {
//             return HttpResponse::InternalServerError().json(Response {
//                 message: "Couldn't patch email.".to_string(),
//             });
//         }
//     }
//
//     HttpResponse::Ok().json(Response {
//         message: "Updated the user.".to_string(),
//     })
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct DeleteUserBody {
//     id: i32,
// }
//
// #[delete("/users/{id}")]
// pub async fn delete(body: web::Json<DeleteUserBody>, app_state: web::Data<AppState>) -> HttpResponse {
//     let deleted: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
//         "DELETE FROM users WHERE id=?",
//         body.id,
//     ).execute(&app_state.pool).await;
//
//     if deleted.is_err() {
//         println!("{}", deleted.unwrap_err());
//         return HttpResponse::InternalServerError().json(Response {
//             message: "Couldn't delete the user.".to_string(),
//         });
//     }
//
//     HttpResponse::Ok().json(Response {
//         message: "Deleted the user.".to_string(),
//     })
// }