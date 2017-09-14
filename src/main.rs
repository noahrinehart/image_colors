use std::path::Path;
use std::process;
mod lib;

#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {

    env_logger::init().expect("Failed to init env_logger!");

    let cli_yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(cli_yaml);
    let matches = app.get_matches();

    debug!("{:?}", matches);

    // Check aruments
    let path = match matches.value_of("PATH") {
        Some(path) => path,
        None => {
            println!("{}", matches.usage());
            process::exit(1);
        }
    };

    let delimiter = matches.value_of("delimiter").unwrap();
    let depth = matches.value_of("depth").unwrap_or("1").parse::<usize>().unwrap();
    let rgb = matches.is_present("rgb");
    let colors = matches.is_present("colors");
    let num_colors = matches.value_of("NUM_COLORS").unwrap_or("5").parse::<usize>().unwrap();

    let mut pixels = lib::fetch_colors(&Path::new(path), depth);
    pixels = lib::sort_colors(&pixels, num_colors);

    lib::print_colors(pixels, colors, delimiter, rgb);
}
