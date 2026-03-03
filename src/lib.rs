#![deny(clippy::all)]
mod types;
use crate::types::nodetype::Node;
use napi_derive::napi;

#[napi(object)]
pub struct RustiaOptions {
  pub nodes: Vec<Node>,
}

#[napi]
pub struct Rustia {
  nodes: Vec<Node>,
}

#[napi]
impl Rustia {
  /// Create a new Rustia instance.
  /// ### Example
  /// ```javascript
  /// const options = {
  ///   nodes: [
  ///     { name: "my-node", host: "127.0.0.1:2333", secure: false }
  ///   ]
  /// };
  /// const rustia = new Rustia(options);
  /// ```
  #[napi(constructor)]
  pub fn new(options: RustiaOptions) -> Self {
    Self {
      nodes: options.nodes,
    }
  }
  /// Returns information about registered nodes.
  #[napi]
  pub fn get_nodes(&self) -> Vec<Node> {
    self.nodes.clone()
  }
}
