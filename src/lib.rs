
use std::path::Path;
use std::collections::HashMap;
use std::iter::FromIterator;

extern crate image;

pub fn fetch_colors(filename: &String, num_colors: usize) {

    let img = image::open(&Path::new(filename)).unwrap();
    let raw_pixels = img.raw_pixels();
    let raw_pixels_size = raw_pixels.len();
    let mut pixel_map = HashMap::new();

    let mut i = 0;
    while i < raw_pixels_size {
        let pixel = convert_nibbles_to_u32(&[raw_pixels[i], raw_pixels[i+1], raw_pixels[i+2]]);
        *pixel_map.entry(pixel).or_insert(0) += 1;
        i += 3;
    }

    let mut sorted_pixels = Vec::from_iter(pixel_map);
    sorted_pixels.sort_by(|&(_, a), &(_, b)| b.cmp(&a));


    i = 0;
    while i < num_colors {
         match sorted_pixels.get(i) {
            Some(&(pixel, count)) => {
                println!("#{:X} - {} pixels", pixel, count);
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
