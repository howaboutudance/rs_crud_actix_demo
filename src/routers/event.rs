use actix_web::{web, get, post, Responder, Result, Scope};
use uuid::Uuid;
use crate::datasources::event::{ResponseEvent, RequestEvent};


#[get("/")]
async fn get_events() -> Result<impl Responder> {
    Ok(
        web::Json(
            ResponseEvent {
                id: Uuid::new_v4(),
                name: "Flammable".to_string(),
                doc_type: "Calendar".to_string()
            }
    ))
}

#[post("/")]
async fn create_event(event: web::Json<RequestEvent>) -> impl Responder {
    ResponseEvent {
        id: Uuid::new_v4(),
        name: event.name.clone(),
        doc_type: event.doc_type.clone(),
    }
}

pub fn event_service_factory() -> Scope {
    web::scope( "/event")
        .service(get_events)
        .service(create_event)
}
