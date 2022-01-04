use crate::badge::Badge;

use crate::render::base::BaseConfig;
use crate::render::font::Font;
use crate::render::renderers::{render_badge, RenderBadgeConfig};
use crate::render::{BadgeRenderer, FONT_FAMILY};

pub struct Plastic {}

impl BadgeRenderer for Plastic {
    fn font_family(&self) -> &'static str {
        FONT_FAMILY
    }

    fn font(&self) -> Font {
        Font::Verdana11Px
    }

    fn height(&self) -> usize {
        18
    }

    fn vertical_margin(&self) -> isize {
        -10
    }

    fn shadow(&self) -> bool {
        true
    }

    fn render(badge: &Badge) -> String {
        let renderer = Self {};

        let config = BaseConfig::new(badge, &renderer);

        let body = format!(
            r##"<linearGradient id="{s}" x2="0" y2="100%">
        <stop offset="0"  stop-color="#fff" stop-opacity=".7"/>
        <stop offset=".1" stop-color="#aaa" stop-opacity=".1"/>
        <stop offset=".9" stop-color="#000" stop-opacity=".3"/>
        <stop offset="1"  stop-color="#000" stop-opacity=".5"/>
      </linearGradient>
      <clipPath id="{r}">
        <rect width="{width}" height="{height}" rx="4" fill="#fff"/>
      </clipPath>
      <g clip-path="url(#{r})">
        <rect width="{left_width}" height="{height}" fill="{label_color}"/>
        <rect x="{left_width}" width="{right_width}" height="{height}" fill="{color}"/>
        <rect width="{width}" height="{height}" fill="url(#{s})"/>
      </g>
      <g fill="#fff" text-anchor="middle" {font_family} text-rendering="geometricPrecision" font-size="110">
        {rendered_logo}{rendered_label}{rendered_message}
      </g>"##,
            left_width = config.left_width,
            right_width = config.right_width,
            width = config.width,
            rendered_label = config.rendered_label,
            rendered_message = config.rendered_message,
            font_family = renderer.font_family(),
            height = renderer.height(),
            label_color = config.label_color,
            color = config.color,
            rendered_logo = config.rendered_logo,
            s = badge.ids(),
            r = badge.idr()
        );
        render_badge(
            RenderBadgeConfig {
                left_width: config.left_width,
                right_width: config.right_width,
                height: renderer.height(),
                accessible_text: &config.accessible_text,
                links: &config.links,
            },
            &body,
        )
    }
}
