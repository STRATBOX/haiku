// dependencies
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use chrono::{Utc};
use ulid::Ulid;

// module declarations
use crate::models::{NewPerson, Packet};
use crate::AppState;

pub async fn ping(_req: HttpRequest) -> impl Responder {
    web::Json(Packet {
        // id: 12345.to_string(),
        id: Ulid::new().to_string().to_lowercase(),
        service: String::from("haiku"),
        message: String::from("running..."),
        created_on: Utc::now().timestamp_millis()
    })
}

pub async fn signup(
    state: web::Data<AppState>,
    person: web::Json<NewPerson>
) -> impl Responder {

    let result = web::block(move || 
        state.services.users.create(person.into_inner())
    ).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}