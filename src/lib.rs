#![deny(clippy::all)]
mod types;
use napi_derive::napi;

use crate::types::nodetype::Node;

#[napi]
pub struct Rustia {
  pub nodes: Vec<Node>,
}
