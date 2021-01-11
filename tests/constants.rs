use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub const COLORS: [&str; 3] = ["brightgreen", "#c33", "#ff40b0"];
pub const LABELS: [&str; 3] = [
  r##"<>;&Hello!!&"'"##,
  "This is a super long label testing some other things like compounding error possibilities",
  "💀💀emoji💀💀test💀💀",
];
pub const STYLES: [&str; 3] = ["flat", "plastic", "flatsquare"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBadge {
  pub label: String,
  pub message: String,
  pub color: String,
  pub label_color: String,
  pub style: String,
}
pub fn get_badges() -> Vec<NewBadge> {
  COLORS
    .iter()
    .cartesian_product(COLORS.iter())
    .cartesian_product(LABELS.iter())
    .cartesian_product(LABELS.iter())
    .cartesian_product(STYLES.iter())
    .map(|((((c1, c2), label), message), style)| NewBadge {
      label: label.to_string(),
      message: message.to_string(),
      color: c1.to_string(),
      label_color: c2.to_string(),
      style: style.to_string(),
    })
    .collect_vec()
}
