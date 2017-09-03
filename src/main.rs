
mod lib;

#[macro_use]
extern crate serde_derive;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate env_logger;

use docopt::Docopt;

const USAGE: &'static str = "
Image Colors
Get colors from an image. Optionally sort by most common.

Usage:
    ./image_colors [options] <path> <num-colors>
    ./image_colors (-h | --help)
    ./image_colors --version

Options:
    -h, --help              Show this screen.
    --version               Show version.
    -c, --colors            Show colors.
    -l, --delimiter DELIM   Delimiter between color and count [defaults: ' has a pixel count of: '].
    -d, --depth DEPTH       Set depth of search (how many pixels iterated by) [default: 1].
    -r, --rgb               Display rgb instead of hex.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_version: bool,
    flag_help: bool,
    flag_delimiter: String,
    flag_depth: usize,
    flag_rgb: bool,
    flag_colors: bool,
    arg_path: String,
    arg_num_colors: usize,
}

fn main() {

    env_logger::init().expect("Failed to init env_logger!");

    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());
    debug!("{:?}", args);
    if args.flag_version {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let mut pixels = lib::fetch_colors(&args.arg_path, args.flag_depth);
    pixels = lib::sort_colors(pixels, args.arg_num_colors);
    lib::print_colors(pixels, args.flag_colors, args.flag_delimiter, args.flag_rgb);
}
