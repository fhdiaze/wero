#![allow(dead_code)]

type Fo<'a, T, U> = dyn Fn(Option<T>) -> Option<U> + 'a;
type Fs<T, U> = dyn Fn(T) -> U;

pub fn map<T, U>(f: &Fs<T, U>) -> Box<Fo<'_, T, U>> {
  let fm = move |x: Option<T>| x.map(f);

  Box::new(fm)
}

pub fn bind<F, T, U>(f: &F) -> Box<Fo<'_, T, U>>
where
  F: Fn(T) -> Option<U>,
{
  let fm = move |x: Option<T>| x.and_then(f);

  Box::new(fm)
}

pub fn tee<F, T>(f: &F) -> Box<Fo<'_, T, T>>
where
  F: Fn(&T),
{
  let fm = move |x: Option<T>| {
    x.map(|t| {
      f(&t);
      t
    })
  };

  Box::new(fm)
}
