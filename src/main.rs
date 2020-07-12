use chrono::{Local, Timelike};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use svg::Document;
use wallrnd::color::Color;
use wallrnd::optionfmt::MetaConfig;
use wallrnd::scene::Scene;

fn main() {
    let args = env::args().collect::<Vec<_>>();
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
    let stroke = Color(0, 0, 0).to_string();

    let mut document = Document::new().set("viewBox", cfg.frame.into_tuple());
    for (pos, elem) in cfg.make_tiling(&mut rng) {
        let fill = scene.color(pos, &mut rng);
        document = document.add(
            elem.set("fill", fill.to_string())
                .set("stroke", &stroke[..]),
        );
    }

    svg::save(dest, &document).unwrap();
}
