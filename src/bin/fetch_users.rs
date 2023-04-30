extern crate sojas_api;

use std::env;
use std::process::{ExitCode};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PinkPolitiekUser {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> ExitCode {
    sojas_api::set_env();

    let url = env::var("PINKPOLITIEK_URL").unwrap() + "/wp/v2/users?page=1";

    // let client = reqwest::Client::new();
    //
    // let res = client
    //     .get(&url)
    //     .header(AUTHORIZATION, "Bearer ")
    //     .send()
    //     .await?;

    let Ok(response) = reqwest::get(&url).await else {
        return ExitCode::FAILURE
    };

    match response.status() {
        StatusCode::OK => {
            let users = response
                .json::<Vec<PinkPolitiekUser>>()
                .await
                .unwrap();

            dbg!(users);

            return ExitCode::SUCCESS;
        },
        // This is actually an unauthorized route
        _ => {
            println!("Code: {}\n URL: {}\n Body: {}", response.status(), &url, response.text().await.unwrap());

            return ExitCode::FAILURE;
        }
    };
}