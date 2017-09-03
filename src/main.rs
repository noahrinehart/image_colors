
use std::env;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 3 {
        panic!("Not enough or too many arguments");
    }
    let filename = &args[1];
    let num_colors = args[2].parse::<usize>().expect("Expected an integer");

    let pixels = lib::fetch_colors(filename, num_colors);
    lib::print_colors(pixels, true, Some(", ".to_string()));
}
