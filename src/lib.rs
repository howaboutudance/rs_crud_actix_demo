#![allow(unused_attributes)]

use actix_web::{web, dev, http::header, Result,
                middleware::{ErrorHandlers, ErrorHandlerResponse}};
pub mod routers;
pub mod datasources;
pub mod metrics;

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(routers::event::event_service_factory());
}

pub fn add_json_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"),
    );
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use actix_web::http::StatusCode;
    use actix_web::middleware::ErrorHandlers;
    use super::*;

    #[actix_web::test]
    async fn test_get_404_json(){
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, add_json_error_header))
                .wrap(ErrorHandlers::new().handler(StatusCode::BAD_REQUEST, add_json_error_header))
                .configure(app_config)
        ).await;

        let req = test::TestRequest::get().uri("/4o4").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());
        assert_eq!(resp.headers().get("content-type").unwrap(), "application/json")
    }
}