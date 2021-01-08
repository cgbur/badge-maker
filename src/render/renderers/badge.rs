use crate::badge::Links;
use crate::render::renderers::{render_attributes, render_title};
use crate::render::util::xml::strip_xml_whitespace;

pub struct RenderBadgeConfig<'a> {
  pub left_width: usize,
  pub right_width: usize,
  pub height: usize,
  pub accessible_text: &'a str,
  pub links: &'a Links,
}

pub fn render_badge(config: RenderBadgeConfig, main: String) -> String {
  let width = config.left_width + config.right_width;

  let main = if let Some(left_link) = config.links.single() {
    format!(
      r#"<a target="_blank" xlink:href="{left_link}">{main}</a>"#,
      main = main,
      left_link = left_link
    )
  } else {
    main
  };

  let badge = format!(
    r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{width}" height="{height}" {attributes}>
      {title}{main}
      </svg>"#,
    width = width,
    height = config.height,
    attributes = render_attributes(config.links, config.accessible_text),
    title = render_title(config.links, config.accessible_text),
    main = main
  );

  strip_xml_whitespace(&badge)
}
