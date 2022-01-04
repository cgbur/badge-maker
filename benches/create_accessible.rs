use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use badge_maker::Links;
use lazy_static::lazy_static;

pub fn create_accessible_text_old(label: &Option<String>, message: &str) -> String {
    if let Some(label) = label {
        format!("{}: {}", label, message)
    } else {
        message.to_string()
    }
}

pub fn create_accessible_text_new(label: &Option<String>, message: &str) -> String {
    if let Some(label) = label {
        let mut buffer = String::with_capacity(label.len() + message.len() + 2);
        buffer.push_str(label);
        buffer.push_str(": ");
        buffer.push_str(message);
        buffer
    } else {
        message.to_string()
    }
}
