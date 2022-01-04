pub fn hex_to_rgb_repeat(hex: &str) -> (u32, u32, u32) {
    let rgb = match hex.len() {
        3 => {
            let r = u32::from_str_radix(&hex[0..1].repeat(2), 16).unwrap();
            let g = u32::from_str_radix(&hex[1..2].repeat(2), 16).unwrap();
            let b = u32::from_str_radix(&hex[2..3].repeat(2), 16).unwrap();

            (r, g, b)
        }
        6 => {
            let r = u32::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u32::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u32::from_str_radix(&hex[4..6], 16).unwrap();
            (r, g, b)
        }
        _ => panic!("unknown hex length"),
    };

    rgb
}

pub fn hex_to_rgb_or(hex: &str) -> (u32, u32, u32) {
    let rgb = match hex.len() {
        3 => {
            let r = u32::from_str_radix(&hex[0..1], 16).unwrap();
            let g = u32::from_str_radix(&hex[1..2], 16).unwrap();
            let b = u32::from_str_radix(&hex[2..3], 16).unwrap();

            let r = r << 4 | r;
            let g = g << 4 | g;
            let b = b << 4 | b;
            (r, g, b)
        }
        6 => {
            let r = u32::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u32::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u32::from_str_radix(&hex[4..6], 16).unwrap();
            (r, g, b)
        }
        _ => panic!("unknown hex length"),
    };

    rgb
}

fn hex_to_u32_one(c: &char) -> u32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!("unknown digit"),
    }
}

pub fn hex_to_u32(s: &str) -> u32 {
    let mut num = 0;
    for c in s.chars() {
        num = num << 4 | hex_to_u32_one(&c);
    }
    num
}

pub fn hex_to_rgb_custom_radix(hex: &str) -> (u32, u32, u32) {
    let rgb = match hex.len() {
        3 => {
            let r = hex_to_u32(&hex[0..1]);
            let g = hex_to_u32(&hex[1..2]);
            let b = hex_to_u32(&hex[2..3]);

            let r = r << 4 | r;
            let g = g << 4 | g;
            let b = b << 4 | b;
            (r, g, b)
        }
        6 => {
            let r = hex_to_u32(&hex[0..2]);
            let g = hex_to_u32(&hex[2..4]);
            let b = hex_to_u32(&hex[4..6]);
            (r, g, b)
        }
        _ => panic!("unknown hex length"),
    };

    rgb
}
