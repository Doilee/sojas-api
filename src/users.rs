use actix_web::{get, HttpResponse, web};
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;
use crate::{AppState, Response};

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    id: u32,
    name: String,
    soy_balance: i32,
    is_admin: i8,
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

    HttpResponse::Ok().json(user)
}

#[get("/users")]
pub async fn get_all(app_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(sqlx::query_as!(
        User,
        "SELECT * FROM users",
    ).fetch_all(&app_state.pool).await.unwrap())
}