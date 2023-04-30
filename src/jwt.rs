use std::env;
use std::future::Future;
use std::pin::Pin;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};

use reqwest::header::AUTHORIZATION;
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Role {
    Member,
    Admin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    role: Role,
}

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

        Box::pin(async move {
            match validate_token(&token).await {
                Ok(response) => {
                    match response.status() {
                        StatusCode::OK => {
                            Ok(User {
                                id: "1".to_string(),
                                first_name: Option::from("Matthijs".to_string()),
                                last_name: Option::from("Dam".to_string()),
                                role: Role::Member
                            })
                        },
                        _ => {
                            Err(ErrorUnauthorized("Token invalid."))
                        }
                    }
                },
                Err(_error) => {
                    Err(ErrorInternalServerError("Could not validate token."))
                }
            }
        })
    }
}

async fn validate_token(token : &str) -> Result<Response, reqwest::Error> {
    reqwest::Client::new()
        .post(env::var("PINKPOLITIEK_URL").unwrap() + "/jwt-auth/v1/token/validate")
        .header(AUTHORIZATION, token)
        .send()
        .await
}