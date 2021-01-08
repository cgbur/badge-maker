use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CharDatum {
  pub low: u32,
  pub high: u32,
  pub width: f32,
}
