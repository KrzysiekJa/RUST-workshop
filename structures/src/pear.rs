pub enum Color {
    Red,
    Green,
    Yellow,
    Orange
}
pub enum Taste {
    Sweet,
    Sour,
    Dry
}
pub struct Pear {
    variety: String,
    color: Color,
    weight: f32,
    size: f32,
    taste: Taste
}
