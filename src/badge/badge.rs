use crate::badge::color::util::u64_to_hex;
use crate::badge::style::Style;
use crate::badge::{Links, Logo};

use crate::render::renderers::styles::*;
use crate::render::BadgeRenderer;
use seahash::hash;

/// The purpose of a unique id is to make the svg compatible with direct website embedding.
///
/// Hashing is done as a escaping mechanism.
fn gen_id(
  label: &Option<String>,
  message: &str,
  label_color: &str,
  color: &str,
  style: &Style,
) -> String {
  let buf = vec![
    label.as_ref().unwrap_or(&"not_set".to_string()).as_bytes(),
    message.as_bytes(),
    label_color.as_bytes(),
    color.as_bytes(),
    style.to_string().as_bytes(),
  ]
  .iter()
  .cloned()
  .flatten()
  .cloned()
  .collect::<Vec<u8>>();
  u64_to_hex(hash(&buf))
}

/// Badges are valid and have all the necessary
/// fields to construct an SVG without error.
/// Use the [BadgeBuilder](crate::badge::BadgeBuilder) to construct.
///
/// # Example
///
/// ```rust
/// use badge_maker::BadgeBuilder;
/// # use badge_maker::error::Error;
///
/// let badge = BadgeBuilder::new()
///   .message("Example")
///   .build()?;
///
/// # Ok::<(), Error>(())
/// ```
///
#[derive(Debug, Eq, PartialEq)]
pub struct Badge {
  label: Option<String>,
  message: String,
  label_color: String,
  color: String,
  style: Style,
  links: Links,
  logo: Option<Logo>,
  id: String,
  ids: String,
  idr: String,
}

impl Badge {
  pub(crate) fn new(
    label: Option<String>,
    message: String,
    label_color: String,
    color: String,
    style: Style,
    links: Links,
    logo: Option<Logo>,
  ) -> Self {
    let id = gen_id(&label, &message, &label_color, &color, &style);
    let ids = format!("bms-{}", id);
    let idr = format!("bmr-{}", id);

    Self {
      label,
      message,
      label_color,
      color,
      style,
      links,
      logo,
      id,
      ids,
      idr,
    }
  }

  pub fn svg(&self) -> String {
    match self.style {
      Style::Flat => Flat::render(self),
      Style::Plastic => Plastic::render(self),
      Style::FlatSquare => FlatSquare::render(self),
    }
  }

  pub fn label(&self) -> &Option<String> {
    &self.label
  }

  pub fn message(&self) -> &str {
    &self.message
  }

  pub fn label_color(&self) -> &str {
    &self.label_color
  }

  pub fn color(&self) -> &str {
    &self.color
  }

  pub fn style(&self) -> Style {
    self.style
  }

  pub fn links(&self) -> &Links {
    &self.links
  }

  pub fn logo(&self) -> &Option<Logo> {
    &self.logo
  }

  pub fn ids(&self) -> &str {
    &self.ids
  }

  pub fn idr(&self) -> &str {
    &self.idr
  }
  pub fn id(&self) -> &str {
    &self.id
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::badge::style::Style::Flat;

  #[test]
  fn gen_id_test() {
    assert_eq!(
      gen_id(&Some("hello".to_string()), "e", "e", "3", &Flat),
      "bf81a730957fd01d"
    );
    assert_eq!(
      gen_id(
        &Some("hello".to_string()),
        "message",
        "label_color",
        "#ec3d",
        &Flat
      ),
      "58129eed803f3f02"
    );
  }
}
