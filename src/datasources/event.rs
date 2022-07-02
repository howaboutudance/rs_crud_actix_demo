use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder,
    http::header::ContentType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RequestEvent {
    pub name: String,
    pub doc_type: String
}

#[derive(Serialize, Deserialize)]
pub struct ResponseEvent {
    pub id: Uuid,
    pub name: String,
    pub doc_type: String,
}

impl Responder for ResponseEvent {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}