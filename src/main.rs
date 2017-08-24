
use std::env;

extern crate image;

mod lib;
use lib::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    fetchColors(filename, 5);
}
