 # Badge-Maker

_Links are generated from cargo, view on docs page_


 A fast and accurate badge maker for services like [shields.io](https://shields.io/). Verified to
 match [badge-maker](https://www.npmjs.com/package/badge-maker) 1-to-1 with side by rendering tests*.

 ![example_flat][flat]

 ![example_plastic][plastic]

 ![example_flat_square][flatsquare]

 ![example_badge_maker][badge_maker]

 ![example_color][example_color]

 ![example_badge][example_badge]

 [flat]: https://raw.githubusercontent.com/cgburgess/badge-maker/master/doc_svgs/example_flat.svg
 [plastic]: https://raw.githubusercontent.com/cgburgess/badge-maker/master/doc_svgs/example_plastic.svg
 [flatsquare]: https://raw.githubusercontent.com/cgburgess/badge-maker/master/doc_svgs/example_flatsquare.svg
 [badge_maker]: https://raw.githubusercontent.com/cgburgess/badge-maker/master/doc_svgs/example_badge_maker.svg
 [example_color]: https://raw.githubusercontent.com/cgburgess/badge-maker/master/doc_svgs/example_color.svg
 [example_badge]: https://raw.githubusercontent.com/cgburgess/badge-maker/master/doc_svgs/example_badge.svg
 [example_cli]: https://raw.githubusercontent.com/cgburgess/badge-maker/master/doc_svgs/example_cli.svg
 [link_logo]: https://raw.githubusercontent.com/cgburgess/badge-maker/master/doc_svgs/example_link_logo.svg
*_This library differs in that it generates unique IDs for the svg so it can be directly
  embedded in websites (such as in this doc). So a diff between the outputs will not match. We
 only claim the visual outputs match which is whats important._

 ## About

 This library is meant to be the Rust version of
 [badge-maker](https://www.npmjs.com/package/badge-maker).
 The use cases for this library are two-fold:
  * **Rusty** badge maker for any rust-based [shields.io](https://shields.io/) clones
  * **WASM** based npm package for a speed increase over the node based version (numbers coming when
 optimizations are finished)

 **Current todos**
  1. Better code coverage
  2. Optimize text formatters
  3. Fix errors. The output is ugly.
  4. Other badge styles (requires a custom renderer)

 ### Example
![example_badge_maker][badge_maker]
 ```rust
 use badge_maker::BadgeBuilder;

 let svg = BadgeBuilder::new()
       .label("badge")
       .message("maker")
       .color_parse("#33B5E5")
       .build()?
       .svg();

 println!("{}", svg);

 # Ok::<(), badge_maker::error::Error>(())
 ```

 ## Features
 > This library is still in its infancy. Tests and documentation are being added whenever
 possible. If you are interested in contributing then check out the [repository](https://github.com/cgburgess/badge-maker).
 The API is likely to change. If you see something you think would work better than
 the way I've done it open an issue, I'd love to hear your suggestions.

 We support different [styles](Style), [colors](color::Color), [logos](Logo), and [links](Links). The
 [badge builder](BadgeBuilder) accepts all of these options with the `field()` and an
 alternate method of `field_parse()` which accepts a string and will attempt parse the text as
 a valid field.

 #### CLI

 This is a library but you can use it with a simple cli tool as well.

 ![cli][example_cli]

 install
 ```bash
 cargo install badge-maker --features cli
 ```
 use
 ```bash
 badge-maker example badge -c informational -l #282828 -s flat
 ```


 ### [Colors](Color)

 We currently support hex colors 3 and 6 chars long, [named colors](color::NamedColor)
 and their [alias's](color::AliasColor), and [RGB](color::Color::Rgb) color inputs. These can be constructed
 with their enum variants or using the `...parse()` methods.

![example_color][example_color]

 ```rust
 use badge_maker::BadgeBuilder;
 use badge_maker::color::{Color, AliasColor, NamedColor};

 let svg = BadgeBuilder::new()
     .label("color")
     .message("example")
     // by enums
     .color(Color::Named(NamedColor::BrightGreen))
     .color(Color::Alias(AliasColor::Success))
     .color(Color::Rgb(10, 200, 50))
     // or parsing
     .color_parse("brightgreen")
     .color_parse("success")
     .color_parse("rgb(10, 200, 50)")
     .build()?
     .svg();

 # Ok::<(), badge_maker::error::Error>(())
 ```


 ### [Styles](Style)
 **Supported**. Others coming soon. See [Style](Style) enum for choices when
 building or use the string literals.

  - **Flat** ![example_flat][flat]
  - **Plastic** ![example_plastic][plastic]
  - **FlatSquare** ![example_flat_square][flatsquare]
  - ForTheBadge
  - Social

 ```rust
 use badge_maker::{BadgeBuilder, Style};

 let svg = BadgeBuilder::new()
   .label("example")
   .message("plastic")
   .color_parse("#FFB932")
   .style(Style::Plastic) // example of using typed input
   .style_parse("plastic") // example of parsing to derive
   .build()?
   .svg();

 println!("{}", svg);

 # Ok::<(), badge_maker::error::Error>(())
 ```

 ### [Links](Links) & [Logos](Logo)
 Adding links to the natively rendered badge supported. This is great if you need
 to embed the svg directly. However, on a website like the rust docs they may show
 the underline. To solve this, your third-party api that renders the badges should
 wrap the svg in markdown `[![name for readers](link to api endpoint)](link when clicked)`.


![example_link_logo][link_logo]
 ```rust
 use badge_maker::BadgeBuilder;

 let logo_url = "https://upload.wikimedia.org/wikipedia/commons/\
   thumb/d/d5/Rust_programming_language_black_logo.svg/\
   1024px-Rust_programming_language_black_logo.svg.png";

 let svg = BadgeBuilder::new()
   .label("lang")
   .message("rust")
   .color_parse("#F5F5F5")
   .link("https://www.rust-lang.org/")
   .logo_url(logo_url)
   .style_parse("flatsquare")
   .build()?
   .svg();

 # Ok::<(), badge_maker::error::Error>(())
 ```