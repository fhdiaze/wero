use crate::infra::core::result::AppResult;

pub fn map_opt<'a, F, T, U>(
  f: &'a F,
) -> Box<dyn Fn(Option<T>) -> Option<U> + 'a>
where
  F: Fn(T) -> U,
{
  let fm = move |x: Option<T>| x.map(f);

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

pub fn bind_opt<'a, F, T, U>(
  f: &'a F,
) -> Box<dyn Fn(Option<T>) -> Option<U> + 'a>
where
  F: Fn(T) -> Option<U>,
{
  let fm = move |x: Option<T>| x.and_then(f);

  Box::new(fm)
}

pub fn bind_result<'a, F, T, U>(
  f: &'a F,
) -> Box<dyn Fn(AppResult<T>) -> AppResult<U> + 'a>
where
  F: Fn(T) -> AppResult<U>,
{
  let fm = move |x: AppResult<T>| x.and_then(f);

  Box::new(fm)
}

pub fn tee<'a, F, T>(f: &'a F) -> Box<dyn Fn(AppResult<T>) -> AppResult<T> + 'a>
where
  F: Fn(&T),
{
  let fm = move |x: AppResult<T>| {
    x.map(|t| {
      f(&t);
      t
    })
  };

  Box::new(fm)
}

pub fn tee_mut<'a, F, T>(
  f: &'a F,
) -> Box<dyn Fn(AppResult<T>) -> AppResult<T> + 'a>
where
  F: Fn(&mut T),
{
  let fm = move |x: AppResult<T>| {
    x.map(|mut t| {
      f(&mut t);
      t
    })
  };

  Box::new(fm)
}

pub fn mix<'a, F, C, T, U, E, X>(
  f: &'a F,
  c: &'a C,
) -> Box<dyn Fn(Result<T, E>) -> Result<U, X> + 'a>
where
  F: Fn(T) -> U,
  C: Fn(E) -> X,
{
  let fm = move |x: Result<T, E>| x.map(f).map_err(c);

  Box::new(fm)
}
