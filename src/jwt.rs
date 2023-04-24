use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

// Define a struct to hold the JWT token
#[derive(Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

// Implement the FromRequest trait to extract the JWT token from the Authorization header
impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    // type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            let token = auth_header.to_str().unwrap().replace("Bearer ", "");
            let decoding_key = DecodingKey::from_secret("my_secret_key".as_ref());

            match decode::<Claims>(&token, &decoding_key, &Validation::default()) {
                Ok(token_data) => return ok(token_data.claims),
                Err(_) => return err(actix_web::error::ErrorUnauthorized("Invalid token")),
            }
        }
        err(actix_web::error::ErrorUnauthorized("Authorization header not found"))
    }
}