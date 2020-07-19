use chrono::{Local, Timelike};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use wallrnd::deserializer::MetaConfig;
use wallrnd::scene::Scene;
use wallrnd::svg::*;

fn main() {
    let args = read_command_line_arguments();

    // Get local time and convert to app-specific format: HHMM
    let time = args.time.unwrap_or_else(|| {
        let now = Local::now();
        let h = now.hour();
        let m = now.minute();
        (h * 100 + m) as usize
    });
    let dest = args.image;
    let fname = args.config;

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

#[derive(Default)]
struct Args {
    help: bool,
    log: bool,
    verbose: bool,
    time: Option<usize>,
    image: String,
    config: String,
}

fn read_command_line_arguments() -> Args {
    let mut args = Args::default();
    let mut it = env::args().collect::<Vec<_>>().into_iter().skip(1);
    loop {
        match it.next().as_deref() {
            None => return args,
            Some("--help") => args.help = true,
            Some("--log") => args.log = true,
            Some("--verbose") => args.verbose = true,
            Some("--time") => args.time = Some(it.next().unwrap_or_else(|| panic!("Option --time should be followed by a timestamp.")).parse().unwrap_or_else(|e| panic!("Failed to parse time: {}", e))),
            Some("--image") => args.image = it.next().unwrap_or_else(|| panic!("Option --image should be followed by a destination file")).to_string(),
            Some("--config") => args.config = it.next().unwrap_or_else(|| panic!("Option --config should be followed by a source file")).to_string(),
            Some(o) => panic!("Unknown option {}", o),
        }
    }
}
