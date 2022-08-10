use lazy_static::lazy_static;
use prometheus::{register_counter_vec, CounterVec};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

lazy_static! {
    pub static ref EVENT_CREATE_COUNTER: CounterVec =
        register_counter_vec!(
            "craete_event_actions", "number of create event",
            &["success", "path"]
        ).expect("create metric");
}

pub fn create_app_metrics() -> PrometheusMetrics {
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    // register metric types to registry
    prometheus.registry
        .register(Box::new(EVENT_CREATE_COUNTER.clone()))
        .expect("failed to register");
    prometheus.clone()
}