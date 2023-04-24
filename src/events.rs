use std::env;
use std::ops::Deref;
use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, web};
use actix_web::web::{Data, Query};
use futures::future::err;
use futures::StreamExt;
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::Map;
use serde_json::Value::Array;
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryResult;
use sqlx::types::JsonValue;
use crate::{AppState, DataResponse};

#[derive(Serialize, Deserialize, Copy, Clone)]
struct Participant {
    user_id: u32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
struct Event {
    id: u32,
    cached_results: Option<JsonValue>,
    participants: Vec<Participant>
}

#[derive(Serialize, Deserialize, FromRow)]
struct DatabaseResult {
    id: u32,
    cached_results: Option<JsonValue>,
    user_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
struct PinkPolitiekVenue {
    id: i32,
    author: String,
    status: String,
    date: String,
    date_utc: String,
    modified: String,
    modified_utc: String,
    url: String,
    venue: String,
    slug: String,
    show_map: bool,
    show_map_link: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum PinkPolitiekVenueOrVec {
    Venue(PinkPolitiekVenue),
    Arr([u8; 0])
}

#[derive(Serialize, Deserialize, Clone)]
struct PinkPolitiekEvent {
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
    venue: PinkPolitiekVenueOrVec,
}

#[derive(Serialize, Deserialize)]
struct PinkPolitiekEventsData {
    events: Vec<PinkPolitiekEvent>,
    rest_url: String,
    total: i32,
    total_pages: i32,
}

#[derive(Serialize, Deserialize)]
struct IndexParams {
    cached: bool
}

#[get("/events")]
pub async fn all_events(app_state: Data<AppState>, req: HttpRequest) -> HttpResponse {
    let params = web::Query::<IndexParams>::from_query(req.query_string())
        .unwrap_or(web::Query(IndexParams { cached: false }));

    if params.cached == true {
        return cached_events(app_state).await;
    }

    let events = fetch_events_from_pink_politiek().await;

    for event in &events.events {
        let result = sqlx::query!(
            "INSERT INTO events (id, cached_results) VALUES(?, ?) AS new ON DUPLICATE KEY UPDATE cached_results = new.cached_results",
            event.id,
            serde_json::to_string(&event).unwrap(),
        ).execute(&app_state.pool).await;

        if result.is_err() {
            return actix_web::error::ErrorInternalServerError("Something went wrong.").error_response()
        }
    }

    HttpResponse::Ok().json(DataResponse {
        data: events,
        message: Option::from("Got all events.".to_string()),
    })
}

async fn cached_events(app_state: web::Data<AppState>) -> HttpResponse {
    let results: Vec<DatabaseResult> = sqlx::query_as!(
        DatabaseResult,
        "SELECT id, cached_results, user_id FROM events LEFT JOIN participants ON participants.event_id = events.id",
    ).fetch_all(&app_state.pool).await.unwrap();

    HttpResponse::Ok().json(DataResponse {
        data: convert_results_to_events(results),
        message: Option::from("Got all cached events.".to_string()),
    })
}

fn convert_results_to_events(results: Vec<DatabaseResult>) -> Vec<Event> {
    let mut event_map: std::collections::HashMap<u32, Event> = std::collections::HashMap::new();

    for result in results {
        let event = event_map.entry(result.id).or_insert_with(|| Event {
            id: result.id,
            cached_results: result.cached_results,
            participants: vec![],
        });

        if let Some(user_id) = result.user_id {
            event.participants.push(Participant { user_id });
        }
    }

    event_map.into_iter().map(|(_, event)| event).collect()
}

async fn fetch_events_from_pink_politiek() -> PinkPolitiekEventsData {
    let api_url: String = env::var("PINKPOLITIEK_URL").unwrap();

    return reqwest::get(api_url + "/wp-json/tribe/events/v1/events")
        .await
        .unwrap()
        .json::<PinkPolitiekEventsData>()
        .await
        .unwrap();
}

// todo: Auth before being able to participate to events
// #[derive(Serialize, Deserialize)]
// struct ParticipateBody {
//     event_id: i32,
//     user_id: i32,
// }
//
// #[put("/participate/{event_id}")]
// pub async fn participate(app_state: web::Data<AppState>) -> HttpResponse {
//     let created: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
//         "INSERT INTO participants(event_id, user_id) VALUES(?, ?)",
//         body.event_id,
//         body.user_id,
//     ).execute(&app_state.pool).await;
//
//     if created.is_err() {
//         return HttpResponse::InternalServerError().json(Response {
//             message: "Couldn't participate to event.".to_string(),
//         });
//     }
//
//     HttpResponse::Ok().json(Response {
//         message: "Created a user.".to_string(),
//     })
// }
