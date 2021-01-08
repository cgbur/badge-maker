use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod hex;
mod render_attributes;
mod xml;
use crate::hex::*;
use crate::render_attributes::*;
use crate::xml::*;
use badge_maker::BadgeBuilder;

pub fn bench_escape_xml(c: &mut Criterion) {
  let mut group = c.benchmark_group("escape_xml");

  group.bench_function("escape_xml_aho_corasick_optimized", |b| {
    b.iter(|| escape_xml_optimized(black_box(TEXT_TO_ESCAPE)))
  });
  // group.bench_function("escape_xml_aho_corasick_optimized_sized", |b| {
  //   b.iter(|| escape_xml_optimized_sized(black_box(TEXT_TO_ESCAPE)))
  // });
  group.bench_function("escape_xml_auto_optimized", |b| {
    b.iter(|| escape_xml_auto_optimized(black_box(TEXT_TO_ESCAPE)))
  });
  // group.bench_function("escape_xml_replace", |b| {
  //   b.iter(|| escape_xml_old(black_box(TEXT_TO_ESCAPE)))
  // });
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

  group.bench_function("strip_xml_trailing_str_find_iter", |b| {
    b.iter(|| strip_xml_trailing_str_find_iter(black_box(SVG_TO_STRIP)))
  });

  group.bench_function("strip_xml_trailing_replace_all", |b| {
    b.iter(|| strip_xml_trailing_replace_all(black_box(SVG_TO_STRIP)))
  });

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

  group.finish();
}

criterion_group!(benches, bench_render_attributes);
criterion_main!(benches);
