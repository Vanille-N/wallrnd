use std::collections::HashSet;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

fn main() {
    let frame = Frame {
        x: 0,
        y: 0,
        w: 1000,
        h: 600,
    };
    let mut document = Document::new().set("viewBox", frame.into_tuple());

    for elem in frame.triangle_fill(Triangle { size: 14., rot: 15 }) {
        document = document.add(elem)
    }

    svg::save("image.svg", &document).unwrap();
}
