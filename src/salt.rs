struct SaltItem {
    color: Color,
    likeliness: f64,
    variability: usize,
}

pub struct Salt(Vec<SaltItem>);
