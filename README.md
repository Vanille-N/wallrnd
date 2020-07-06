# wallrnd

[![](https://img.shields.io/badge/github-Vanille--N/wallrnd-8da0cb?logo=github)](https://github.com/Vanille-N/wallrnd)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This project aims to provide a utility for generating random abstract wallpapers.

It is only intended for *generating* the image (SVG), not actually setting it as a wallpaper.

A bash script is provided that has been tested on Ubuntu 20.04 (Focal Fossa).
Portability of this script is outside of the scope of this project (the image generator however should be portable to any OS), but scripts that work for other shells/distros are welcome.

### Recommended setup (executable `prototype.py`)

* Edit `set-wallpaper` to your liking, make it executable, and add it to your `$PATH`.

* Make a new Crontab entry: `* * * * * set-wallpaper`

This will generate a new image every minute and set it as wallpaper. Note that even having the program run every 60 seconds is not a problem, thanks to [`nice`](https://en.wikipedia.org/wiki/Nice_(Unix)) giving it a low priority and [`psutil`](https://pypi.org/project/psutil/) being used to abort the process if CPU usage is already high.

### Recommended setup (executable `wallrnd`)

*Work in progress*

### Alternative tools

#### Online

* [Random Wallpaper Generator!](http://bjmiller.net/canvas/wallpaper/)

* [Background Generator](https://bggenerator.com/)

#### Scripts

* [flopp/RandomWallpapers](https://github.com/flopp/RandomWallpapers)
