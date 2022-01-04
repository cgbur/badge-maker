use badge_maker::BadgeBuilder;
use clap::{Clap, ErrorKind};

#[derive(Clap, Debug)]
#[clap(name = "badge-maker", version = "0.1")]
struct Opts {
    label: String,
    message: String,
    #[clap(short, long, default_value("lightgrey"))]
    color: String,
    #[clap(short, long, default_value("grey"))]
    label_color: String,
    #[clap(
    short,
    long("Style from [plastic, flat, flatsquare]"),
    possible_values(&["flat", "plastic", "flatsquare"]),
  default_value("flat")
  )]
    style: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    match BadgeBuilder::new()
        .message(&opts.message)
        .label(&opts.label)
        .color_parse(&opts.color)
        .label_color_parse(&opts.label_color)
        .style_parse(&opts.style)
        .build()
    {
        Ok(badge) => println!("{}", badge.svg()),
        Err(e) => clap::Error::with_description(
            format!("Failed to build badge: {:?}", e),
            ErrorKind::InvalidValue,
        )
        .exit(),
    }
}
