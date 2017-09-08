use std::path::Path;
use std::str::FromStr;
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
    let matches = App::from_yaml(cli_yaml).get_matches();

    debug!("{:?}", matches);

    // Check aruments
    if matches.is_present("version") {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if matches.is_present("help") {
        
    }
    

    // let path = matches.value_of("PATH").unwrap();


    // let mut pixels = lib::fetch_colors(&Path::new(&args.arg_path), args.flag_depth);
    // pixels = lib::sort_colors(&pixels, args.arg_num_colors);
    // lib::print_colors(pixels, args.flag_colors, &args.flag_delimiter, args.flag_rgb);
}
