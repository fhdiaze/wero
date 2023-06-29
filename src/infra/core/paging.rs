use axum::{
  response::{IntoResponse, Response},
  Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
  /// The items of the page
  pub items: Vec<T>,
  /// The page number: 0, 1, ...
  pub number: usize,
  /// The size of the page
  pub size: usize,
}

impl<T> Page<T> {
  /// Creates a new page with `items`
  ///
  /// # Arguments
  ///
  /// * `items` - The items of the page
  /// * `number` - The number of the page: 0, 1, ...
  pub fn new(items: Vec<T>, number: usize) -> Self {
    let size = items.len();
    Page {
      items,
      number,
      size,
    }
  }
}

impl<T: Serialize> IntoResponse for Page<T> {
  fn into_response(self) -> Response {
    Json(self).into_response()
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cursor<Query> {
  pub query: Option<Query>,
  #[serde(default = "default_page_number")]
  pub page_number: usize,
  #[serde(default = "default_page_size")]
  pub page_size: usize,
}

fn default_page_number() -> usize {
  0
}

fn default_page_size() -> usize {
  10
}
