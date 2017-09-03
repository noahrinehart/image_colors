
use std::path::Path;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::fmt;

extern crate image;
extern crate ansi_term;
use self::ansi_term::Color as AnsiColor;

type ColorCounts = Vec<(PixelColor, usize)>;

#[derive(Hash,Eq,PartialEq,Debug,Clone)]
pub struct PixelColor {
   r: u8,
   g: u8,
   b: u8,
}

impl fmt::UpperHex for PixelColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = 0;
        ret = ret << 8 | self.r as u32;
        ret = ret << 8 | self.g as u32;
        ret = ret << 8 | self.b as u32;
        write!(f, "{:X}", ret)
    }
}

pub fn fetch_colors(filename: &String, depth: usize) -> ColorCounts {

    let img = image::open(&Path::new(filename)).unwrap();
    let raw_pixels = img.raw_pixels();
    let raw_pixels_size = raw_pixels.len();
    let mut pixel_map = HashMap::new();

    let mut i = 0;
    while i < raw_pixels_size - 3 {
        let color = PixelColor { r: raw_pixels[i], g: raw_pixels[i+1], b: raw_pixels[i+2] };
        *pixel_map.entry(color).or_insert(0) += 1;
        i += depth * 3;
    }

    Vec::from_iter(pixel_map)
}

pub fn sort_colors(mut colors: ColorCounts, num_colors: usize) -> ColorCounts {
    colors.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    let pixels = if num_colors > colors.len() {
        colors.to_vec()
    } else {
        colors[0..num_colors].to_vec()
    };
    pixels
}

pub fn print_colors(colors: ColorCounts, with_ansi_color: bool, mut delimiter: String, with_rgb: bool) {
    if delimiter.is_empty() {
        delimiter = " has a pixel count of: ".to_string();
    }

    for (color, count) in colors {

        let display_color = if with_rgb {
            format!("r:{} g:{} b:{}", color.r, color.g, color.b)
        } else {
           format!("#{:X}", color)
        };

        if with_ansi_color {
            let ansi_color = AnsiColor::RGB(color.r, color.g, color.b);
            println!("{} {}{}{}", ansi_color.paint("█"), display_color, delimiter, count);
        } else {
            println!("{}{}{}", display_color, delimiter, count);
        }
    }
}

