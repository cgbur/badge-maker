#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Logo {
  pub(crate) url: String,
  pub(crate) width: usize,
  pub(crate) padding: isize,
}

impl Logo {
  pub fn new(url: String, width: usize, padding: isize) -> Self {
    Self {
      url,
      width,
      padding,
    }
  }

  pub fn url(&self) -> &str {
    &self.url
  }
  pub fn width(&self) -> usize {
    self.width
  }
  pub fn padding(&self) -> isize {
    self.padding
  }
}
