use std::env;
use actix_web::{get, post, HttpResponse, web};
use actix_web::web::Data;
use reqwest::{StatusCode};
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;
use crate::{AppState, Response};
use urlencoding::encode;
use crate::pinkpolitiek_api::{PPErrorResponse, PPLoginResponse};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u32,
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub soy_balance: i32,
    pub is_admin: i8,
    pub jwt: String,
}

#[derive(Deserialize)]
pub struct LoginBody {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(body: web::Json<LoginBody>, app_state: web::Data<AppState>) -> HttpResponse {
    let url = env::var("PINKPOLITIEK_URL").unwrap() + "/jwt-auth/v1/token?username=" + &encode(&body.username).to_string() + "&password=" + &encode(&body.password).to_string();

    let client = reqwest::Client::new();

    let res = client
        .post(&url)
        .send();

    let Ok(response) = res.await else {
        return HttpResponse::ServiceUnavailable().json("Something went wrong when connecting with pinkpolitiek.nl.");
    };

    return match response.status() {
        StatusCode::OK => {
            let login_response = response
                .json::<PPLoginResponse>()
                .await
                .unwrap();

            match store_to_db(app_state, &login_response).await {
                Ok(_) => HttpResponse::Ok().json(login_response),
                Err(message) => HttpResponse::InternalServerError().json(message)
            }
        },
        StatusCode::NOT_FOUND => {
            HttpResponse::NotFound().json(response.json::<PPErrorResponse>().await.unwrap())
        },
        _ => {
            HttpResponse::InternalServerError().json(response.json::<PPErrorResponse>().await.unwrap())
        }
    };
}

pub async fn store_to_db(app_state: Data<AppState>, login_response: &PPLoginResponse) -> Result<(), String> {
    let result = sqlx::query!(r#"
         INSERT INTO users (display_name, username, email, jwt) VALUES(?, ?, ?, ?)
         AS n ON DUPLICATE KEY UPDATE display_name = n.display_name, username = n.username, email = n.email, jwt = n.jwt"#,
        login_response.user_display_name,
        login_response.user_nicename,
        login_response.user_email,
        login_response.token,
    ).execute(&app_state.pool).await;


    if result.is_err() {
        return Err("Something went wrong.".to_string())
    }

    Ok(())
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

#[get("/me")]
pub async fn get_me(app_state: web::Data<AppState>, user: User) -> HttpResponse {
    HttpResponse::Ok().json(user)
}

#[get("/users")]
pub async fn get_all(app_state: web::Data<AppState>, user: User) -> HttpResponse {
    HttpResponse::Ok().json(sqlx::query_as!(
        User,
        "SELECT * FROM users",
    ).fetch_all(&app_state.pool).await.unwrap())
}