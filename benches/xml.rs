use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

use lazy_static::lazy_static;
use regex::Regex;

pub const SVG_TO_STRIP: &'static str = r###"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="340" height="20" role="img"
     aria-label="i spent the past two days: writing the library to make these">
    <title>i spent the past two days: writing the library to make these</title>
    <linearGradient id="s" x2="0" y2="100%">
        <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
        <stop offset="1" stop-opacity=".1"/>
    </linearGradient>
    <clipPath id="r">
        <rect width="340" height="20" rx="3" fill="#fff"/>
    </clipPath>
    <g clip-path="url(#r)">
        <rect width="149" height="20" fill="#555"/>
        <rect x="149" width="191" height="20" fill="#6DF7FF"/>
        <rect width="340" height="20" fill="url(#s)"/>
    </g>
    <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif"
       text-rendering="geometricPrecision" font-size="110">
        <text aria-hidden="true" x="755" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)"
              textLength="1390">i spent the past two days <><>< >< < && & & & 'a'aa'a
        </text>
        <text x="755" y="140" transform="scale(.1)" fill="#fff" textLength="1390">i spent the past two days</text>
        <text aria-hidden="true" x="2435" y="150" fill="#ccc" fill-opacity=".3" transform="scale(.1)" textLength="1810">
            writing the library to make these> << &&&& '"";; &&**&*&
        </text>
        <text x="2435" y="140" transform="scale(.1)" fill="#333" textLength="1810">**888**77&&&&..''""><><><writing the library to make these
        </text>
    </g>
</svg>"###;

pub const TEXT_TO_ESCAPE: &'static str = r###"Hello there!"###;
const XML_ESCAPE_PATTERNS: [&str; 5] = ["&", "<", ">", "\"", "'"];
const XML_ESCAPE_REPLACEMENTS: [&str; 5] = ["&amp;", "&lt;", "&gt;", "&quot;", "&apos;"];

lazy_static! {
  static ref TRAILING_SPACE: Regex = Regex::new(r">\s+").unwrap();
}

lazy_static! {
  static ref TRAILING_SPACE_BYTES: regex::bytes::Regex = regex::bytes::Regex::new(r">\s+").unwrap();
}

const XML_STRIP_TRAILING_PATTERNS_LEN: usize = 8;
lazy_static! {
  static ref XML_STRIP_TRAILING_PATTERNS: (Vec<String>, Vec<String>) = {
    let a = aho_corasick_pattern_builder(XML_STRIP_TRAILING_PATTERNS_LEN, ">", " ");
    let b = aho_corasick_pattern_builder(XML_STRIP_TRAILING_PATTERNS_LEN, ">\n", " ");
    combine_builders(a, b)
  };
}

pub fn aho_corasick_pattern_builder(
  size: usize,
  start: &str,
  onwards: &str,
) -> (Vec<String>, Vec<String>) {
  // todo see if new lines are generating on windows

  let mut matches = vec![];

  let mut building = String::from(format!("{}{}", start, onwards));
  for _ in 0..size {
    matches.push(building.to_string());
    building.push_str(onwards);
  }
  matches.reverse();

  (matches, vec![">".to_string(); size])
}

pub fn strip_xml_trailing_str_find_iter(s: &str) -> String {
  // let TRAILING_SPACE: Regex = Regex::new(r">\s+").unwrap();
  let mut ret_string = String::with_capacity(s.len());
  let mut last = 0;
  for mat in TRAILING_SPACE.find_iter(&s) {
    ret_string.push_str(&s[last..=mat.start()]);
    last = mat.end();
  }
  ret_string.push_str(&s[last..s.len()]);

  ret_string
}

pub fn strip_xml_trailing_replace_all(s: &str) -> String {
  TRAILING_SPACE.replace_all(s, ">").to_string()
}

pub fn strip_xml_trailing_aho(s: &str) -> String {
  lazy_static! {
    static ref AC: AhoCorasick = AhoCorasickBuilder::new()
      .auto_configure(&XML_STRIP_TRAILING_PATTERNS.0)
      .match_kind(MatchKind::LeftmostFirst)
      .build(&XML_STRIP_TRAILING_PATTERNS.0);
  };

  AC.replace_all(&s, &XML_STRIP_TRAILING_PATTERNS.1)
}

pub fn strip_xml_trailing_aho_sized(s: &str) -> String {
  lazy_static! {
    static ref AC: AhoCorasick<u16> = AhoCorasickBuilder::new()
      .auto_configure(&XML_STRIP_TRAILING_PATTERNS.0)
      .match_kind(MatchKind::LeftmostFirst)
      .build_with_size(&XML_STRIP_TRAILING_PATTERNS.0)
      .unwrap();
  };

  AC.replace_all(&s, &XML_STRIP_TRAILING_PATTERNS.1)
}

fn combine_builders(
  (mut a, mut b): (Vec<String>, Vec<String>),
  (mut aa, mut bb): (Vec<String>, Vec<String>),
) -> (Vec<String>, Vec<String>) {
  a.append(&mut aa);
  b.append(&mut bb);
  (a, b)
}

pub fn escape_xml_old(s: &str) -> String {
  s.replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    .replace('"', "&quot;")
    .replace('\'', "&apos;")
}

pub fn escape_xml_static(s: &str) -> String {
  lazy_static! {
    static ref AC: AhoCorasick = AhoCorasickBuilder::new().build(&XML_ESCAPE_PATTERNS);
  };

  AC.replace_all(s, &XML_ESCAPE_REPLACEMENTS)
}

pub fn escape_xml_optimized(s: &str) -> String {
  lazy_static! {
    static ref AC: AhoCorasick = AhoCorasickBuilder::new()
      .dfa(true)
      .build(&XML_ESCAPE_PATTERNS);
  };

  AC.replace_all(s, &XML_ESCAPE_REPLACEMENTS)
}

pub fn escape_xml_auto_optimized(s: &str) -> String {
  lazy_static! {
    static ref AC: AhoCorasick = AhoCorasickBuilder::new()
      .auto_configure(&XML_ESCAPE_REPLACEMENTS)
      .build(&XML_ESCAPE_PATTERNS);
  };

  AC.replace_all(s, &XML_ESCAPE_REPLACEMENTS)
}

pub fn escape_xml_optimized_sized(s: &str) -> String {
  lazy_static! {
    static ref AC: AhoCorasick<u8> = AhoCorasickBuilder::new()
      .dfa(true)
      .build_with_size(&XML_ESCAPE_PATTERNS)
      .unwrap();
  };

  AC.replace_all(s, &XML_ESCAPE_REPLACEMENTS)
}

pub fn escape_xml_auto_optimized_sized(s: &str) -> String {
  lazy_static! {
    static ref AC: AhoCorasick<u16> = AhoCorasickBuilder::new()
      .auto_configure(&XML_ESCAPE_REPLACEMENTS)
      .build_with_size(&XML_ESCAPE_PATTERNS)
      .unwrap();
  };

  AC.replace_all(s, &XML_ESCAPE_REPLACEMENTS)
}
