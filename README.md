# wallrnd
### A configurable generator of random abstract time-aware wallpapers

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

It is only intended for **generating** the images (SVG), not actually setting them as a wallpaper.

A bash script to automatically change wallpaper is provided, and has been tested on Ubuntu 20.04 (Focal Fossa).
Portability of this script is outside of the scope of this project (the image generator however should be portable to any OS), but scripts that work for other shells/distros are welcome.


---

### Recommended setup (executable `wallrnd`)

* `cargo build --release` to create the `wallrnd` executable

* Put `wallrnd` and `setup/wallrnd.toml` in a directory `$DIR`

* Edit accordingly the `set-wallpaper` script, make it executable, and add it to a folder in your `$PATH`

* Adjust the colors, themes, shapes in `wallrnd.toml` to your liking

* Make a new Crontab entry: `* * * * * set-wallpaper`

Performance is good enough that running it every 60 seconds is not even noticeable (4-core Intel i5).

Thanks to [`nice`](https://en.wikipedia.org/wiki/Nice_(Unix)) giving the program a very low priority, it can run in the background without issue. The script also exits early when running on battery.

### Recommended setup (executable `prototype.py`)

`wallrnd` is a more developed product than this, but if you still want to use the prototype it is possible.

Be warned that performance is a lot worse and that this version is far less configurable.

* Edit `set-wallpaper` to your liking, make it executable, and add it to your `$PATH`.

* Make a new Crontab entry: `* * * * * set-wallpaper`

[`psutil`](https://pypi.org/project/psutil/) is used to abort the process if CPU usage is already high.

---

### Alternative tools

#### Online

* [Random Wallpaper Generator!](http://bjmiller.net/canvas/wallpaper/)

* [Background Generator](https://bggenerator.com/)

#### Scripts

* [flopp/RandomWallpapers](https://github.com/flopp/RandomWallpapers)

### Examples

As a random generator of wallpaper ought to provide images of consistent quality, the following sample of images is **unfiltered**\*. All were created with a configuration file similar to the one provided under `setup/wallrnd.toml`.

<img src=samples/image-1.svg width=500> <img src=samples/image-2.svg width=500><br><br>
<img src=samples/image-3.svg width=500> <img src=samples/image-4.svg width=500><br><br>
<img src=samples/image-5.svg width=500> <img src=samples/image-6.svg width=500><br><br>


\* To provide a variety of patterns, tilings, and themes, the six were created in succession by altering the configuration file slightly so that only one pattern, tiling, and theme was available. This method guarantees variability without biasing quality. Hence the above sample can be considered representative of the general quality of generated wallpapers.
