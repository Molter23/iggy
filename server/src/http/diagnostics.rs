use crate::http::shared::RequestDetails;
use crate::streaming::utils::random_id;
use axum::{
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;
use tokio::time::Instant;
use tracing::debug;

pub async fn request_diagnostics<T>(
    ConnectInfo(ip_address): ConnectInfo<SocketAddr>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    let request_id = random_id::get_ulid();
    let path_and_query = request
        .uri()
        .path_and_query()
        .map(|p| p.as_str())
        .unwrap_or("/");
    debug!(
        "Processing a request {} {} with ID: {request_id} from client with IP address: {ip_address}...",
        request.method(),
        path_and_query,
    );
    request.extensions_mut().insert(RequestDetails {
        request_id,
        ip_address,
    });
    let now = Instant::now();
    let result = Ok(next.run(request).await);
    let elapsed = now.elapsed();
    debug!(
        "Processed a request with ID: {request_id} from client with IP address: {ip_address} in {} ms.",
        elapsed.as_millis()
    );
    result
}
