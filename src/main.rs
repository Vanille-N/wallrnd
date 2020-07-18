use chrono::{Local, Timelike};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use wallrnd::deserializer::MetaConfig;
use wallrnd::scene::Scene;
use wallrnd::svg::*;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if let Some("-help") = args.get(1).map(String::as_str) {
        println!("usage: wallrnd path/to/image.svg path/to/configuration.toml");
        std::process::exit(0);
    }

    // Read command line arguments other than -help
    let dest = if args.len() > 1 {
        args[1].clone()
    } else {
        println!("No destination specified, saving to /tmp/wallpaper-random.svg");
        String::from("/tmp/wallpaper-random.svg")
    };
    let fname = if args.len() > 2 {
        args[2].clone()
    } else {
        String::from("")
    };

    // Get local time and convert to app-specific format: HHMM
    let time = {
        let now = Local::now();
        let h = now.hour();
        let m = now.minute();
        (h * 100 + m) as usize
    };

    let mut rng = rand::thread_rng();
    let cfg_file = File::open(fname);
    let mut cfg_contents = String::new();
    if let Ok(mut f) = cfg_file {
        if let Err(e) = f.read_to_string(&mut cfg_contents) {
            println!("{}; Switching to default settings.", e);
        }
    } else {
        println!("Settings file not found");
    }
    let cfg = MetaConfig::from_string(cfg_contents).pick_cfg(&mut rng, time);

    let scene = Scene::new(&cfg, &mut rng);
    let stroke = cfg.line_color;
    let stroke_width = cfg.line_width;
    let stroke_like_fill = stroke_width < 0.0001;


    // Generate document
    let mut document = Document::new(cfg.frame);
    for (pos, elem) in cfg.make_tiling(&mut rng) {
        let fill = scene.color(pos, &mut rng);
        document.add(
            elem.with_fill_color(fill)
                .with_stroke_color(if stroke_like_fill { fill } else { stroke })
                .with_stroke_width(stroke_width),
        );
    }

    document.save(dest).unwrap();
}
