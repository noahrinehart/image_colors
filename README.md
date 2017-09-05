# image_colors [![Crates.io version shield](https://img.shields.io/crates/v/image_colors.svg)](https://crates.io/crates/image_colors) [![Travis-CI build status](https://travis-ci.org/noahrinehart/image_colors.svg?branch=master)](https://travis-ci.org/noahrinehart/image_colors/)
Get colors from an image. Optionally sort by most common.

--------
### Usage
```bash
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
```

### Example
![Example pic](http://i.imgur.com/1slGf0R.png)
