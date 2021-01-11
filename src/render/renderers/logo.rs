use crate::badge::Logo;
use crate::render::util::xml::escape_xml;

const RAW_STR_LEN: usize = 47;
const NUM_STR_LEN: usize = 14;

pub struct RenderLogoConfig<'a> {
  pub logo: &'a Option<Logo>,
  pub badge_height: usize,
  pub horizontal_padding: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RenderLogoReturn {
  pub rendered_logo: String,
  pub logo_width: usize,
}

pub fn render_logo(config: RenderLogoConfig) -> RenderLogoReturn {
  if config.logo.is_none() {
    return RenderLogoReturn {
      rendered_logo: "".to_string(),
      logo_width: 0,
    };
  }

  let logo = config.logo.clone().unwrap();

  let logo_height = 14;
  let y = (config.badge_height - logo_height) / 2;
  let x = config.horizontal_padding;
  let escaped_logo = escape_xml(logo.url());

  let mut buffer = String::with_capacity(RAW_STR_LEN + NUM_STR_LEN + escaped_logo.len());

  #[cfg(debug_assertions)]
  let start_cap = buffer.capacity();

  buffer.push_str(r#"<image x=""#);
  itoa::fmt(&mut buffer, x).unwrap();
  buffer.push_str(r#"" y=""#);
  itoa::fmt(&mut buffer, y).unwrap();
  buffer.push_str(r#"" width=""#);
  itoa::fmt(&mut buffer, logo.width()).unwrap();
  buffer.push_str(r#"" height=""#);
  itoa::fmt(&mut buffer, logo_height).unwrap();
  buffer.push_str(r#"" xlink:href=""#);
  buffer.push_str(&escaped_logo);
  buffer.push_str(r#""/>"#);

  #[cfg(debug_assertions)]
  assert_eq!(start_cap, buffer.capacity());

  RenderLogoReturn {
    rendered_logo: buffer,
    logo_width: (logo.width() as isize + logo.padding()) as usize,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn render_logo_test() {
    assert_eq!(
      render_logo(RenderLogoConfig {
        logo: &None,
        badge_height: 20,
        horizontal_padding: 30
      }),
      RenderLogoReturn {
        rendered_logo: "".to_string(),
        logo_width: 0
      }
    );
    assert_eq!(
      render_logo(RenderLogoConfig {
        logo: &Some(Logo {
          url: "some_<>url".to_string(),
          width: 20,
          padding: 25
        }),
        badge_height: 20,
        horizontal_padding: 30
      }),
      RenderLogoReturn {
        rendered_logo:
          r#"<image x="30" y="3" width="20" height="14" xlink:href="some_&lt;&gt;url"/>"#
            .to_string(),
        logo_width: 45
      }
    );
  }
}
