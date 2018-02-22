use std::path::Path;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::fmt;

extern crate ansi_term;
extern crate image;

use self::ansi_term::Color as AnsiColor;
use self::image::DynamicImage;

/// Type that associates a PixelColor to how many pixels in image
pub type ColorCounts = Vec<(PixelColor, usize)>;

/// A single color within an image, stored in rgb
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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

/// Returns a ColorCounts pixel to count vector for a file
///
/// # Arguments
///
/// * 'filename' - A String reference to the filename of the picture
/// * 'depth' - How many pixels from the image to ingest
///
/// # Example
/// ```rust,no_run
/// # use std::path::Path;
/// let _colors = image_colors::fetch_colors(&Path::new("path/to/file.jpg"), 5);
/// ```
pub fn fetch_colors(filepath: &Path, depth: usize) -> ColorCounts {
     let img = image::open(filepath).expect("File couldn't be opened!");
     fetch_colors_img(img, depth)
}
/// Returns a ColorCounts pixel to count vector for a image
///
/// # Arguments
///
/// * 'image' - A Image reference to the instance of the picture
/// * 'depth' - How many pixels from the image to ingest
///
/// # Example
/// ```rust,no_run
/// # extern crate image;
/// # extern crate image_colors;
/// # use std::path::Path;
/// fn main() {
///     let filepath = &Path::new("path/to/file.jpg");
///     let img = image::open(filepath).expect("File couldn't be opened!");
///     let _colors = image_colors::fetch_colors_img(img, 5);
/// }
/// ```
pub fn fetch_colors_img(img: DynamicImage, depth: usize) -> ColorCounts {
    let raw_pixels = img.raw_pixels();
    let raw_pixels_size = raw_pixels.len();
    let mut pixel_map = HashMap::new();

    let mut i = 0;
    while i < raw_pixels_size - 3 {
        let color = PixelColor {
            r: raw_pixels[i],
            g: raw_pixels[i + 1],
            b: raw_pixels[i + 2],
        };
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
/// # use std::path::Path;
/// let _colors = image_colors::fetch_colors(&Path::new("path/to/file.jpg"), 5);
/// let _sorted_colors = image_colors::sort_colors(&_colors, 5);
/// ```
pub fn sort_colors(colors: &ColorCounts, num_colors: usize) -> ColorCounts {
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
/// # use std::path::Path;
/// let _colors = image_colors::fetch_colors(&Path::new("path/to/file.jpg"), 5);
/// let sorted_colors = image_colors::sort_colors(&_colors, 5);
/// image_colors::print_colors(sorted_colors, false, " - ", false);
/// ```
pub fn print_colors(colors: ColorCounts, with_ansi_color: bool, delimiter: &str, with_rgb: bool) {
    for (color, count) in colors {
        let display_color = if with_rgb {
            format!("r:{} g:{} b:{}", color.r, color.g, color.b)
        } else {
            format!("#{:X}", color)
        };

        if with_ansi_color {
            let ansi_color = AnsiColor::RGB(color.r, color.g, color.b);
            println!(
                "{} {}{}{}",
                ansi_color.paint("█"),
                display_color,
                delimiter,
                count
            );
        } else {
            println!("{}{}{}", display_color, delimiter, count);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std;

    fn set_crate_dir() {
        let current = std::env::current_exe().unwrap();
        let deps = current.parent().unwrap();
        let debug = deps.parent().unwrap();
        let target = debug.parent().unwrap();
        let base = target.parent().unwrap();
        assert!(std::env::set_current_dir(base).is_ok());
    }

    #[test]
    fn fetch_tests_dir() {
        set_crate_dir();
        assert!(std::env::current_dir().unwrap().join("tests").is_dir());
    }

    #[test]
    fn fetch_test_image() {
        set_crate_dir();
        assert!(
            std::env::current_dir()
                .unwrap()
                .join("tests")
                .join("diamond.png")
                .is_file()
        )
    }

    #[test]
    fn basic_sort_colors() {
        set_crate_dir();
        let colors = fetch_colors(
            &Path::new(
                std::env::current_dir()
                    .unwrap()
                    .join("tests")
                    .join("diamond.png")
                    .as_path(),
            ),
            10,
        );
        let sorted_colors = sort_colors(&colors, 5);
        assert_eq!(sorted_colors.len(), 5);
        let pixel_colors = sorted_colors.get(0).unwrap().0.clone();
        let pixel_count = sorted_colors.get(0).unwrap().1.clone();
        assert_eq!(pixel_colors.r, 40);
        assert_eq!(pixel_colors.g, 44);
        assert_eq!(pixel_colors.b, 52);
        assert_eq!(pixel_count, 805174);
    }
}
