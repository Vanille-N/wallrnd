use svg::Document;
use wallrnd::shapes::*;
use wallrnd::tesselation::*;

fn main() {
    let frame = Frame {
        x: 0,
        y: 0,
        w: 1000,
        h: 600,
    };
    let mut document = Document::new().set("viewBox", frame.into_tuple());

    for elem in tile_triangles(&frame, Triangle { size: 14., rot: 15 }) {
        document = document.add(elem)
    }

    svg::save("image.svg", &document).unwrap();
}
