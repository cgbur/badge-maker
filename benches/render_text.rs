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
pub struct RenderTextConfig<'a> {
    pub left_margin: isize,
    pub horizontal_padding: usize,
    pub content: &'a str,
    pub height: usize,
    pub vertical_margin: isize,
    pub shadow: bool,
    pub color: &'a str,
}

pub struct RenderTextReturn {
    pub text: String,
    pub width: usize,
}

pub fn render_text_old(config: RenderTextConfig) -> RenderTextReturn {
    if config.content.is_empty() {
        return RenderTextReturn {
            text: "".to_string(),
            width: 0,
        };
    }

    let text_length = 10;
    let escaped_content = escape_xml(&config.content);

    let shadow_margin = 150 + config.vertical_margin;
    let text_margin = 140 + config.vertical_margin;
    let out_text_length = 10 * text_length;

    let x = 10.0
        * (config.left_margin as f32 + 0.5 * text_length as f32 + config.horizontal_padding as f32);

    let text = "#333";
    let shadow = "#shadow";

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

    RenderTextReturn {
        text: rendered_text,
        width: text_length,
    }
}

pub fn render_text_new(config: RenderTextConfig) -> RenderTextReturn {
    if config.content.is_empty() {
        return RenderTextReturn {
            text: "".to_string(),
            width: 0,
        };
    }

    let text_length = 10;
    let escaped_content = escape_xml(&config.content);

    let shadow_margin = 150 + config.vertical_margin;
    let text_margin = 140 + config.vertical_margin;
    let out_text_length = 10 * text_length;

    let x = 10.0
        * (config.left_margin as f32 + 0.5 * text_length as f32 + config.horizontal_padding as f32);

    let text = "#333";
    let shadow = "#shadow";

    let mut buffer = if config.shadow {
        String::with_capacity(160 + 150 + escaped_content.len() * 2)
    } else {
        String::with_capacity(150 + escaped_content.len())
    };

    if config.shadow {
        buffer.push_str(r#"<text aria-hidden="true" x=""#);
        // itoa::fmt(&mut buffer, x as u32);
        buffer.push_str(r#"" y=""#);
        // itoa::fmt(&mut buffer, shadow_margin);
        buffer.push_str(r#"" fill=""#);
        buffer.push_str(shadow);
        buffer.push_str(r#"" fill-opacity=".3" transform="scale(.1)" textLength=""#);
        // itoa::fmt(&mut buffer, out_text_length);
        buffer.push_str(r#"">"#);
        buffer.push_str(&escaped_content);
        buffer.push_str("</text>");
    };

    buffer.push_str(r#"<text x=""#);
    // itoa::fmt(&mut buffer, x as u32);
    buffer.push_str(r#"" y=""#);
    // itoa::fmt(&mut buffer, text_margin);
    buffer.push_str(r#"" transform="scale(.1)" fill=""#);
    buffer.push_str(text);
    buffer.push_str(r#"" textLength=""#);
    // itoa::fmt(&mut buffer, out_text_length);
    buffer.push_str(r#"">"#);
    buffer.push_str(&escaped_content);
    buffer.push_str(r#"</text>"#);

    RenderTextReturn {
        text: buffer,
        width: text_length,
    }
}
