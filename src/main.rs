use chrono::{Local, Timelike};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use wallrnd::deserializer::MetaConfig;
use wallrnd::scene::Scene;
use wallrnd::svg::*;
use wallrnd::prelude::*;

fn main() {
    let args = read_command_line_arguments();

    if args.help {
        print_help();
        std::process::exit(0);
    }
    let verbose = args.verbose;

    if args.init != "" {
        if verbose.prog {
            println!("Initializing configuration file");
        }
        make_config_file(&args.init[..]);
        std::process::exit(0);
    }

    // Get local time and convert to app-specific format: HHMM
    if verbose.prog {
        println!("Reading time");
    }
    let time = args.time.unwrap_or_else(|| {
        let now = Local::now();
        let h = now.hour();
        let m = now.minute();
        let current = (h * 100 + m) as usize;
        if verbose.info {
            println!("Using current time: {}", current);
        }
        current
    });
    let dest = args.image;
    let fname = args.config;

    if verbose.prog {
        println!("Creating random number generator");
    }
    let mut rng = rand::thread_rng();
    if verbose.prog {
        println!("Attempting to open configuration file");
    }
    let cfg_file = File::open(fname);
    let mut cfg_contents = String::new();
    if let Ok(mut f) = cfg_file {
        if let Err(e) = f.read_to_string(&mut cfg_contents) {
            if verbose.warn {
                println!("{}; Switching to default settings.", e);
            }
        }
    } else if verbose.warn {
        println!("Settings file not found");
    }
    if verbose.prog {
        println!("Choosing random settings according to configuration");
    }
    let cfg = MetaConfig::from_string(cfg_contents, verbose).pick_cfg(&mut rng, time, verbose);

    if verbose.prog {
        println!("Building scene");
    }
    let scene = Scene::new(&cfg, &mut rng);
    let stroke = cfg.line_color;
    let stroke_width = cfg.line_width;
    let stroke_like_fill = stroke_width < 0.0001;

    // Generate document
    if verbose.prog {
        println!("Creating tiling");
    }
    let mut document = Document::new(cfg.frame);
    for (pos, elem) in cfg.make_tiling(&mut rng) {
        let fill = scene.color(pos, &mut rng);
        document.add(
            elem.with_fill_color(fill)
                .with_stroke_color(if stroke_like_fill { fill } else { stroke })
                .with_stroke_width(stroke_width),
        );
    }

    if verbose.prog {
        println!("Writing image to file");
    }
    document.save(dest).unwrap_or_else(|_| {
        if verbose.warn {
            println!("No valid destination specified");
        }
        std::process::exit(1);
    });
    if verbose.prog {
        println!("Process exited successfully");
    }
}

#[derive(Default)]
struct Args {
    help: bool,
    log: bool,
    verbose: Verbosity,
    time: Option<usize>,
    image: String,
    config: String,
    init: String,
}

fn read_command_line_arguments() -> Args {
    let mut args = Args::default();
    let mut it = env::args().collect::<Vec<_>>().into_iter().skip(1);
    loop {
        match it.next().as_deref() {
            None => return args,
            Some("--help") => args.help = true,
            Some("--log") => args.log = true,
            Some("--verbose") => args.verbose = Verbosity::from(&it.next().unwrap_or_else(|| panic!("Option --verbose should be followed by a verbosity descriptor: '^[PDIWA]*$',
P: Progress
D: Details
I: Info
W: Warnings
A: All"))[..]),
            Some("--init") => {
                args.init = it
                    .next()
                    .unwrap_or_else(|| panic!("Option --init should be followed by a source file"))
                    .to_string()
            }
            Some("--time") => {
                args.time = Some(
                    it.next()
                        .unwrap_or_else(|| {
                            panic!("Option --time should be followed by a timestamp.")
                        })
                        .parse()
                        .unwrap_or_else(|e| panic!("Failed to parse time: {}", e)),
                )
            }
            Some("--image") => {
                args.image = it
                    .next()
                    .unwrap_or_else(|| {
                        panic!("Option --image should be followed by a destination file")
                    })
                    .to_string()
            }
            Some("--config") => {
                args.config = it
                    .next()
                    .unwrap_or_else(|| {
                        panic!("Option --config should be followed by a source file")
                    })
                    .to_string()
            }
            Some(o) => panic!("Unknown option {}", o),
        }
    }
}

fn print_help() {
    print!(include_str!("../assets/man"));
}

fn make_config_file(fname: &str) {
    let mut buffer = std::fs::File::create(fname).unwrap_or_else(|e| {
        println!("Error creating configuration: {}", e);
        std::process::exit(2);
    });
    let sample_cfg = include_str!("../setup/wallrnd.toml");
    buffer
        .write_all(&sample_cfg.to_string().into_bytes())
        .unwrap_or_else(|e| {
            println!("Error writing configuration: {}", e);
            std::process::exit(3);
        });
}
