use std::path::Path;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::fmt;

extern crate image;
extern crate ansi_term;
use self::ansi_term::Color as AnsiColor;

/// Type that associates a PixelColor to how many pixels in image
pub type ColorCounts = Vec<(PixelColor, usize)>;

/// A single color within an image, stored in rgb
#[derive(Hash,Eq,PartialEq,Debug,Clone)]
pub struct PixelColor {
   r: u8,
   g: u8,
   b: u8,
}

/// Provides hex output for PixelColor
impl fmt::UpperHex for PixelColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = 0;
        ret = ret << 8 | self.r as u32;
        ret = ret << 8 | self.g as u32;
        ret = ret << 8 | self.b as u32;
        write!(f, "{:X}", ret)
    }
}

/// Returns a ColorCounts pixel to count vector
///
/// # Arguments
///
/// * 'filename' - A String reference to the filename of the picture
/// * 'depth' - How many pixels from the image to ingest
///
/// # Example
/// ```rust,no_run
/// let colors: image_colors::ColorCounts = image_colors::fetch_colors("path/to/file.jpg", 5);
/// ```
pub fn fetch_colors(filename: &str, depth: usize) -> ColorCounts {

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

/// Sorts a ColorCounts type by number of pixels
///
/// # Arguments
///
/// * 'colors' - A ColorCounts vector of colors to pixel count
/// * 'num_colors' - How many colors to return
///
/// # Example
/// ```rust,no_run
/// let colors: image_colors::ColorCounts = image_colors::fetch_colors("path/to/file.jpg", 5);
/// let sorted_colors = image_colors::sort_colors(colors, 5);
/// ```
pub fn sort_colors(colors: ColorCounts, num_colors: usize) -> ColorCounts {
    let mut color_copy = colors.clone();
    color_copy.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    let pixels = if num_colors > color_copy.len() {
        color_copy.to_vec()
    } else {
        color_copy[0..num_colors].to_vec()
    };
    pixels
}

/// Prints colors from ColorCounts vector, options to use color, a delimiter, or print in rgb, not
/// hex.
///
/// # Arguments
///
/// * 'colors' - A ColorCounts vector of colors to pixel count
/// * 'with_ansi_color' - Print with color?
/// * 'delimiter' - Delimiter to be used when printing ColorCounts
/// * 'with_rgb' - Print in rgb
///
/// # Example
/// ```rust,no_run
/// let colors: image_colors::ColorCounts = image_colors::fetch_colors("path/to/file.jpg", 5);
/// let sorted_colors = image_colors::sort_colors(colors, 5);
/// image_colors::print_colors(sorted_colors, false, " - ", false);
/// ```
pub fn print_colors(colors: ColorCounts, with_ansi_color: bool, delimiter: &str, with_rgb: bool) {
    let mut delimiter_copy = " has a pixel count of: ".to_string();
    if !delimiter.is_empty() {
        delimiter_copy = " has a pixel count of: ".to_string();
    }

    for (color, count) in colors {
        let display_color = if with_rgb {
            format!("r:{} g:{} b:{}", color.r, color.g, color.b)
        } else {
           format!("#{:X}", color)
        };

        if with_ansi_color {
            let ansi_color = AnsiColor::RGB(color.r, color.g, color.b);
            println!("{} {}{}{}", ansi_color.paint("█"), display_color, delimiter_copy, count);
        } else {
            println!("{}{}{}", display_color, delimiter_copy, count);
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
