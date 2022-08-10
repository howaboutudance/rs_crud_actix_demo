use std::str::from_utf8;
use rs_crud_actix::{app_config, metrics};
use actix_web::{App, test};
use actix_web_json_error_middleware::JsonMiddleware;

/// check that the nname of a metric is included in the metrics endpoint response
#[actix_web::test]
async fn test_intg_metrics_label_substring() {
    let  promethesus_metrics = metrics::create_app_metrics();
    let app = test::init_service(
        App::new()
            .wrap(JsonMiddleware)
            .wrap(promethesus_metrics)
            .configure(app_config)
    ).await;

    for _times in 0..10 {
        let req = test::TestRequest::get().uri("/docs/").to_request();
        test::call_service(&app, req).await;
    }

    let req = test::TestRequest::get().uri("/metrics").to_request();
    let resp= test::call_and_read_body(&app, req).await.to_vec();
    let resp_str = from_utf8(&resp).unwrap().to_string();

    assert!(resp_str.contains("api_http_requests_duration_seconds_bucket"))
}

/// check that the content_type contains text/plain
#[actix_web::test]
async fn test_intg_content_type(){
    let  promethesus_metrics = metrics::create_app_metrics();
    let app = test::init_service(
        App::new()
            .wrap(JsonMiddleware)
            .wrap(promethesus_metrics)
            .configure(app_config)
    ).await;

    for _times in 0..10 {
    let req = test::TestRequest::get().uri("/docs/").to_request();
    test::call_service(&app, req).await;
    }

    let req = test::TestRequest::get().uri("/metrics").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.headers().get("content-type").unwrap(),
               "text/plain; version=0.0.4; charset=utf-8");

}
