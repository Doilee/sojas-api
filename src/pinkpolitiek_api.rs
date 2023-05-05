use std::env;
use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use crate::AppState;

#[derive(Serialize, Deserialize, Clone)]
struct Venue {
    id: i32,
    url: String,

    #[serde(rename(deserialize = "venue"))]
    name: String,
    show_map: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum PPVenueOrVec {
    Venue(Venue),
    Arr([u8; 0])
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PPEvent {
    id: i32,
    global_id: String,
    author: String,
    status: String,
    date: String,
    date_utc: String,
    modified: String,
    modified_utc: String,
    url: String,
    rest_url: String,
    title: String,
    description: String,
    excerpt: String,
    slug: String,
    all_day: bool,
    start_date: String,
    venue: PPVenueOrVec,
}

#[derive(Serialize, Deserialize)]
pub struct PPEventsData {
    events: Vec<PPEvent>,
    rest_url: String,
    total: i32,
    total_pages: i32,
}

pub async fn get_events(app_state: Data<AppState>) -> Result<Vec<PPEvent>, String> {
    let response = reqwest::get(
        env::var("PINKPOLITIEK_URL").unwrap() + "/tribe/events/v1/events"
    ).await;

    if response.is_err() {
        return Err("Could not connect to ".to_owned() + &env::var("PINKPOLITIEK_URL").unwrap());
    }

    let events = response
        .unwrap()
        .json::<PPEventsData>()
        .await
        .unwrap()
        .events;

    match store_to_db(app_state, &events).await {
        Ok(_) => Ok(events),
        Err(message) => Err(message)
    }
}

async fn store_to_db(app_state: Data<AppState>, events: &Vec<PPEvent>) -> Result<(), String> {
    for event in events {
        let result = sqlx::query!(r#"
            INSERT INTO events (id, title, description, source, url)
            VALUES(?, ?, ?, 'external', ?) AS n ON DUPLICATE KEY UPDATE
            title = n.title, description = n.description, source = n.source, url = n.url"#,
            event.id,
            event.title,
            event.description,
            event.url,
        ).execute(&app_state.pool).await;

        if result.is_err() {
            return Err("Something went wrong.".to_string())
        }
    }

    Ok(())
}