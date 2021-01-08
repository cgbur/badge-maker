use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use badge_maker::Links;
use lazy_static::lazy_static;

const XML_ESCAPE_PATTERNS: [&str; 5] = ["&", "<", ">", "\"", "'"];
const XML_ESCAPE_REPLACEMENTS: [&str; 5] = ["&amp;", "&lt;", "&gt;", "&quot;", "&apos;"];

pub fn escape_xml(s: &str) -> String {
  lazy_static! {
    static ref AC: AhoCorasick = AhoCorasickBuilder::new()
      .dfa(true)
      .build(&XML_ESCAPE_PATTERNS);
  };

  AC.replace_all(s, &XML_ESCAPE_REPLACEMENTS)
}

pub fn render_attributes_string(links: &Links, accessible_text: &str) -> String {
  if !links.any() {
    let escaped = escape_xml(accessible_text);
    let mut build = String::with_capacity(50 + escaped.len());
    build.push_str(r##"role="img" aria-label=""##);
    build.push_str(&escaped);
    build.push('"');
    build
  } else {
    "".to_string()
  }
}

pub fn render_attributes_format(links: &Links, accessible_text: &str) -> String {
  if links.any() {
    "".to_string()
  } else {
    format!(
      r##"role="img" aria-label="{}""##,
      escape_xml(accessible_text)
    )
  }
}
