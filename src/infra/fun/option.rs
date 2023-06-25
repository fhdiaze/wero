type Frt<'a, T, E> = dyn Fn(Result<T, E>) -> Result<T, E> + 'a;

pub fn map<'a, F, T, U>(f: &'a F) -> Box<dyn Fn(Option<T>) -> Option<U> + 'a>
where
  F: Fn(T) -> U,
{
  let fm = move |x: Option<T>| x.map(f);

  Box::new(fm)
}

pub fn bind<'a, F, T, U>(f: &'a F) -> Box<dyn Fn(Option<T>) -> Option<U> + 'a>
where
  F: Fn(T) -> Option<U>,
{
  let fm = move |x: Option<T>| x.and_then(f);

  Box::new(fm)
}

pub fn tee<F, T, E>(f: &F) -> Box<Frt<'_, T, E>>
where
  F: Fn(&T),
{
  let fm = move |x: Result<T, E>| {
    x.map(|t| {
      f(&t);
      t
    })
  };

  Box::new(fm)
}

pub fn tee_mut<F, T, E>(f: &F) -> Box<Frt<'_, T, E>>
where
  F: Fn(&mut T),
{
  let fm = move |x: Result<T, E>| {
    x.map(|mut t| {
      f(&mut t);
      t
    })
  };

  Box::new(fm)
}
