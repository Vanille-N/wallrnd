use std::fs::File;
use std::io::prelude::*;
use svg::Document;
use wallrnd::cfg::*;
use wallrnd::color::Color;
use wallrnd::deserializer::MetaConfig;
use wallrnd::scene::Scene;
use wallrnd::tesselation::Frame;

fn main() {
    let mut rng = rand::thread_rng();
    let mut cfg_file = File::open("wallrnd.toml").unwrap();
    let mut cfg_contents = String::new();
    cfg_file.read_to_string(&mut cfg_contents).unwrap();
    let cfg = MetaConfig::from_string(cfg_contents).pick_cfg(&mut rng);

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

    svg::save("image.svg", &document).unwrap();
}
