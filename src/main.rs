use svg::Document;
use wallrnd::color::Color;
use wallrnd::scene::Scene;
use wallrnd::tesselation::*;

fn main() {
    let frame = Frame {
        x: 0,
        y: 0,
        w: 1000,
        h: 600,
    };
    let mut document = Document::new().set("viewBox", frame.into_tuple());
    let scene = Scene::new();
    let stroke = Color(0, 0, 0).to_string();

    for (pos, elem) in random_delaunay(&frame, &mut rand::thread_rng()) {
        let fill = scene.color(pos);
        document = document.add(
            elem.set("fill", fill.to_string())
                .set("stroke", &stroke[..]),
        );
    }

    svg::save("image.svg", &document).unwrap();
}
