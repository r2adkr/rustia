use napi_derive::napi;

/// It's a structure that manages node connections.
/// ### Example
/// ```
/// {
///     name: "node-name",
///     host: "myserver:2333",
///     secure: true,
///     is_nodelink: true,
///     session_id: "session_id",
/// }
/// ```
#[derive(Clone)]
#[napi(object)]
pub struct Node {
  /// Node name
  pub name: String,
  /// Server address (must include the port)
  pub host: String,
  /// Whether the server uses a secure connection
  pub secure: bool,
  /// If you are using NodeLink, set this option to true.
  pub is_nodelink: Option<bool>,
  /// Option: Session ID
  pub session_id: Option<String>,
}
