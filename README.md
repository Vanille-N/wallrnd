# wallrnd

[![](https://img.shields.io/badge/github-Vanille--N/wallrnd-8da0cb?logo=github)](https://github.com/Vanille-N/wallrnd)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---
Direct dependencies

`serde` [![](http://meritbadge.herokuapp.com/serde)](https://crates.io/crates/serde)
[![API](https://docs.rs/serde/badge.svg)](https://docs.rs/serde)

`serde_derive` [![](http://meritbadge.herokuapp.com/serde_derive)](https://crates.io/crates/serde_derive)
[![API](https://docs.rs/serde_derive/badge.svg)](https://docs.rs/serde_derive)

`svg` [![](http://meritbadge.herokuapp.com/svg)](https://crates.io/crates/svg)
[![API](https://docs.rs/svg/badge.svg)](https://docs.rs/svg)

`rand` [![](http://meritbadge.herokuapp.com/rand)](https://crates.io/crates/rand)
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/rand)

`toml` [![](http://meritbadge.herokuapp.com/toml)](https://crates.io/crates/toml)
[![API](https://docs.rs/toml/badge.svg)](https://docs.rs/toml)

`chrono` [![](http://meritbadge.herokuapp.com/chrono)](https://crates.io/crates/chrono)
[![API](https://docs.rs/chrono/badge.svg)](https://docs.rs/chrono)

`delaunator` [![](http://meritbadge.herokuapp.com/delaunator)](https://crates.io/crates/delaunator)
[![API](https://docs.rs/delaunator/badge.svg)](https://docs.rs/delaunator)

---

This project aims to provide a utility for generating random abstract wallpapers.

It is only intended for *generating* the image (SVG), not actually setting it as a wallpaper.

A bash script is provided that has been tested on Ubuntu 20.04 (Focal Fossa).
Portability of this script is outside of the scope of this project (the image generator however should be portable to any OS), but scripts that work for other shells/distros are welcome.

### Recommended setup (executable `prototype.py`)

* Edit `set-wallpaper` to your liking, make it executable, and add it to your `$PATH`.

* Make a new Crontab entry: `* * * * * set-wallpaper`

This will generate a new image every minute and set it as wallpaper. Note that even having the program run every 60 seconds is not a problem, thanks to [`nice`](https://en.wikipedia.org/wiki/Nice_(Unix)) giving it a low priority and [`psutil`](https://pypi.org/project/psutil/) being used to abort the process if CPU usage is already high.

### Recommended setup (executable `wallrnd`)

* `cargo build --release` to create the `wallrnd` executable

* Put `wallrnd` and `wallrnd.toml` in a directory `$DIR`

* Edit accordingly the `set-wallpaper` script, make it executable, and add it to a folder in your `$PATH`

* Adjust the colors, themes, shapes in `wallrnd.toml`

* Make a new Crontab entry: `* * * * * set-wallpaper`

### Alternative tools

#### Online

* [Random Wallpaper Generator!](http://bjmiller.net/canvas/wallpaper/)

* [Background Generator](https://bggenerator.com/)

#### Scripts

* [flopp/RandomWallpapers](https://github.com/flopp/RandomWallpapers)


### Examples

As a random generator of wallpaper ought to provide images of consistent quality, the following sample of images is **unfiltered**. All were created with a configuration file similar to the one provided under `setup/wallrnd.toml`.

<img src=samples/fire-pstr.svg width=400> <img src=samples/forest-cstr.svg width=400><br><br>
<img src=samples/night-fstr.svg width=400> <img src=samples/rust-cstr.svg width=400><br><br>
<img src=samples/sky-ccir.svg width=400> <img src=samples/spring-ccir.svg width=400><br><br>
