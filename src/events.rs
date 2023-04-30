use std::env;
use std::ops::Deref;
use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, web};
use actix_web::web::{Data, Query};
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

fn convert_results_to_events(results: Vec<DatabaseResult>) -> Vec<Event> {
    let mut event_list: Vec<Event> = Vec::new();

    for result in results {
        let event: Option<&mut Event> = event_list.iter_mut().find(|e| e.id == result.id);

        match event {
            Some(event) => {
                if result.user_id.is_some() {
                    event.participants.push(Participant { user_id: result.user_id.unwrap() });
                }
            },
            None => {
                let mut new_event = Event {
                    id: result.id,
                    cached_results: result.cached_results,
                    participants: vec![],
                };

                if result.user_id.is_some() {
                    new_event.participants.push(Participant { user_id: result.user_id.unwrap() });
                }

                event_list.push(new_event);
            }
        }
    }

    event_list
}

pub async fn cached_events(app_state: web::Data<AppState>) -> HttpResponse {
    let results: Vec<DatabaseResult> = sqlx::query_as!(
        DatabaseResult,
        "SELECT id, cached_results, user_id FROM events LEFT JOIN participants ON participants.event_id = events.id",
    ).fetch_all(&app_state.pool).await.unwrap();

    HttpResponse::Ok().json(convert_results_to_events(results))
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

type PinkPolitiekEvents = Vec<PinkPolitiekEvent>;

#[derive(Serialize, Deserialize)]
struct PinkPolitiekEventsData {
    events: PinkPolitiekEvents,
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
        sqlx::query!(
            "UPDATE events SET cached_results = ? WHERE id = ?",
            serde_json::to_string(&event).unwrap(),
            event.id
        ).execute(&app_state.pool);
    }

    HttpResponse::Ok().json(events)
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
