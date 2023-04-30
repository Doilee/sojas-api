use actix_web::{delete, get, HttpResponse, post, put, web};
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryResult;
use crate::{AppState, Response};

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    id: u32,
    soy_balance: Option<i32>,
    is_admin: Option<i8>,
}

#[get("/users/{id}")]
pub async fn get(path: web::Path<u32>, app_state: web::Data<AppState>) -> HttpResponse {
    let id: u32 = path.into_inner();

    let Ok(user): Result<User, sqlx::Error> = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id=?",
        id
    ).fetch_one(&app_state.pool).await else {
        return HttpResponse::BadRequest().json(Response {
            message: "No user found with given id.".to_string()
        });
    };

    HttpResponse::Ok().json(user.unwrap())
}

#[get("/users")]
pub async fn get_all(app_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(sqlx::query_as!(
        User,
        "SELECT * FROM users",
    ).fetch_all(&app_state.pool).await.unwrap())
}