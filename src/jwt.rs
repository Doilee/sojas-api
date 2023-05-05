use std::future::Future;
use std::pin::Pin;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;

use reqwest::header::AUTHORIZATION;
use crate::AppState;
use crate::users::User;

// Implement the FromRequest trait to extract the JWT token from the Authorization header
impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<User, Error>>>>;
    // type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if req.headers().get(AUTHORIZATION).is_none() {
            return Box::pin(async {
                Err(ErrorUnauthorized("Please provide an authorization token."))
            });
        }

        let token = req.headers().get(AUTHORIZATION)
            .unwrap()
            .to_str()
            .unwrap()
            .replace("Bearer ", "");

        let app_state = req.app_data::<Data<AppState>>().unwrap().to_owned();

        Box::pin(async move {
            match validate_token(&token, &app_state).await {
                Ok(user) => Ok(user),
                Err(message) => Err(ErrorInternalServerError(message))
            }
        })
    }
}

async fn validate_token(token : &str, app_state: &Data<AppState>) -> Result<User, String> {
    let Ok(user): Result<User, sqlx::Error> = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE jwt=?",
        token
    ).fetch_one(&app_state.pool).await else {
        return Err("Unauthorized.".to_string())
    };

    Ok(user)
}

// Old way, but not removing as it may prove useful in the future
// async fn validate_token(token : &str) -> Result<Response, reqwest::Error> {
//     reqwest::Client::new()
//         .post(env::var("PINKPOLITIEK_URL").unwrap() + "/jwt-auth/v1/token/validate")
//         .header(AUTHORIZATION, token)
//         .send()
//         .await
// }