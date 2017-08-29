
use std::path::Path;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::fmt;

extern crate image;

#[derive(Hash,Eq,PartialEq)]
struct Color {
   r: u8,
   g: u8,
   b: u8,
}

impl Color {
    pub fn convert_to_u32(&self) -> u32 {
        let mut ret = 0;
        ret = ret << 8 | self.r as u32;
        ret = ret << 8 | self.g as u32;
        ret = ret << 8 | self.b as u32;
        ret
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = 0;
        ret = ret << 8 | self.r as u32;
        ret = ret << 8 | self.g as u32;
        ret = ret << 8 | self.b as u32;
        write!(f, "{:X}", ret)
    }
}

pub fn fetch_colors(filename: &String, num_colors: usize) {

    let img = image::open(&Path::new(filename)).unwrap();
    let raw_pixels = img.raw_pixels();
    let raw_pixels_size = raw_pixels.len();
    let mut pixel_map = HashMap::new();

    let mut i = 0;
    while i < raw_pixels_size {
        let color = Color { r: raw_pixels[i], g: raw_pixels[i+1], b: raw_pixels[i+2] };
        *pixel_map.entry(color).or_insert(0) += 1;
        i += 3;
    }

    let mut sorted_pixels = Vec::from_iter(pixel_map);
    sorted_pixels.sort_by(|&(_, a), &(_, b)| b.cmp(&a));


    i = 0;
    while i < num_colors {
         match sorted_pixels.get(i) {
            Some(&(color :Color, count)) => {
                let num = color.convert_nibbles_to_u32();
                println!("#{:X} - {} pixels", num, count);
            },
            None => break,
        }
        i += 1;
    }
}

fn convert_nibbles_to_u32(values: &[u8]) -> u32{
    let mut ret = 0;
    for &i in values {
        ret = ret << 8 | i as u32;
    }
    ret
}

