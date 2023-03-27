use axum::{
  response::{IntoResponse, Response},
  Json,
};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Page<T> {
  items: Vec<T>,
  page: usize,
  size: usize,
  total: usize,
}

impl<T> Page<T> {
  /// Creates a new page with `items`
  ///
  /// # Arguments
  ///
  /// * `items` - The items of the page
  /// * `page` - The index of the page
  /// * `size` - The size of the page
  /// * `total` - The total items
  pub fn new(items: Vec<T>, page: usize, size: usize, total: usize) -> Self {
    Page {
      items,
      page,
      size,
      total,
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
  pub continuation_token: Option<ObjectId>,
  #[serde(default = "default_size")]
  pub size: i64,
}

fn default_size() -> i64 {
  10
}
