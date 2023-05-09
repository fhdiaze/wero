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
  pub page: usize,
  /// The size of the page
  pub size: usize,
}

impl<T> Page<T> {
  /// Creates a new page with `items`
  ///
  /// # Arguments
  ///
  /// * `items` - The items of the page
  /// * `page` - The number of the page: 0, 1, ...
  /// * `size` - The size of the page
  pub fn new(items: Vec<T>, page: usize, size: usize) -> Self {
    Page { items, page, size }
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
  #[serde(default = "default_page")]
  pub page: usize,
  #[serde(default = "default_size")]
  pub size: usize,
}

fn default_page() -> usize {
  0
}

fn default_size() -> usize {
  10
}
