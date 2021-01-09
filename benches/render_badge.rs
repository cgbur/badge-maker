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

#[derive(Copy, Clone)]
pub struct RenderBadgeConfig<'a> {
  pub left_width: usize,
  pub right_width: usize,
  pub height: usize,
  pub accessible_text: &'a str,
  pub links: &'a Links,
}

pub fn render_badge_old(config: RenderBadgeConfig, main: &str) -> String {
  let width = config.left_width + config.right_width;

  let main = if let Some(left_link) = config.links.single() {
    format!(
      r#"<a target="_blank" xlink:href="{left_link}">{main}</a>"#,
      main = main,
      left_link = left_link
    )
  } else {
    main.to_string()
  };

  let badge = format!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{width}" height="{height}" {attributes}>
      {title}{main}
      </svg>"#,
    width = width,
    height = config.height,
    attributes = "attributes",
    title = "title",
    main = main
  );

  badge
}

pub fn render_badge_new(config: RenderBadgeConfig, main: &str) -> String {
  let width = config.left_width + config.right_width;
  let link = config.links.single().unwrap_or("");
  let link_len = if !link.is_empty() { 60 + link.len() } else { 0 };

  let attributes = "render_attributes(config.links, config.accessible_text)";
  let title = "render_title(config.links, config.accessible_text)";

  let mut buffer =
    String::with_capacity(140 + link_len + main.len() + attributes.len() + title.len());

  buffer.push_str(r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width=""##);
  itoa::fmt(&mut buffer, width);
  buffer.push_str(r#"" height=""#);
  itoa::fmt(&mut buffer, config.height);
  buffer.push_str(r#"" "#);

  buffer.push_str(attributes);
  buffer.push('>');
  buffer.push_str(title);

  if link_len != 0 {
    buffer.push_str(r##"<a target="_blank" xlink:href=""##);
    buffer.push_str(link);
    buffer.push_str(r#"">"#);
    buffer.push_str(main);
    buffer.push_str(r#"</a>"#);
  } else {
    buffer.push_str(main);
  };

  buffer.push_str(r#"</svg>"#);

  buffer
}
