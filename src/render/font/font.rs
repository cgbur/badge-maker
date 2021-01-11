use crate::render::font::char_datum::CharDatum;
use crate::render::font::util::is_control_char;
use lazy_static::lazy_static;

lazy_static! {
  static ref FONTS: [Vec<CharDatum>; 1] = {
    // let helv = include_bytes!("../../fonts/helvetica-11px-bold.bincode");
    // let verd10bold = include_bytes!("../../fonts/verdana-10px-bold.bincode");
    // let verd10 = include_bytes!("../../fonts/verdana-10px-normal.bincode");
    let verd11 = include_bytes!("../../../fonts/verdana-11px-normal.bincode");
    [
         bincode::deserialize(verd11).unwrap(),
      // bincode::deserialize(helv).unwrap(),
      // bincode::deserialize(verd10bold).unwrap(),
      // bincode::deserialize(verd10).unwrap(),
    ]
  };
}

pub enum Font {
  Verdana11Px,
  // Helvetica11PxBold, unused, needed for new badges
  // Verdana10PxBold,
  // Verdana10Px,
}

impl Font {
  pub fn width_of_str(&self, s: &str) -> f32 {
    let dict = self.char_dict();

    s.chars()
      .map(|c| Font::width_of_char_code(c as u32, dict))
      .sum::<f32>()
  }

  fn width_of_char_code(c: u32, dict: &[CharDatum]) -> f32 {
    if is_control_char(c) {
      return 0.0;
    }
    match dict.binary_search_by_key(&c, |datum| datum.low) {
      Ok(idx) => dict[idx].width,
      Err(idx) => {
        let datum = &dict[idx - 1];
        if datum.contains(c) {
          datum.width
        } else {
          Font::width_of_char_code('m' as u32, dict)
        }
      }
    }
  }

  fn char_dict(&self) -> &[CharDatum] {
    match self {
      Font::Verdana11Px => &FONTS[0],
      // Since the only badges implemented rely on verdana11px...
      // Font::Helvetica11PxBold => Some(&FONTS[1]),
      // Font::Verdana10PxBold => Some(&FONTS[2]),
      // Font::Verdana10Px => Some(&FONTS[3]),
    }
  }
}
