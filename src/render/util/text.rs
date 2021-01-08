use crate::badge::color::util::brightness;
use crate::render::font::Font;
use crate::render::util::round::round_up_to_odd;
use crate::render::BRIGHTNESS_THRESHOLD;

pub struct TextColoring {
  pub text: &'static str,
  pub shadow: &'static str,
}

pub fn colors_for_background(color: &str) -> TextColoring {
  if brightness(color) <= BRIGHTNESS_THRESHOLD {
    TextColoring {
      text: "#fff",
      shadow: "#010101",
    }
  } else {
    TextColoring {
      text: "#333",
      shadow: "#ccc",
    }
  }
}

pub fn preferred_width(text: &str, font: &Font) -> usize {
  round_up_to_odd(font.width_of_str(text) as u32) as usize
}

pub fn create_accessible_text(label: &Option<String>, message: &str) -> String {
  if let Some(label) = label {
    format!("{}: {}", label, message)
  } else {
    message.to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_accessible_text_test() {
    assert_eq!(
      create_accessible_text(&Some("hello".to_string()), "message"),
      "hello: message".to_string()
    );

    assert_eq!(
      create_accessible_text(&None, "message"),
      "message".to_string()
    );
  }
}
