#![allow(unused_attributes)]

use actix_web::web;

pub mod routers;
pub mod datasources;
pub mod metrics;

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(routers::event::event_service_factory());
}