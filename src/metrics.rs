use prometheus::{Encoder, TextEncoder, IntCounter, IntCounterVec, Registry, opts};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref HTTP_COUNTER: IntCounter = IntCounter::new("http_requests_total", "Total number of HTTP requests made.").unwrap();
    pub static ref HTTP_COUNTER_VEC: IntCounterVec = IntCounterVec::new(
        opts!("http_requests_total_vec", "Total number of HTTP requests made grouped by status code and method."),
        &["status_code", "method"]
    ).unwrap();
}

pub fn register_metrics() {
    REGISTRY.register(Box::new(HTTP_COUNTER.clone())).unwrap();
    REGISTRY.register(Box::new(HTTP_COUNTER_VEC.clone())).unwrap();
}

pub async fn metrics_handler() -> impl actix_web::Responder {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    actix_web::HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}
