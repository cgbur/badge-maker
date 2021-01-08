use crate::render::font::Font;

use crate::render::renderers::{render_link, RenderLinkConfig};
use crate::render::util::text::{colors_for_background, preferred_width, TextColoring};
use crate::render::util::xml::escape_xml;

pub struct RenderTextConfig<'a> {
  pub left_margin: isize,
  pub horizontal_padding: usize,
  pub content: &'a str,
  pub height: usize,
  pub vertical_margin: isize,
  pub shadow: bool,
  pub color: &'a str,
  pub link: &'a Option<String>,
  pub font: &'a Font,
}

pub struct RenderTextReturn {
  pub text: String,
  pub width: usize,
}

pub fn render_text(config: RenderTextConfig) -> RenderTextReturn {
  if config.content.is_empty() {
    return RenderTextReturn {
      text: "".to_string(),
      width: 0,
    };
  }

  let text_length = preferred_width(&config.content, config.font);
  let escaped_content = escape_xml(&config.content);

  let shadow_margin = 150 + config.vertical_margin;
  let text_margin = 140 + config.vertical_margin;
  let out_text_length = 10 * text_length;

  let x = 10.0
    * (config.left_margin as f32 + 0.5 * text_length as f32 + config.horizontal_padding as f32);

  let TextColoring { text, shadow } = colors_for_background(&config.color);

  let shadow_text = if config.shadow {
    format!(
      r#"<text aria-hidden="true" x="{x}" y="{shadow_margin}" fill="{shadow_color}" fill-opacity=".3" transform="scale(.1)" textLength="{out_text_length}">{escaped_content}</text>"#,
      x = x,
      shadow_margin = shadow_margin,
      shadow_color = shadow,
      out_text_length = out_text_length,
      escaped_content = escaped_content
    )
  } else {
    "".to_string()
  };
  let rendered_text = format!(
    r#"{shadow}<text x="{x}" y="{text_margin}" transform="scale(.1)" fill="{text_color}" textLength="{out_text_length}">{escaped_content}</text>"#,
    escaped_content = escaped_content,
    out_text_length = out_text_length,
    x = x,
    text_margin = text_margin,
    text_color = text,
    shadow = shadow_text
  );

  let rendered_text = if let Some(link) = config.link.as_ref() {
    render_link(RenderLinkConfig {
      link,
      height: config.height,
      text_length,
      horizontal_padding: config.horizontal_padding,
      left_margin: config.left_margin,
      rendered_text,
    })
  } else {
    rendered_text
  };

  RenderTextReturn {
    text: rendered_text,
    width: text_length,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn render_text_test() {
    let s = render_text(RenderTextConfig {
      left_margin: 12,
      horizontal_padding: 13,
      content: "content<>",
      height: 20,
      vertical_margin: 2,
      shadow: true,
      color: &"#ccc".to_string(),
      link: &None,
      font: &Font::Verdana11Px,
    });

    assert_eq!(
      s.text,
      r##"<text aria-hidden="true" x="545" y="152" fill="#ccc" fill-opacity=".3" transform="scale(.1)" textLength="590">content&lt;&gt;</text><text x="545" y="142" transform="scale(.1)" fill="#333" textLength="590">content&lt;&gt;</text>"##
    );
    assert_eq!(s.width, 59);
  }
}
