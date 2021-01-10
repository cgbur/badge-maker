use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod hex;
mod render_attributes;
mod render_text;
use render_text::*;
mod create_accessible;
mod render_badge;
mod render_title;
use crate::create_accessible::*;
use crate::render_badge::*;
use render_title::*;
mod xml;
use crate::hex::*;
use crate::render_attributes::*;
use crate::xml::*;
use badge_maker::{BadgeBuilder, Links};

pub fn bench_escape_xml(c: &mut Criterion) {
  let mut group = c.benchmark_group("escape_xml");

  group.bench_function("escape_xml_optimized", |b| {
    b.iter(|| escape_xml_optimized(black_box(TEXT_TO_ESCAPE)))
  });
  group.bench_function("escape_xml_optimized_sized", |b| {
    b.iter(|| escape_xml_optimized_sized(black_box(TEXT_TO_ESCAPE)))
  });
  group.bench_function("escape_xml_auto_optimized", |b| {
    b.iter(|| escape_xml_auto_optimized(black_box(TEXT_TO_ESCAPE)))
  });
  group.bench_function("escape_xml_auto_optimized_sized", |b| {
    b.iter(|| escape_xml_auto_optimized_sized(black_box(TEXT_TO_ESCAPE)))
  });
  // group.bench_function("escape_xml_aho_corasick", |b| {
  //   b.iter(|| escape_xml_static(black_box(TEXT_TO_ESCAPE)))
  // });

  group.finish();
}

pub fn bench_strip_xml_whitespace(c: &mut Criterion) {
  let mut group = c.benchmark_group("strip_xml_whitespace");

  group.bench_function("strip_xml_trailing_aho", |b| {
    b.iter(|| strip_xml_trailing_aho(black_box(SVG_TO_STRIP)))
  });

  // group.bench_function("strip_xml_trailing_aho_sized", |b| {
  //   b.iter(|| strip_xml_trailing_aho_sized(black_box(SVG_TO_STRIP)))
  // });

  // group.bench_function("strip_xml_trailing_str_find_iter", |b| {
  //   b.iter(|| strip_xml_trailing_str_find_iter(black_box(SVG_TO_STRIP)))
  // });
  //
  // group.bench_function("strip_xml_trailing_replace_all", |b| {
  //   b.iter(|| strip_xml_trailing_replace_all(black_box(SVG_TO_STRIP)))
  // });

  group.finish();
}

pub fn bench_render_attributes(c: &mut Criterion) {
  let mut group = c.benchmark_group("render_aria_attributes");

  let badge = BadgeBuilder::new().message("hello").build().unwrap();
  let links = badge.links();

  group.bench_function("render_attributes_format", |b| {
    b.iter(|| render_attributes_format(black_box(links), black_box("accessible accessible")))
  });

  group.bench_function("render_attributes_string", |b| {
    b.iter(|| render_attributes_string(black_box(links), black_box("accessible accessible")))
  });

  group.finish();
}

pub fn bench_hex(c: &mut Criterion) {
  let mut group = c.benchmark_group("hex_converter");

  group.bench_function("hex_to_rgb_custom_radix", |b| {
    b.iter(|| hex_to_rgb_custom_radix(black_box("c33c33")))
  });

  group.bench_function("hex_to_rgb_or", |b| {
    b.iter(|| hex_to_rgb_or(black_box("c33c33")))
  });

  group.bench_function("hex_to_rgb_repeat", |b| {
    b.iter(|| hex_to_rgb_repeat(black_box("c33c33")))
  });
  group.finish();
}

pub fn bench_render_text(c: &mut Criterion) {
  let mut group = c.benchmark_group("render_text");

  let param = RenderTextConfig {
    left_margin: 10,
    horizontal_padding: 20,
    content: "Hello There",
    height: 20,
    vertical_margin: 5,
    shadow: true,
    color: "#c33",
  };

  group.bench_function("render_text_old", |b| {
    b.iter(move || render_text_old(black_box(param.clone())))
  });

  group.bench_function("render_text_new", |b| {
    b.iter(move || render_text_new(black_box(param)))
  });

  group.finish();
}

pub fn bench_render_badge(c: &mut Criterion) {
  let mut group = c.benchmark_group("render_badge");

  let badge = BadgeBuilder::new().message("hello").build().unwrap();

  let param = RenderBadgeConfig {
    left_width: 200,
    right_width: 300,
    height: 10,
    accessible_text: "access",
    links: badge.links(),
  };

  group.bench_function("render_badge_old", |b| {
    b.iter(move || render_badge_old(black_box(param.clone()), black_box("main")))
  });

  group.bench_function("render_badge_new", |b| {
    b.iter(move || render_badge_new(black_box(param), black_box("main")))
  });

  group.finish();
}

pub fn bench_render_title(c: &mut Criterion) {
  let mut group = c.benchmark_group("render_title");

  let badge = BadgeBuilder::new().message("hello").build().unwrap();

  group.bench_function("render_title_old", |b| {
    b.iter(|| render_title(black_box(&badge.links().clone()), black_box("main")))
  });

  group.bench_function("render_title_new", |b| {
    b.iter(|| render_title_new(black_box(&badge.links().clone()), black_box("main")))
  });

  group.finish();
}

pub fn bench_crete_accessible(c: &mut Criterion) {
  let mut group = c.benchmark_group("create_accessible");

  group.bench_function("create_accessible_text_old", |b| {
    b.iter(|| create_accessible_text_old(black_box(&Some("hello".to_string())), black_box("there")))
  });

  group.bench_function("create_accessible_text_new", |b| {
    b.iter(|| create_accessible_text_new(black_box(&Some("hello".to_string())), black_box("there")))
  });

  group.finish();
}

criterion_group!(benches, bench_strip_xml_whitespace);
criterion_main!(benches);
