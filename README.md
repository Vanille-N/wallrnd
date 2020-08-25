# wallrnd
### A configurable generator of random abstract time-aware wallpapers

[![](https://img.shields.io/badge/github-Vanille--N/wallrnd-8da0cb?logo=github)](https://github.com/Vanille-N/wallrnd)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![crates.io](http://meritbadge.herokuapp.com/wallrnd)](https://crates.io/crates/wallrnd)
[![API](https://docs.rs/wallrnd/badge.svg)](https://docs.rs/wallrnd)

---
Direct dependencies

| dependency        | crate                              | docs                                  |
|-------------------|------------------------------------|---------------------------------------|
| `serde`           | [![][serde_cb]][serde_c]           | [![API][serde_db]][serde_d]           |
| `serde_derive`    | [![][derive_cb]][derive_c]         | [![API][derive_db]][derive_d]         |
| `rand`            | [![][rand_cb]][rand_c]             | [![API][rand_db]][rand_d]             |
| `chrono`          | [![][chrono_cb]][chrono_c]         | [![API][chrono_db]][chrono_d]         |
| `delaunator`      | [![][delaunator_cb]][delaunator_c] | [![API][delaunator_db]][delaunator_d] |
| `toml`            | [![][toml_cb]][toml_c]             | [![API][toml_db]][toml_d]             |
| `resvg`           | [![][resvg_cb]][resvg_c]           | [![API][resvg_db]][resvg_d]           |
| `usvg`            | [![][usvg_cb]][usvg_c]             | [![API][usvg_db]][usvg_d]             |
| `wallpaper_rs`    | [![][wallpaper_cb]][wallpaper_c]   | [![API][wallpaper_db]][wallpaper_d]   |
| `scrummage`       | [![][scrummage_cb]][scrummage_c]   | [![API][scrummage_db]][scrummage_d]   |

---

This project aims to provide a utility for generating random abstract wallpapers.

Until recently it could only generate the images, not actually set them as wallpapers. This limitation is being addressed.

A bash script to automatically change wallpaper is provided, and has been tested on Ubuntu 20.04 (Focal Fossa).
Portability of this script is outside of the scope of this project (the image generator however should be portable to any OS), but scripts that work for other shells/distros are welcome.


---

## Features and licensing

While the code in this crate is licensed under the [MIT license](https://opensource.org/licenses/MIT), the binary target includes (purely for user convenience) dependencies that have more restrictive licenses.

[resvg](https://crates.io/crates/resvg) and [usvg](https://crates.io/usvg) require the [MPL 2.0 license](https://opensource.org/licenses/MPL-2.0) and pull in some other dependencies under the [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause).

[wallpaper_rs](https://crates.io/crates/wallpaper_rs) is licensed under [GPL 3.0](https://opensource.org/licenses/GPL-3.0)

The features provided by these crates are purely optional. The different features available are explained in more detail in the [Advanced setup](#advanced-setup) section.

Using the `make-png` feature requires MPL 2.0 or a compatible license.

Using the `set-wallpaper` feature requires GPL 3.0 or a compatible license.

Using both requires GPL 3.0 at least.

The inclusion of MPL- and GPL-licensed crates as dependencies of this crate licensed under MIT does not grant to anyone the right to distribute executables that were compiled using the corresponding feature flags under non-GPL-compatible licenses. Any derivative work that does not include these flags can safely be provided under the MIT license.

It is not recommended for any crates dependent on this one to use the feature flags, as the functionality obtained from the GPL dependencies is not reexported by wallrnd and thus adds needless dependencies.

## Recommended setup (executable `wallrnd`)

* `cargo install wallrnd --features set-wallpaper,nice`

* Make a copy of `setup/wallrnd.toml` and adjust the settings to your liking

* Make a new Crontab entry: `* * * * * wallrnd --config /path/to/wallrnd.toml --image /tmp/wallpaper.svg --set --nice`

The `--nice` option causes `wallrnd` to lower its priority as soon as launched, which prevents the user from experiencing a short delay each time a new wallpaper is generated.

## Recommended setup (executable `prototype/prototype.py`)

`wallrnd` is a more developed product than this, but if you still want to use the prototype it is possible.

Be warned that performance is a lot worse and that this version is far less configurable.

* Edit `prototype/set-wallpaper` to your liking, make it executable, and add it to your `$PATH`.

* Make a new Crontab entry: `* * * * * set-wallpaper`

[`psutil`](https://pypi.org/project/psutil/) is used to abort the process if CPU usage is already high.


## Advanced setup

You may be interested in these other setup methods if
- your OS does not support setting an SVG image as wallpaper
- you do not wish to use GPL- or MPL-licensed products
- your OS is not included in [this list of supported environments](https://github.com/vineetred/flowy#supported-environments)
- you want custom functionality such as aborting the script when running on battery
- you want to build from source
- `scrummage` (the crate that `wallrnd` depends on to provide the `nice` feature) is not yet compatible with your OS

### Installation:
* If you do not need `wallrnd` to set wallpapers, then do not use the feature `set-wallaper`: `cargo install wallrnd`. The same is true if you don't want to use the `nice` feature.

* If you want to be able to create png images, then you should add the `make-png` feature: `cargo install wallrnd --features make-png`

* To have all features, you can use `cargo install wallrnd --features all`

* You can also build from source:
  ```
  git clone https://github.com/Vanille-N/wallrnd
  cd wallrnd
  cargo build --release --features nice,set-wallpaper
  cp target/release/wallrnd ~/bin/wallrnd
  ```

### Configuration
* `setup/wallrnd.toml` includes examples and explanations on how to setup options. Feel free to ask for more explanations.

* The configuration file doesn't have to be named `wallrnd.toml`, but it has to be formatted like a TOML file.

### Automation
* `setup/set-wallpaper-*` are examples of how to set wallrnd to be executed.

* The appropriate version should be put in your path and executed whenever necessary by adding an entry to your Crontab. `* * * * * set-wallpaper`

* Note that the file path does not have to be absolute. `wallrnd` resolves paths before writing the wallpaper to a file.

---

## Alternative tools

### Online

* [Random Wallpaper Generator!](http://bjmiller.net/canvas/wallpaper/)

* [Background Generator](https://bggenerator.com/)

### Scripts

* [flopp/RandomWallpapers](https://github.com/flopp/RandomWallpapers)

* [qryxip/sky-color-wallpaper](https://crates.io/crates/sky-color-wallpaper)

### Apps
* [Tapet](https://play.google.com/store/apps/details?id=com.sharpregion.tapet&hl=en_US)

## Examples

As a random generator of wallpaper ought to provide images of consistent quality, the following sample of images is **unfiltered**<a name="return-methodology">[\*](#methodology)</a>. All were created with a configuration file similar to the one provided under `setup/wallrnd.toml`.

![](docs/samples/image-1.svg)

![](docs/samples/image-2.svg)

![](docs/samples/image-3.svg)

![](docs/samples/image-4.svg)

![](docs/samples/image-5.svg)

![](docs/samples/image-6.svg)


<a name="methodology">[\*](#return-methodology)</a> To provide a variety of patterns, tilings, and themes, the six were created in succession by altering the configuration file slightly so that only one pattern, tiling, and theme was available. This method guarantees variability without biasing quality. Hence the above sample can be considered representative of the general quality of generated wallpapers.

<!-- Links section -->
[serde_c]: https://crates.io/crates/serde
[serde_cb]: https://meritbadge.herokuapp.com/serde
[serde_d]: https://docs.rs/serde
[serde_db]: https://docs.rs/serde/badge.svg

[derive_c]: https://crates.io/crates/serde_derive
[derive_cb]: https://meritbadge.herokuapp.com/serde_derive
[derive_d]: https://docs.rs/serde_derive
[derive_db]: https://docs.rs/serde_derive/badge.svg

[rand_c]: https://crates.io/crates/rand
[rand_cb]: https://meritbadge.herokuapp.com/rand
[rand_d]: https://docs.rs/rand
[rand_db]: https://docs.rs/rand/badge.svg

[toml_c]: https://crates.io/crates/toml
[toml_cb]: https://meritbadge.herokuapp.com/toml
[toml_d]: https://docs.rs/toml
[toml_db]: https://docs.rs/toml/badge.svg

[chrono_c]: https://crates.io/crates/chrono
[chrono_cb]: https://meritbadge.herokuapp.com/chrono
[chrono_d]: https://docs.rs/chrono
[chrono_db]: https://docs.rs/chrono/badge.svg

[delaunator_c]: https://crates.io/crates/delaunator
[delaunator_cb]: https://meritbadge.herokuapp.com/delaunator
[delaunator_d]: https://docs.rs/delaunator
[delaunator_db]: https://docs.rs/delaunator/badge.svg

[resvg_c]: https://crates.io/crates/resvg
[resvg_cb]: https://meritbadge.herokuapp.com/resvg
[resvg_d]: https://docs.rs/resvg
[resvg_db]: https://docs.rs/resvg/badge.svg

[usvg_c]: https://crates.io/crates/usvg
[usvg_cb]: https://meritbadge.herokuapp.com/usvg
[usvg_d]: https://docs.rs/usvg
[usvg_db]: https://docs.rs/usvg/badge.svg

[wallpaper_c]: https://crates.io/crates/wallpaper_rs
[wallpaper_cb]: https://meritbadge.herokuapp.com/wallpaper_rs
[wallpaper_d]: https://docs.rs/wallpaper_rs
[wallpaper_db]: https://docs.rs/wallpaper_rs/badge.svg

[scrummage_c]: https://crates.io/crates/scrummage
[scrummage_cb]: https://meritbadge.herokuapp.com/scrummage
[scrummage_d]: https://docs.rs/scrummage
[scrummage_db]: https://docs.rs/scrummage/badge.svg
