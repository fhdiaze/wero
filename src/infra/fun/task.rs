#![allow(dead_code)]
type Action<T, U> = dyn Fn(Option<T>) -> U;

pub struct Task<T, U> {
  action: Action<T, U>,
}

impl<T, U> Task<T, U> {
  fn run() {}
}
