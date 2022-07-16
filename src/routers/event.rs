use actix_web::{get, post, Responder, Result, Scope, web};
use uuid::Uuid;

use crate::datasources::event::{RequestEvent, ResponseEvent};
use crate::metrics::EVENT_CREATE_COUNTER;


#[get("/")]
async fn get_events() -> Result<impl Responder> {
    Ok(
        web::Json(
            ResponseEvent {
                id: Uuid::new_v4(),
                name: "Flammable".to_string(),
                doc_type: "Calendar".to_string(),
            }
        ))
}

#[post("/")]
async fn create_event(event: web::Json<RequestEvent>) -> impl Responder {
    EVENT_CREATE_COUNTER.with_label_values(&["true", "/event/"]).inc();
    ResponseEvent {
        id: Uuid::new_v4(),
        name: event.name.clone(),
        doc_type: event.doc_type.clone(),
    }
}

pub fn event_service_factory() -> Scope {
    web::scope("/event")
        .service(get_events)
        .service(create_event)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use actix_web::{App, HttpResponse, test, web, body};
    use actix_web::http::Method;
    use super::*;

    #[actix_web::test]
    async fn test_get_event_success() {
        let app = test::init_service(App::new()
            .service(
                web::scope("/event")
                    .service(get_events)
            )).await;
        let req = test::TestRequest::get().uri("/event/").to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("content-type").unwrap(), "application/json")
    }

    #[actix_web::test]
    async fn test_post_event__no_payload() {
        let app = test::init_service(App::new()
            .service(
                web::scope("/event")
                    .service(create_event)
            )).await;
        let req = test::TestRequest::post().uri("/event/")
            .insert_header(("content-type", "application/json"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        assert!(resp.status().as_u16() == 400);
        // TODO: implement json error responses to fix
        assert_eq!(resp.headers().get("content-type").unwrap(), "application/json")
    }

    #[actix_web::test]
    async fn test_post_event__with_payload() {
        let app = test::init_service(App::new()
            .service(
                web::scope("/event")
                    .service(create_event)
            )).await;
        let req = test::TestRequest::default()
            .method(Method::POST)
            .uri("/event/")
            .set_json(RequestEvent { name: "foo".to_string(), doc_type: "event".to_string() })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("content-type").unwrap(), "application/json");

        let resp_http = HttpResponse::from(resp).into_body();
        let resp_bytes = body::to_bytes(resp_http).await.unwrap();
        assert!(resp_bytes.len() > 0);
    }

    //TODO: implement test for counter
}
