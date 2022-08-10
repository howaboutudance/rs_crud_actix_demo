use rs_crud_actix::routers::event::{create_event, get_events};
use rs_crud_actix::datasources::event::{RequestEvent};
use actix_web::{App, web, test, HttpResponse, http::{Method, header::ACCEPT}, body};

#[actix_web::test]
async fn test_get_event_success() {
    let app = test::init_service(App::new()
        .service(
            web::scope("/event")
                .service(get_events)
        )
    ).await;

    let req = test::TestRequest::get().uri("/event/")
        .insert_header((ACCEPT, "application/json"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(resp.headers().get("content-type").unwrap(), "application/json")
}


// #[actix_web::test]
async fn _test_get_event_accept_html_no() {
    let app = test::init_service(App::new()
        .service(
            web::scope("/event")
                .service(get_events)
        )
    ).await;

    let req = test::TestRequest::get().uri("/event/")
        .insert_header((ACCEPT, "text/html"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[allow(non_snake_case)]
#[actix_web::test]
async fn test_post_event__no_payload() {
    let app = test::init_service(App::new()
        .service(
            web::scope("/event")
                .service(create_event)
        )).await;
    let req = test::TestRequest::post().uri("/event/")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
    assert_eq!(resp.status().as_u16(), 400);
}

#[allow(non_snake_case)]
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

    let resp_http = HttpResponse::from(resp).into_body();
    let resp_bytes = body::to_bytes(resp_http).await.unwrap();
    assert!(resp_bytes.len() > 0);
}
