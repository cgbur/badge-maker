use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub const COLORS: [&str; 3] = ["brightgreen", "#c33", "#ff40b0"];
pub const LABELS: [&str; 1] = [r##"<>;&Hello!!&"'"##];
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
    .cartesian_product(LABELS.iter())
    .cartesian_product(STYLES.iter())
    .map(|((color, message), style)| NewBadge {
      label: message.to_string(),
      message: message.to_string(),
      color: color.to_string(),
      label_color: color.to_string(),
      style: style.to_string(),
    })
    .collect_vec()
}
