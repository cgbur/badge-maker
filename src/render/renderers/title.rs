use crate::badge::Links;
use crate::render::util::xml::escape_xml;

pub fn render_title(links: &Links, accessible_text: &str) -> String {
  if links.any() {
    "".to_string()
  } else {
    format!("<title>{}</title>", escape_xml(accessible_text))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn render_title_test() {
    assert_eq!(
      render_title(
        &Links {
          left: None,
          right: None
        },
        "hel&lo!"
      ),
      "<title>hel&amp;lo!</title>"
    );

    assert_eq!(
      render_title(
        &Links {
          left: Some("link".to_string()),
          right: None
        },
        "hel&lo!"
      ),
      ""
    );
  }
}
