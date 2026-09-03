use axum::{Router, http::{HeaderName, StatusCode}, response::IntoResponse, routing::get};
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};

use crate::AppState;

const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn init(state: AppState) -> Router {
    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(x_request_id.clone(), MakeRequestUuid));
}















async fn livez() -> impl IntoResponse {
    StatusCode::OK
}
