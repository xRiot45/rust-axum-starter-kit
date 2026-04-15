use axum::Router;
use axum_prometheus::PrometheusMetricLayer;

/// Attaches a Prometheus metrics layer to the Axum router.
/// Exposes /metrics endpoint for scraping by Prometheus / Grafana.
pub fn attach_metrics(router: Router) -> (Router, axum_prometheus::metrics_exporter_prometheus::PrometheusHandle) {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let router = router.layer(prometheus_layer);
    (router, metric_handle)
}
