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

#[derive(Clone, Copy)]

pub struct ICoordinate {
    pub x: i32,
    pub y: i32,
}
impl ICoordinate {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
