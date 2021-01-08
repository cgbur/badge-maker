use crate::render::util::xml::escape_xml;

pub struct RenderLinkConfig<'a> {
  pub link: &'a str,
  pub height: usize,
  pub text_length: usize,
  pub horizontal_padding: usize,
  pub left_margin: isize,
  pub rendered_text: String,
}

pub fn render_link(config: RenderLinkConfig) -> String {
  let rect_height = config.height;
  let rect_width = config.text_length + config.horizontal_padding * 2;
  let rect_x = if config.left_margin > 1 {
    config.left_margin + 1
  } else {
    0
  };

  format!(
    r#"<a target="_blank" xlink:href="{escaped_link}">
      <rect width="{rect_width}" x="{rect_x}" height="{rect_height}" fill="rgba(0,0,0,0)" />
      {rendered_text}
    </a>"#,
    escaped_link = escape_xml(config.link),
    rect_width = rect_width,
    rect_x = rect_x,
    rect_height = rect_height,
    rendered_text = config.rendered_text
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn render_link_test() {
    assert_eq!(
      render_link(RenderLinkConfig {
        link: "some>link",
        height: 212,
        text_length: 123,
        horizontal_padding: 13,
        left_margin: 15,
        rendered_text: "rendered_text".to_string()
      }),
      "<a target=\"_blank\" xlink:href=\"some&gt;link\">\n      <rect width=\"149\" x=\"16\" height=\"212\" fill=\"rgba(0,0,0,0)\" />\n      rendered_text\n    </a>"
    );
  }
}
