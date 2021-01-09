use crate::badge::Links;
use crate::render::util::xml::escape_xml;

const RAW_STRING_LEN: usize = 30;

pub fn render_attributes(links: &Links, accessible_text: &str) -> String {
  if !links.any() {
    let escaped = escape_xml(accessible_text);
    let mut build = String::with_capacity(RAW_STRING_LEN + escaped.len());
    build.push_str(r##"role="img" aria-label=""##);
    build.push_str(&escaped);
    build.push('"');
    build
  } else {
    "".to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn render_attributes_test() {
    assert_eq!(
      render_attributes(
        &Links {
          left: None,
          right: None,
        },
        "access",
      ),
      r##"role="img" aria-label="access""##
    );

    assert_eq!(
      render_attributes(
        &Links {
          left: None,
          right: None,
        },
        r##"&'"<> he,lo"##,
      ),
      r##"role="img" aria-label="&amp;&apos;&quot;&lt;&gt; he,lo""##
    );
  }
}
