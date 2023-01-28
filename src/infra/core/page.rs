use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

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
