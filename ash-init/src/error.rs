// © ickk 2023-2026, All Rights Reserved.

/// Simple [`::core::result::Result`] type with error variant being a
/// `Box<dyn Error>`
pub type Error = Box<dyn ::std::error::Error>;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ErrorList(Box<[Box<dyn ::std::error::Error>]>);

impl ErrorList {
  pub fn iter(&self) -> impl Iterator<Item = &Box<dyn ::std::error::Error>> {
    self.0.iter()
  }
}

impl ::core::fmt::Display for ErrorList {
  fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
    for e in self.0.iter() {
      f.write_fmt(format_args!("{e}, "))?
    }
    Ok(())
  }
}

impl<It, E> From<It> for ErrorList
where
  It: IntoIterator<Item = E>,
  E: Into<Box<dyn ::std::error::Error>>,
{
  fn from(it: It) -> Self {
    let v: Vec<Box<dyn ::std::error::Error>> =
      it.into_iter().map(|e| e.into()).collect();
    ErrorList(v.into_boxed_slice())
  }
}

impl ::std::error::Error for ErrorList {}
