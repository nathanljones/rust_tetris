#[derive(Clone, Copy)]
pub struct UCoordinate {
    pub x: u32,
    pub y: u32,
}
impl UCoordinate {
    #[must_use]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}