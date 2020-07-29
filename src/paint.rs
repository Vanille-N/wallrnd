use crate::cfg::SceneCfg;
use crate::prelude::*;
use crate::scene::*;
use rand::{rngs::ThreadRng, Rng};

pub fn create_free_circles(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<Disc> {
    let mut items = Vec::new();
    for i in 1..=cfg.nb_pattern {
        let c = cfg.choose_color(rng);
        items.push(Disc::random(
            rng,
            &cfg.frame,
            c,
            i as f64 / cfg.nb_pattern as f64 * 0.5,
        ));
    }
    items.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());
    if verbose.details {
        println!("{:#?}", items);
    }
    items
}

pub fn create_free_triangles(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<Triangle> {
    let mut items = Vec::new();
    for i in 1..=cfg.nb_pattern {
        let c = cfg.choose_color(rng);
        items.push(Disc::random(
            rng,
            &cfg.frame,
            c,
            i as f64 / cfg.nb_pattern as f64 * 0.7,
        ));
    }
    items.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());
    let items = items
        .into_iter()
        .map(|d| Triangle::random(rng, d))
        .collect::<Vec<_>>();
        if verbose.details {
            println!("{:#?}", items);
        }
        items
}

pub fn create_free_stripes(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<Stripe> {
    let mut items = Vec::new();
    for _ in 0..cfg.nb_pattern {
        let c = cfg.choose_color(rng);
        let w = cfg.width_pattern * cfg.frame.h as f64 * (rng.gen::<f64>() + 0.5);
        items.push(Stripe::random(rng, &cfg.frame, c, w));
    }
    if verbose.details {
        println!("{:#?}", items);
    }
    items
}

pub fn create_free_spirals(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<Spiral> {
    let mut items = Vec::new();
    for _ in 0..cfg.nb_pattern {
        let c = cfg.choose_color(rng);
        let w = cfg.width_pattern * cfg.frame.h as f64 * (rng.gen::<f64>() + 0.5);
        items.push(Spiral::random(rng, &cfg.frame, c, w, cfg.tightness_spiral));
    }
    items.sort_by(|a, b| a.width.partial_cmp(&b.width).unwrap());
    if verbose.details {
        println!("{:#?}", items);
    }
    items
}

pub fn create_concentric_circles(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<Disc> {
    let mut items = Vec::new();
    let center = Pos::random(&cfg.frame, rng);
    let d = center
        .dist(Pos(0., 0.))
        .max(center.dist(Pos(0., cfg.frame.w as f64)))
        .max(center.dist(Pos(cfg.frame.h as f64, 0.)))
        .max(center.dist(Pos(cfg.frame.h as f64, cfg.frame.w as f64)));
    for i in 1..=cfg.nb_pattern {
        items.push(Disc {
            center,
            radius: d * i as f64 / cfg.nb_pattern as f64,
            color: cfg.choose_color(rng),
        })
    }
    items.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());
    if verbose.details {
        println!("{:#?}", items);
    }
    items
}

pub fn create_parallel_stripes(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<HalfPlane> {
    let mut items = Vec::new();
    let (a, b, dir) = {
        let c = cfg.frame.center();
        let w = cfg.frame.h + cfg.frame.w;
        let dir = rng.gen_range(0, 360);
        let d = Pos::polar(dir, w as f64 / 2.);
        (c + d, c - d, dir)
    };
    for i in 0..=cfg.nb_pattern {
        let c = cfg.choose_color(rng);
        let p = i as f64 / cfg.nb_pattern as f64;
        items.push(HalfPlane::random(
            rng,
            a * (1. - p) + b * p,
            180 + dir,
            cfg.var_stripes,
            c,
        ));
    }
    if verbose.details {
        println!("{:#?}", items);
    }
    items
}

pub fn create_crossed_stripes(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<HalfPlane> {
    let mut items = Vec::new();
    let (a, b, a_orth, b_orth, dir) = {
        let c = cfg.frame.center();
        let w = cfg.frame.h + cfg.frame.w;
        let dir = rng.gen_range(0, 360);
        let d = Pos::polar(dir, w as f64 / 2.);
        let d_orth = Pos::polar(dir + 90, w as f64 / 2.);
        (c + d, c - d, c - d_orth, c + d_orth, dir)
    };
    for i in 0..=cfg.nb_pattern {
        let p = i as f64 / cfg.nb_pattern as f64;
        let c = cfg.choose_color(rng);
        items.push(HalfPlane::random(
            rng,
            a * (1. - p) + b * p,
            180 + dir,
            cfg.var_stripes,
            c,
        ));
        let c = cfg.choose_color(rng);
        items.push(HalfPlane::random(
            rng,
            a_orth * (1. - p) + b_orth * p,
            90 + dir,
            cfg.var_stripes,
            c,
        ));
    }
    if verbose.details {
        println!("{:#?}", items);
    }
    items
}

pub fn create_waves(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<Wave> {
    let mut items = Vec::new();
    let (a, b, dir) = {
        let c = cfg.frame.center();
        let w = cfg.frame.h + cfg.frame.w;
        let dir = rng.gen_range(0, 360);
        let d = Pos::polar(dir, w as f64 / 2.);
        (c + d, c - d, dir)
    };
    let amplitude = (b - a).norm() / cfg.nb_pattern as f64 / 2.;
    for i in 0..=cfg.nb_pattern {
        let c = cfg.choose_color(rng);
        let p = i as f64 / cfg.nb_pattern as f64;
        items.push(Wave::random(
            rng,
            a * (1. - p) + b * p,
            180 + dir,
            cfg.width_pattern / 5.,
            amplitude,
            c,
        ));
    }
    if verbose.details {
        println!("{:#?}", items);
    }
    items
}

pub fn create_sawteeth(rng: &mut ThreadRng, cfg: &SceneCfg, verbose: Verbosity) -> Vec<Sawtooth> {
    let mut items = Vec::new();
    let (a, b, dir) = {
        let c = cfg.frame.center();
        let w = cfg.frame.h + cfg.frame.w;
        let dir = rng.gen_range(0, 360);
        let d = Pos::polar(dir, w as f64 / 2.);
        (c + d, c - d, dir)
    };
    let amplitude = (b - a).norm() / cfg.nb_pattern as f64 / 2.;
    for i in 0..=cfg.nb_pattern {
        let c = cfg.choose_color(rng);
        let p = i as f64 / cfg.nb_pattern as f64;
        items.push(Sawtooth::random(
            rng,
            a * (1. - p) + b * p,
            180 + dir,
            cfg.width_pattern / 5.,
            amplitude,
            c,
        ));
    }
    if verbose.details {
        println!("{:#?}", items);
    }
    items
}
