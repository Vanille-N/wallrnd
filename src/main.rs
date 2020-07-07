use svg::Document;
use wallrnd::color::Color;
use wallrnd::scene::Scene;
use wallrnd::cfg::*;
use wallrnd::tesselation::*;

fn main() {
    let mut rng = rand::thread_rng();
    let frame = Frame {
        x: 0,
        y: 0,
        w: 1000,
        h: 600,
    };

    let cfg = SceneCfg {
        deviation: 20,
        weight: 40,
        themes: vec![Color(50, 50, 50), Color(100, 0, 0), Color(0, 100, 0)],
        frame,
        tiling: Tiling::HexagonsAndTriangles,
        pattern: Pattern::FreeCircles,
    };

    let scene = Scene::new(&cfg, &mut rng);
    let stroke = Color(0, 0, 0).to_string();

    let mut document = Document::new().set("viewBox", cfg.frame.into_tuple());
    for (pos, elem) in tile_hybrid_squares_triangles(&cfg.frame, 15., 50) {
        let fill = scene.color(pos, &mut rng);
        document = document.add(
            elem.set("fill", fill.to_string())
                .set("stroke", &stroke[..]),
        );
    }

    svg::save("image.svg", &document).unwrap();
}
