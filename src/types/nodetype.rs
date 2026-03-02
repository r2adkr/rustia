use napi_derive::napi;

/// It's a structure that manages node connections.
/// ### Exapmle
/// ```
/// {
///     name: "node-name",
///     host: "myserver:2333",
///     secure: true,
///     session_id: "session_id",
/// }
/// ```
#[napi(object)]
pub struct Node {
  /// Node name
  pub name: String,
  /// Server address (must include the port)
  pub host: String,
  /// Whether the server uses a secure connection
  pub secure: bool,
  /// Option: Session ID
  pub session_id: Option<String>,
}
