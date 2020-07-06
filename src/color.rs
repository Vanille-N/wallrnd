#[derive(Clone, Copy, Debug)]
pub struct Color(pub i32, pub i32, pub i32);

impl Color {
    pub fn to_string(self) -> String {
        format!("rgb({},{},{})", self.0, self.1, self.2)
    }
}
