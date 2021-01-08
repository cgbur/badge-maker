//! # Badge-Maker
//!
//! A fast and accurate badge maker for services like [shields.io](https://shields.io/). Verified to
//! match [badge-maker](https://www.npmjs.com/package/badge-maker) 1-to-1 with side by rendering tests*.
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="84" height="20" role="img" aria-label="example: flat"><title>example: flat</title><linearGradient id="bms-6df9790b166df7b8" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="bmr-6df9790b166df7b8"><rect width="84" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#bmr-6df9790b166df7b8)"><rect width="57" height="20" fill="#555"/><rect x="57" width="27" height="20" fill="#ff5b5a"/><rect width="84" height="20" fill="url(#bms-6df9790b166df7b8)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="295" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="470">example</text><text x="295" y="140" transform="scale(.1)" fill="#fff" textLength="470">example</text><text aria-hidden="true" x="695" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="170">flat</text><text x="695" y="140" transform="scale(.1)" fill="#fff" textLength="170">flat</text></g></svg>
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="102" height="18" role="img" aria-label="example: plastic"><title>example: plastic</title><linearGradient id="bms-673fc3b0d46c7e6f" x2="0" y2="100%"><stop offset="0"  stop-color="#fff" stop-opacity=".7"/><stop offset=".1" stop-color="#aaa" stop-opacity=".1"/><stop offset=".9" stop-color="#000" stop-opacity=".3"/><stop offset="1"  stop-color="#000" stop-opacity=".5"/></linearGradient><clipPath id="bmr-673fc3b0d46c7e6f"><rect width="102" height="18" rx="4" fill="#fff"/></clipPath><g clip-path="url(#bmr-673fc3b0d46c7e6f)"><rect width="57" height="18" fill="#555"/><rect x="57" width="45" height="18" fill="#ffb932"/><rect width="102" height="18" fill="url(#bms-673fc3b0d46c7e6f)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="295" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="470">example</text><text x="295" y="130" transform="scale(.1)" fill="#fff" textLength="470">example</text><text aria-hidden="true" x="785" y="140" fill="#ccc" fill-opacity=".3" transform="scale(.1)" textLength="350">plastic</text><text x="785" y="130" transform="scale(.1)" fill="#333" textLength="350">plastic</text></g></svg>
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="122" height="20" role="img" aria-label="example: flatsquare"><title>example: flatsquare</title><g shape-rendering="crispEdges"><rect width="57" height="20" fill="#555"/><rect x="57" width="65" height="20" fill="#fffe27"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text x="295" y="140" transform="scale(.1)" fill="#fff" textLength="470">example</text><text x="885" y="140" transform="scale(.1)" fill="#333" textLength="550">flatsquare</text></g></svg>
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="88" height="20" role="img" aria-label="badge: maker"><title>badge: maker</title><linearGradient id="bms-65d9c12cf1a1b6af" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="bmr-65d9c12cf1a1b6af"><rect width="88" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#bmr-65d9c12cf1a1b6af)"><rect width="43" height="20" fill="#555"/><rect x="43" width="45" height="20" fill="#33b5e5"/><rect width="88" height="20" fill="url(#bms-65d9c12cf1a1b6af)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="225" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="330">badge</text><text x="225" y="140" transform="scale(.1)" fill="#fff" textLength="330">badge</text><text aria-hidden="true" x="645" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="350">maker</text><text x="645" y="140" transform="scale(.1)" fill="#fff" textLength="350">maker</text></g></svg>
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="94" height="20" role="img" aria-label="color: example"><title>color: example</title><linearGradient id="bms-e0307dea9033836d" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="bmr-e0307dea9033836d"><rect width="94" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#bmr-e0307dea9033836d)"><rect width="37" height="20" fill="#555"/><rect x="37" width="57" height="20" fill="#0ac832"/><rect width="94" height="20" fill="url(#bms-e0307dea9033836d)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="195" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="270">color</text><text x="195" y="140" transform="scale(.1)" fill="#fff" textLength="270">color</text><text aria-hidden="true" x="645" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="470">example</text><text x="645" y="140" transform="scale(.1)" fill="#fff" textLength="470">example</text></g></svg>
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="100" height="20" role="img" aria-label="example: badge"><title>example: badge</title><linearGradient id="bms-058ad8d642fc4c85" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="bmr-058ad8d642fc4c85"><rect width="100" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#bmr-058ad8d642fc4c85)"><rect width="57" height="20" fill="#555"/><rect x="57" width="43" height="20" fill="#4c1"/><rect width="100" height="20" fill="url(#bms-058ad8d642fc4c85)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="295" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="470">example</text><text x="295" y="140" transform="scale(.1)" fill="#fff" textLength="470">example</text><text aria-hidden="true" x="775" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="330">badge</text><text x="775" y="140" transform="scale(.1)" fill="#fff" textLength="330">badge</text></g></svg>
//!
//! *_This library differs in that it generates unique IDs for the svg so it can be directly
//!  embedded in websites (such as in this doc_svgs). So a diff between the outputs will not match. We
//! only claim the visual outputs match which is whats important._
//!
//! ## About
//!
//! This library is meant to be the Rust version of
//! [badge-maker](https://www.npmjs.com/package/badge-maker).
//! The use cases for this library are two-fold:
//!  * **Rusty** badge maker for any rust-based [shields.io](https://shields.io/) clones
//!  * **WASM** based npm package for a speed increase over the node based version (numbers coming when
//! optimizations are finished)
//!
//! **Current todos**
//!  1. Better code coverage
//!  2. Optimize text formatters
//!  3. Fix errors. The output is ugly.
//!  4. Other badge styles (requires a custom renderer)
//!
//! ### Example
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="88" height="20" role="img" aria-label="badge: maker"><title>badge: maker</title><linearGradient id="bms-badgemaker55533b5e5" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="bmr-badgemaker55533b5e5"><rect width="88" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#bmr-badgemaker55533b5e5)"><rect width="43" height="20" fill="#555"/><rect x="43" width="45" height="20" fill="#33b5e5"/><rect width="88" height="20" fill="url(#bms-badgemaker55533b5e5)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="225" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="330">badge</text><text x="225" y="140" transform="scale(.1)" fill="#fff" textLength="330">badge</text><text aria-hidden="true" x="645" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="350">maker</text><text x="645" y="140" transform="scale(.1)" fill="#fff" textLength="350">maker</text></g></svg>
//! ```rust
//! use badge_maker::BadgeBuilder;
//!
//! let svg = BadgeBuilder::new()
//!       .label("badge")
//!       .message("maker")
//!       .color_parse("#33B5E5")
//!       .build()?
//!       .svg();
//!
//! println!("{}", svg);
//!
//! # Ok::<(), badge_maker::error::Error>(())
//! ```
//!
//! ## Features
//! > This library is still in its infancy. Tests and documentation are being added whenever
//! possible. If you are interested in contributing then check out the [repository](https://github.com/cgburgess/badge-maker).
//! The API is likely to change. If you see something you think would work better than
//! the way I've done it open an issue, I'd love to hear your suggestions.
//!
//! We support different [styles](Style), [colors](color::Color), [logos](Logo), and [links](Links). The
//! [badge builder](BadgeBuilder) accepts all of these options with the `field()` and an
//! alternate method of `field_parse()` which accepts a string and will attempt parse the text as
//! a valid field.
//!
//! #### CLI
//!
//! This is a library but you can use it with a simple cli tool as well.
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="100" height="20" role="img" aria-label="example: badge"><title>example: badge</title><linearGradient id="bms-141813996f263f24" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="bmr-141813996f263f24"><rect width="100" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#bmr-141813996f263f24)"><rect width="57" height="20" fill="#282828"/><rect x="57" width="43" height="20" fill="#007ec6"/><rect width="100" height="20" fill="url(#bms-141813996f263f24)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="295" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="470">example</text><text x="295" y="140" transform="scale(.1)" fill="#fff" textLength="470">example</text><text aria-hidden="true" x="775" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="330">badge</text><text x="775" y="140" transform="scale(.1)" fill="#fff" textLength="330">badge</text></g></svg>
//!
//! install
//! ```bash
//! cargo install badge-maker --features cli
//! ```
//! use
//! ```bash
//! badge-maker example badge -c informational -l #282828 -s flat
//! ```
//!
//!
//! ### [Colors](Color)
//!
//! We currently support hex colors 3 and 6 chars long, [named colors](color::NamedColor)
//! and their [alias's](color::AliasColor), and [RGB](color::Color::Rgb) color inputs. These can be constructed
//! with their enum variants or using the `...parse()` methods.
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="94" height="20" role="img" aria-label="color: example"><title>color: example</title><linearGradient id="bms-e0307dea9033836d" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="bmr-e0307dea9033836d"><rect width="94" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#bmr-e0307dea9033836d)"><rect width="37" height="20" fill="#555"/><rect x="37" width="57" height="20" fill="#0ac832"/><rect width="94" height="20" fill="url(#bms-e0307dea9033836d)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="195" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="270">color</text><text x="195" y="140" transform="scale(.1)" fill="#fff" textLength="270">color</text><text aria-hidden="true" x="645" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="470">example</text><text x="645" y="140" transform="scale(.1)" fill="#fff" textLength="470">example</text></g></svg>
//! ```rust
//! use badge_maker::BadgeBuilder;
//! use badge_maker::color::{Color, AliasColor, NamedColor};
//!
//! let svg = BadgeBuilder::new()
//!     .label("color")
//!     .message("example")
//!     // by enums
//!     .color(Color::Named(NamedColor::BrightGreen))
//!     .color(Color::Alias(AliasColor::Success))
//!     .color(Color::Rgb(10, 200, 50))
//!     // or parsing
//!     .color_parse("brightgreen")
//!     .color_parse("success")
//!     .color_parse("rgb(10, 200, 50)")
//!     .build()?
//!     .svg();
//!
//! # Ok::<(), badge_maker::error::Error>(())
//! ```
//!
//!
//! ### [Styles](Style)
//! **Supported**. Others coming soon. See [Style](Style) enum for choices when
//! building or use the string literals.
//!
//!  - **Flat** <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="84" height="20" role="img" aria-label="example: flat"><title>example: flat</title><linearGradient id="bms-exampleflat555ff5b5a" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/><stop offset="1" stop-opacity=".1"/></linearGradient><clipPath id="bmr-exampleflat555ff5b5a"><rect width="84" height="20" rx="3" fill="#fff"/></clipPath><g clip-path="url(#bmr-exampleflat555ff5b5a)"><rect width="57" height="20" fill="#555"/><rect x="57" width="27" height="20" fill="#ff5b5a"/><rect width="84" height="20" fill="url(#bms-exampleflat555ff5b5a)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="295" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="470">example</text><text x="295" y="140" transform="scale(.1)" fill="#fff" textLength="470">example</text><text aria-hidden="true" x="695" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="170">flat</text><text x="695" y="140" transform="scale(.1)" fill="#fff" textLength="170">flat</text></g></svg>
//!  - **Plastic** <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="102" height="18" role="img" aria-label="example: plastic"><title>example: plastic</title><linearGradient id="bms-673fc3b0d46c7e6f" x2="0" y2="100%"><stop offset="0"  stop-color="#fff" stop-opacity=".7"/><stop offset=".1" stop-color="#aaa" stop-opacity=".1"/><stop offset=".9" stop-color="#000" stop-opacity=".3"/><stop offset="1"  stop-color="#000" stop-opacity=".5"/></linearGradient><clipPath id="bmr-673fc3b0d46c7e6f"><rect width="102" height="18" rx="4" fill="#fff"/></clipPath><g clip-path="url(#bmr-673fc3b0d46c7e6f)"><rect width="57" height="18" fill="#555"/><rect x="57" width="45" height="18" fill="#ffb932"/><rect width="102" height="18" fill="url(#bms-673fc3b0d46c7e6f)"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text aria-hidden="true" x="295" y="140" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="470">example</text><text x="295" y="130" transform="scale(.1)" fill="#fff" textLength="470">example</text><text aria-hidden="true" x="785" y="140" fill="#ccc" fill-opacity=".3" transform="scale(.1)" textLength="350">plastic</text><text x="785" y="130" transform="scale(.1)" fill="#333" textLength="350">plastic</text></g></svg>
//!  - **FlatSquare** <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="122" height="20" role="img" aria-label="example: flatsquare"><title>example: flatsquare</title><g shape-rendering="crispEdges"><rect width="57" height="20" fill="#555"/><rect x="57" width="65" height="20" fill="#fffe27"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><text x="295" y="140" transform="scale(.1)" fill="#fff" textLength="470">example</text><text x="885" y="140" transform="scale(.1)" fill="#333" textLength="550">flatsquare</text></g></svg>
//!  - ForTheBadge
//!  - Social
//!
//! ```rust
//! use badge_maker::{BadgeBuilder, Style};
//!
//! let svg = BadgeBuilder::new()
//!   .label("example")
//!   .message("plastic")
//!   .color_parse("#FFB932")
//!   .style(Style::Plastic) // example of using typed input
//!   .style_parse("plastic") // example of parsing to derive
//!   .build()?
//!   .svg();
//!
//! println!("{}", svg);
//!
//! # Ok::<(), badge_maker::error::Error>(())
//! ```
//!
//! ### [Links](Links) & [Logos](Logo)
//! Adding links to the natively rendered badge supported. This is great if you need
//! to embed the svg directly. However, on a website like the rust docs they may show
//! the underline. To solve this, your third-party api that renders the badges should
//! wrap the svg in markdown `[![name for readers](link to api endpoint)](link when clicked)`.
//!
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="81" height="20" ><a target="_blank" xlink:href="https://www.rust-lang.org/"><g shape-rendering="crispEdges"><rect width="50" height="20" fill="#555"/><rect x="50" width="31" height="20" fill="#f5f5f5"/></g><g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><image x="5" y="3" width="14" height="14" xlink:href="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1024px-Rust_programming_language_black_logo.svg.png"/><text x="345" y="140" transform="scale(.1)" fill="#fff" textLength="230">lang</text><text x="645" y="140" transform="scale(.1)" fill="#333" textLength="210">rust</text></g></a></svg>
//! ```rust
//! use badge_maker::BadgeBuilder;
//!
//! let logo_url = "https://upload.wikimedia.org/wikipedia/commons/\
//!   thumb/d/d5/Rust_programming_language_black_logo.svg/\
//!   1024px-Rust_programming_language_black_logo.svg.png";
//!
//! let svg = BadgeBuilder::new()
//!   .label("lang")
//!   .message("rust")
//!   .color_parse("#F5F5F5")
//!   .link("https://www.rust-lang.org/")
//!   .logo_url(logo_url)
//!   .style_parse("flatsquare")
//!   .build()?
//!   .svg();
//!
//! # Ok::<(), badge_maker::error::Error>(())
//! ```

pub use badge::color;
pub use badge::{Badge, BadgeBuilder, Links, Logo, Style};

pub mod error;

mod badge;
mod render;
