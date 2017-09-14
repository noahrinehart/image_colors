# image_colors [![Crates.io version shield](https://img.shields.io/crates/v/image_colors.svg)](https://crates.io/crates/image_colors) [![Travis-CI build status](https://travis-ci.org/noahrinehart/image_colors.svg?branch=master)](https://travis-ci.org/noahrinehart/image_colors/)
Get colors from an image. Optionally sort by most common.

--------
### Usage
```bash
Image Colors 0.4.3
Noah Rinehart <rinehart.noah@gmail.com>
Gets colors from an image. Optionally sort by most common.

USAGE:
    image_colors [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -c               Show colors in terminal
    -h, --help       Prints help information
    -r               Displays color in rgb instead of hex
    -V, --version    Prints version information

OPTIONS:
    -l, --delim <DELIMITER>    Sets output delimiter [default:  has a pixel count of: ]
    -d <DEPTH>                 Sets depth of search

ARGS:
    <PATH>          Path to the image
    <NUM_COLORS>    Number of colors to return
```

### Example
![Example pic](http://i.imgur.com/1slGf0R.png)
