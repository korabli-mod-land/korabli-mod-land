use std::net::IpAddr;

use axum::{
  Extension, RequestExt,
  body::Body,
  extract::Request,
  http::{Response, StatusCode},
  middleware::Next,
  response::IntoResponse,
};
use axum_client_ip::{InsecureClientIp, SecureClientIp};

#[derive(Debug, Clone)]
pub struct AllowedIp(pub Vec<IpAddr>);

#[derive(Debug, Clone, Copy)]
pub struct BehindReverseProxy;

pub async fn validate_client_ip(
  Extension(AllowedIp(ips)): Extension<AllowedIp>,
  InsecureClientIp(insecure_client_ip): InsecureClientIp,
  mut request: Request,
  next: Next,
) -> impl IntoResponse {
  let client_ip = if request.extensions().get::<BehindReverseProxy>().is_some() {
    let SecureClientIp(secure_client_ip) = request
      .extract_parts::<SecureClientIp>()
      .await
      .expect("wtf");
    secure_client_ip
  } else {
    insecure_client_ip
  };
  if ips.iter().all(|x| x != &client_ip) {
    return Response::builder()
      .status(StatusCode::FORBIDDEN)
      .body(Body::empty())
      .expect("huh?");
  }
  next.run(request).await
}
