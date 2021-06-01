use crate::badge::style::Style::*;
use crate::error::Error;
use crate::error::Error::BadStyleChoice;
use std::fmt::Formatter;
use std::fmt::Display;

/// Used to define the style of a badge. Used in [BadgeBuilder.style()](crate::badge::BadgeBuilder)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Style {
  Flat,
  Plastic,
  FlatSquare,
  // ForTheBadge,
  // Social,
}

pub fn parse_style(s: &str) -> Result<Style, Error> {
  match s {
    "flat" => Ok(Flat),
    "plastic" => Ok(Plastic),
    "flatsquare" => Ok(FlatSquare),
    // "forthebadge" => Ok(ForTheBadge),
    // "social" => Ok(Social),
    choice => Err(BadStyleChoice(choice.to_string())),
  }
}

impl Display for Style {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Flat => "flat",
        Plastic => "plastic",
        FlatSquare => "flatsquare",
        // ForTheBadge => "forthebadge",
        // Social => "social",
      }
    )
  }
}
