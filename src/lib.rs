#![deny(clippy::all)]
mod error;
mod node;
mod player;
mod rest;
mod types;
mod ws;
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
  ///     { name: "my-node", host: "127.0.0.1:2333", secure: false, is_nodelink: true, session_id: "sid" }
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
  /// ### Example
  /// ```javascript
  /// [
  ///   { name: 'test', host: '127.0.0.1:3000', secure: true }
  /// ]
  /// ```
  #[napi]
  pub fn get_nodes(&self) -> Vec<Node> {
    self.nodes.clone()
  }
}
