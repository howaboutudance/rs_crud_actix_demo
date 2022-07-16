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
            .json(body)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use super::*;
    use uuid::Uuid;

    fn setup(expected_uuid: Uuid) -> ResponseEvent {

        let res = ResponseEvent{id: expected_uuid, name: "foo".to_string(), doc_type: "bar".to_string()};
        res
    }

    #[test]
    fn test_response_event_new(){
        let expected_uuid = Uuid::new_v4();
        let res = setup(expected_uuid);

        assert_eq!(res.id, expected_uuid);
        assert_eq!(res.name, "foo");
        assert_eq!(res.doc_type, "bar");
    }

   #[test]
    fn test_reponse_event_json_deserialize(){
       let expected_uuid = Uuid::new_v4();
       let res = setup(expected_uuid);
       assert_eq!(json!(res).to_string(),
                  format!("{{\"doc_type\":\"bar\",\"id\":\"{}\",\"name\":\"foo\"}}", expected_uuid.to_string()));
   }
}