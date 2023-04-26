use std::env;
use std::future::Future;
use std::num::NonZeroI16;
use std::pin::Pin;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures::future::{err, ok, Ready};
use reqwest::header::AUTHORIZATION;
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};

// Define a struct to hold the JWT token
#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Role {
    Guest,
    User,
    Admin,
}

impl Default for Role {
    fn default() -> Self {
        Role::Guest
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct User {
    id: String,
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
        if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
            let token = auth_header.to_str().unwrap().replace("Bearer ", "");

            return Box::pin(async move {
                return match validate_token(&token).await {
                    Ok(response) => {
                        return match response.status() {
                            StatusCode::OK => {
                                Ok(User {
                                    id: "1".to_string(),
                                    first_name: Option::from("Matthijs".to_string()),
                                    last_name: Option::from("Dam".to_string()),
                                    role: Role::Admin
                                })
                            },
                            _ => {
                                Err(ErrorUnauthorized("Token invalid."))
                            }
                        }
                    },
                    Err(error) => {
                        Err(ErrorUnauthorized("Could not validate token."))
                    }
                }
            });
        }

        return Box::pin(async {
            Err(ErrorUnauthorized("Unauthorized"))
        });

        // if let Some(auth_header) = req.headers().get("authorization") {
        //
        //     return check_token_on_pinkpolitiek(auth_header.to_str().unwrap());

            // let token = auth_header.to_str().unwrap().replace("Bearer ", "");

            // let decoding_key = DecodingKey::from_secret("my_secret_key".as_ref());

            // match decode::<Claims>(&token, &decoding_key, &Validation::default()) {
            //     Ok(token_data) => return ok(token_data.claims),
            //     Err(_) => return err(actix_web::error::ErrorUnauthorized("Invalid token")),
            // }
        // }
        // err(actix_web::error::ErrorUnauthorized("Authorization header not found"))
    }
}

async fn validate_token(token : &str) -> Result<Response, reqwest::Error> {
    let api_url: String = env::var("PINKPOLITIEK_URL").unwrap();

    let client = reqwest::Client::new();

    return client
        .post(api_url + "/wp-json/jwt-auth/v1/token/validate")
        .header(AUTHORIZATION, token)
        .send()
        .await;
}