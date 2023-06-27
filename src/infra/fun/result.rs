type Fr<'a, T, E, U> = dyn Fn(Result<T, E>) -> Result<U, E> + 'a;
type Frt<'a, T, E> = dyn Fn(Result<T, E>) -> Result<T, E> + 'a;
type Fs<T, U> = dyn Fn(T) -> U;
type Fot<T, U, E> = dyn Fn(T) -> Result<U, E>;

/// Adapts a function to receive and return a Result
pub fn map<T, E, U>(f: &Fs<T, U>) -> Box<Fr<'_, T, E, U>> {
  let fm = move |x: Result<T, E>| x.map(f);

  Box::new(fm)
}

pub fn bind<T, U, E>(f: &Fot<T, U, E>) -> Box<Fr<T, E, U>> {
  let fm = move |x: Result<T, E>| x.and_then(f);

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

pub fn mix<'a, Fs, Fe, T, U, E>(f: &'a Fs, c: &'a Fe) -> Box<Fr<'a, T, E, U>>
where
  Fs: Fn(T) -> U,
  Fe: Fn(E) -> E,
{
  let fm = move |x: Result<T, E>| x.map(f).map_err(c);

  Box::new(fm)
}
