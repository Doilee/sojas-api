use actix_web::{delete, get, HttpResponse, post, put, web};
use actix_web::web::Data;
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryResult;
use crate::{AppState, DataResponse, Response};

#[derive(Serialize, Deserialize, Copy, Clone)]
struct Participant {
    user_id: u32,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
struct Event {
    id: u32,
    participants: Vec<Participant>
}

#[derive(Serialize, Deserialize, FromRow)]
struct DatabaseResult {
    id: u32,
    user_id: u32,
}

fn convert_results_to_events(results: Vec<DatabaseResult>) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    for result in results {
        let mut event: Option<&mut Event> = events.iter_mut().find(|e| e.id == result.id);

        if let Some(event) = event {
            event.participants.push(Participant { user_id: result.user_id });
        } else {
            events.push(Event {
                id: result.id,
                participants: vec![Participant { user_id: result.user_id }],
            });
        }
    }

    events
}

#[get("/events")]
pub async fn index(app_state: web::Data<AppState>) -> HttpResponse {
    let results: Vec<DatabaseResult> = sqlx::query_as!(
        DatabaseResult,
        "SELECT id, user_id FROM events INNER JOIN participants ON participants.event_id = events.id",
    ).fetch_all(&app_state.pool).await.unwrap();

    HttpResponse::Ok().json(DataResponse {
        data: convert_results_to_events(results),
        message: Option::from("Got all events.".to_string()),
    })
}


// #[post("/participate/{event}")]
// pub async fn create(body: web::Json<CreateUserBody>, app_state: web::Data<AppState>) -> HttpResponse {
//     let created: Result<MySqlQueryResult, sqlx::Error> = sqlx::query!(
//         "INSERT INTO events(id, soy_balance, is_admin) VALUES(?, ?, ?)",
//         body.id,
//         body.soy_balance,
//         body.is_admin,
//     ).execute(&app_state.pool).await;
// 
//     if created.is_err() {
//         println!("{}", created.unwrap_err());
//         return HttpResponse::InternalServerError().json(Response {
//             message: "Couldn't create a new user.".to_string(),
//         });
//     }
// 
//     HttpResponse::Ok().json(Response {
//         message: "Created a user.".to_string(),
//     })
// }
