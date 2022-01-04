pub enum Color {
    Named(NamedColor),
    Alias(AliasColor),
    Rgb(u8, u8, u8),
}

/// Colors from the
/// [badge-maker](https://github.com/badges/shields/blob/master/badge-maker/lib/color.js) spec
pub enum NamedColor {
    BrightGreen,
    Green,
    Yellow,
    YellowGreen,
    Orange,
    Red,
    Blue,
    Grey,
    LightGrey,
}

pub enum AliasColor {
    Gray,
    LightGray,
    Critical,
    Important,
    Success,
    Informational,
    Inactive,
}

impl NamedColor {
    pub fn hex(&self) -> &'static str {
        match self {
            NamedColor::BrightGreen => "#4c1",
            NamedColor::Green => "#97ca00",
            NamedColor::Yellow => "#dfb317",
            NamedColor::YellowGreen => "#a4a61d",
            NamedColor::Orange => "#fe7d37",
            NamedColor::Red => "#e05d44",
            NamedColor::Blue => "#007ec6",
            NamedColor::Grey => "#555",
            NamedColor::LightGrey => "#9f9f9f",
        }
    }
}

impl AliasColor {
    pub fn hex(&self) -> &'static str {
        match self {
            AliasColor::Gray => NamedColor::Grey.hex(),
            AliasColor::LightGray => NamedColor::LightGrey.hex(),
            AliasColor::Critical => NamedColor::Red.hex(),
            AliasColor::Important => NamedColor::Orange.hex(),
            AliasColor::Success => NamedColor::BrightGreen.hex(),
            AliasColor::Informational => NamedColor::Blue.hex(),
            AliasColor::Inactive => NamedColor::LightGrey.hex(),
        }
    }
}
