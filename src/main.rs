use svg::Document;
use wallrnd::color::Color;
use wallrnd::scene::Scene;
use wallrnd::cfg::*;
use wallrnd::tesselation::Frame;

fn main() {
    let mut rng = rand::thread_rng();
    let frame = Frame {
        x: 0,
        y: 0,
        w: 1000,
        h: 600,
    };

    let cfg = SceneCfg {
        deviation: 30,
        weight: 40,
        themes: vec![Color(50, 50, 50), Color(100, 0, 0), Color(0, 100, 0)],
        frame,
        tiling: Tiling::SquaresAndTriangles,
        pattern: Pattern::FreeSpirals,
        nb_concentric_circles: 5,
        nb_free_circles: 10,
        nb_free_spirals: 3,
        nb_free_stripes: 10,
        nb_free_triangles: 10,
        nb_crossed_stripes: 7,
        nb_parallel_stripes: 15,
        var_parallel_stripes: 10,
        delaunay_count: 1000,
        tiling_size: 10.,
        stripe_width: 0.1,
        spiral_width: 0.3,
    };

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
