<link rel="shortcut icon" type="image/png" href="profile.ico?">

<div class="css-slideshow">
    <figure>
        <img src="samples/image-1.svg" width="950" height="500" class="alignnone size-full wp-image-172" />
        <figcaption><strong>Rust</strong> (Parallel stripes -- Delaunay)</figcaption>
    </figure>
    <figure>
        <img src="samples/image-2.svg" width="950" height="500" class="alignnone size-full wp-image-179" />
        <figcaption><strong>Ocean</strong> (Parallel waves -- Squares and triangles</figcaption>
    </figure>
    <figure>
        <img src="samples/image-3.svg" width="950" height="500" class="alignnone size-large wp-image-178" />
        <figcaption><strong>Fire</strong> (Free circles -- Hexagons and triangles)</figcaption>
    </figure>
    <figure>
        <img src="samples/image-4.svg" width="950" height="500" class="alignnone size-full wp-image-177" />
        <figcaption><strong>Forest</strong> (Free triangles -- Triangles)</figcaption>
    </figure>
    <figure>
        <img src="samples/image-5.svg" width="950" height="500" class="alignnone size-large wp-image-176" />
        <figcaption><strong>Sky</strong> (Concentric circles -- Hexagons)</figcaption>
    </figure>
    <figure>
        <img src="samples/image-6.svg" width="950" height="500" class="alignnone size-large wp-image-175" />
        <figcaption><strong>Blood</strong> (Crossed stripes -- Hexagons and triangles)</figcaption>
    </figure>
  </div>
<p class="css-slideshow-attr"><a href="http://www.w3.org/html/logo/" target="_top">All images are generated as SVG</a></p>

---

<div>
    <p style="float: left;"><a href="https://www.rust-lang.org"><img src="https://www.rust-lang.org/logos/rust-logo-64x64.png"></a></p>
    <p>This project is written in pure Rust and aims to provide a cross-platform utility for generating random abstract wallpapers. It is fast and memory-efficient enough to be able to run in the background at regular intervals</p>
</div>
<br>
<div>
    <p style="float: left;"><a href="https://www.github.com/Vanille-N/wallrnd"><img src="https://github.githubassets.com/images/modules/logos_page/Octocat.png" width="64"></a></p>
    <p>The full source code is hosted on GitHub, and so is this website. Contributions in any form (pull requests, feature requests, bug reports, etc...) are welcome.</p>
</div>
<br>

---

# How to install

Clone the repository and `cd` inside.

Use `cargo build --release` (1.44 stable) to create the `wallrnd` executable

Run `wallrnd path/to/image.svg path/to/configuration.toml` to create a new wallpaper.

A configuration file is provided under `/setup/wallrnd.toml`.

To generate wallpapers at regular intervals, you can create a new cronjob that calls `wallrnd`. Examples of this are available in `/setup`.

---

# Alternative tools

#### Online

* [Random Wallpaper Generator!](http://bjmiller.net/canvas/wallpaper/)

* [Background Generator](https://bggenerator.com/)

#### Scripts

* [flopp/RandomWallpapers](https://github.com/flopp/RandomWallpapers)

#### Apps
* [Tapet](https://play.google.com/store/apps/details?id=com.sharpregion.tapet&hl=en_US)

Do you know of another similar tool ? You can suggest it [here](https://github.com/Vanille-N/wallrnd/issues)
