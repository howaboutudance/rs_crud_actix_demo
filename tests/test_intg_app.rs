use rs_crud_actix::app_config;
use actix_web::{App, test};
use actix_web_json_error_middleware::JsonMiddleware;

#[actix_web::test]
async fn test_get_404_json(){
    let app = test::init_service(
        App::new()
            .wrap(JsonMiddleware)
            .configure(app_config)
    ).await;

    let req = test::TestRequest::get().uri("/4o4").to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_client_error());
    assert_eq!(resp.headers().get("content-type").unwrap(), "application/json")
}