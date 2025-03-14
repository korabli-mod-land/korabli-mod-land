#[utoipa::path(get, path = "/hello", operation_id = "v1_hello", tag = "utils", responses(
  (status = StatusCode::OK, description = "Hello")
))]
pub(super) async fn hello() -> Result<(), ()> {
  Ok(())
}
