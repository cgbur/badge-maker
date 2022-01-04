mod char_datum;
mod font;
pub(crate) use font::*;
mod util;

#[cfg(test)]
mod tests {
    use crate::render::font::Font::Verdana11Px;

    #[test]
    fn font_test() {
        assert_eq!(Verdana11Px.width_of_str("crates"), 33.64);
        assert_eq!(Verdana11Px.width_of_str(&(207 as char).to_string()), 4.63);
    }
}
