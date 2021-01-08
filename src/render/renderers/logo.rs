use crate::badge::Logo;
use crate::render::util::xml::escape_xml;

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

  let rendered_logo = format!(
    r#"<image x="{x}" y="{y}" width="{logo_width}" height="{logo_height}" xlink:href="{escaped_logo}"/>"#,
    x = x,
    y = y,
    logo_width = logo.width(),
    logo_height = logo_height,
    escaped_logo = escape_xml(logo.url())
  );

  RenderLogoReturn {
    rendered_logo,
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
