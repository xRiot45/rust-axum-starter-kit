use axum::http::HeaderValue;
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

/// Generates a unique UUID v4 per request and attaches it as `x-request-id`.
#[derive(Clone)]
pub struct UuidRequestId;

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(&mut self, _request: &axum::http::Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string();
        HeaderValue::from_str(&id).ok().map(RequestId::new)
    }
}
