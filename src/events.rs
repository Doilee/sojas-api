use actix_web::{delete, get, HttpRequest, HttpResponse, post, web};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryResult;

use crate::{AppState, Response};
use crate::users::User;

#[derive(Serialize, Deserialize, Clone, sqlx::Type)]
#[serde(rename_all = "lowercase")]
enum Source {
    Local,
    External,
}

impl Source {
    fn from_str(input: &str) -> Source {
        match input {
            "local"    => Source::Local,
            "external" => Source::External,
            _          => panic!("String needs to be either 'local' or 'external'."),
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
struct ParticipantModel {
    user_id: u32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
struct Event {
    id: u32,
    region_id: Option<u32>,
    title: String,
    description: Option<String>,
    reward: i32,
    source: Source,
    url: Option<String>,
    image_url: Option<String>,
    participants: Vec<ParticipantModel>
}

#[derive(Serialize, Deserialize)]
struct IndexParams {
    cached: bool
}

#[get("/events")]
pub async fn all_events(app_state: Data<AppState>, req: HttpRequest) -> HttpResponse {
    let params = web::Query::<IndexParams>::from_query(req.query_string())
        .unwrap_or(web::Query(IndexParams { cached: false }));

    if params.cached {
        return cached_events(app_state).await;
    }

    match crate::pinkpolitiek_api::get_events(app_state).await {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(message) => ErrorInternalServerError(message).error_response()
    }
}

#[derive(Serialize, Deserialize, FromRow)]
struct DatabaseResult {
    id: u32,
    region_id: Option<u32>,
    title: String,
    description: Option<String>,
    reward: i32,
    source: String,
    url: Option<String>,
    image_url: Option<String>,
    image_srcset: Option<String>,
    user_id: Option<u32>,
}

async fn cached_events(app_state: web::Data<AppState>) -> HttpResponse {
    let results: Vec<DatabaseResult> = sqlx::query_as!(
        DatabaseResult,
        "SELECT events.*, participants.user_id FROM events LEFT JOIN participants ON participants.event_id = events.id",
    ).fetch_all(&app_state.pool).await.unwrap();

    HttpResponse::Ok().json(convert_results_to_events(results))
}

fn convert_results_to_events(results: Vec<DatabaseResult>) -> Vec<Event> {
    let mut event_map: std::collections::HashMap<u32, Event> = std::collections::HashMap::new();

    for result in results {
        let event = event_map.entry(result.id).or_insert_with(|| Event {
            id: result.id,
            region_id: result.region_id,
            title: result.title,
            description: result.description,
            reward: result.reward,
            source: Source::from_str(&result.source),
            url: result.url,
            image_url: result.image_url,
            participants: vec![],
        });

        if let Some(user_id) = result.user_id {
            event.participants.push(ParticipantModel { user_id });
        }
    }

    event_map.into_values().collect()
}

#[post("/events/{event_id}/participate")]
pub async fn participate(path: web::Path<u32>, app_state: web::Data<AppState>, user: User) -> HttpResponse {
    let event_id: u32 = path.into_inner();

    let Ok(_): Result<_, sqlx::Error> = sqlx::query!(
        "SELECT * FROM events WHERE id=?",
        event_id
    ).fetch_one(&app_state.pool).await else {
        return HttpResponse::BadRequest().json(Response {
            message: "Event wasn\'t found.".to_string()
        });
    };

    let Ok(_created): Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        "INSERT INTO participants(event_id, user_id) VALUES(?, ?) ON DUPLICATE KEY UPDATE event_id=event_id",
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

    let Ok(_deleted): Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
        "DELETE FROM participants WHERE event_id = ? AND user_id = ?",
        event_id,
        user.id,
    ).execute(&app_state.pool).await else {
        return HttpResponse::InternalServerError().json(Response {
            message: "Unable to remove participation (or didn\'t exist).".to_string(),
        });
    };

    HttpResponse::Ok().json(Response {
        message: "Stopped participating to event.".to_string(),
    })
}
