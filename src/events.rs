use std::env;
use actix_web::{delete, get, HttpRequest, HttpResponse, post, web};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Query};
use futures::future::err;
use futures::StreamExt;
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::Map;
use serde_json::Value::Array;
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryResult;
use sqlx::types::JsonValue;
use crate::{AppState, Response};
use crate::jwt::User;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
enum PinkPolitiekVenueOrVec {
    Venue(PinkPolitiekVenue),
    Arr([u8; 0])
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

    let Ok(response) = reqwest::get(
        env::var("PINKPOLITIEK_URL").unwrap() + "/tribe/events/v1/events"
    ).await else {
        return ErrorInternalServerError("Could not connect to ".to_owned() + &env::var("PINKPOLITIEK_URL")
            .unwrap())
            .error_response();
    };

    let events = response
        .unwrap()
        .json::<PinkPolitiekEventsData>()
        .await
        .unwrap()
        .events;

    for event in &events {
        let result = sqlx::query!(
            "INSERT INTO events (id, cached_results) VALUES(?, ?) AS new ON DUPLICATE KEY UPDATE cached_results = new.cached_results",
            event.id,
            serde_json::to_string(&event).unwrap(),
        ).execute(&app_state.pool).await;

        if result.is_err() {
            return ErrorInternalServerError("Something went wrong.").error_response()
        }
    }

    HttpResponse::Ok().json(events)
}

async fn cached_events(app_state: web::Data<AppState>) -> HttpResponse {
    let results: Vec<DatabaseResult> = sqlx::query_as!(
        DatabaseResult,
        "SELECT id, cached_results, user_id FROM events LEFT JOIN participants ON participants.event_id = events.id",
    ).fetch_all(&app_state.pool).await.unwrap();

    HttpResponse::Ok().json(convert_results_to_events(results))
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

#[post("/events/{event_id}/participate")]
pub async fn participate(path: web::Path<u32>, app_state: web::Data<AppState>, user: User) -> HttpResponse {
    let event_id: u32 = path.into_inner();

    //todo: Check if user already participated

    let Ok(created): Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        "INSERT INTO participants(event_id, user_id) VALUES(?, ?)",
        event_id,
        user.id,
    ).execute(&app_state.pool).await else {
        return HttpResponse::InternalServerError().json(Response {
            message: "Couldn't participate to event.".to_string(),
        });
    };

    HttpResponse::Ok().json(Response {
        message: "Participated to event.".to_string(),
    })
}

#[delete("/events/{event_id}/participate")]
pub async fn stop_participating(path: web::Path<u32>, app_state: web::Data<AppState>, user: User) -> HttpResponse {
    let event_id: u32 = path.into_inner();

    let Ok(deleted): Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        "DELETE FROM participants WHERE event_id = ? AND user_id = ?",
        event_id,
        user.id,
    ).execute(&app_state.pool).await else {
        return HttpResponse::InternalServerError().json(Response {
            message: "Couldn't remove participation.".to_string(),
        });
    };

    HttpResponse::Ok().json(Response {
        message: "Stopped participating to event.".to_string(),
    })
}
