use crate::infra::core::result::AppResult;

pub fn map_opt<'a, F, T, U>(
  f: &'a F,
) -> Box<dyn Fn(&'a Option<T>) -> Option<U> + 'a>
where
  F: Fn(&'a T) -> U,
{
  let fm = move |x: &'a Option<T>| x.as_ref().map(f);

  Box::new(fm)
}

pub fn flat_map_opt<'a, F, T, U>(
  f: &'a F,
) -> Box<dyn Fn(Option<T>) -> Option<U> + 'a>
where
  F: Fn(T) -> Option<U>,
{
  let fm = move |x: Option<T>| x.and_then(f);

  Box::new(fm)
}

pub fn map_result<'a, F, T, U>(
  f: &'a F,
) -> Box<dyn Fn(AppResult<T>) -> AppResult<U> + 'a>
where
  F: Fn(T) -> U,
{
  let fm = move |x: AppResult<T>| x.map(f);

  Box::new(fm)
}

pub fn flat_map_result<'a, F, T, U>(
  f: &'a F,
) -> Box<dyn Fn(AppResult<T>) -> AppResult<U> + 'a>
where
  F: Fn(T) -> AppResult<U>,
{
  let fm = move |x: AppResult<T>| x.and_then(f);

  Box::new(fm)
}
