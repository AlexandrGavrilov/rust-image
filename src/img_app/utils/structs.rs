pub struct Crop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Crop {
    pub fn new(x:u32, y:u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height
        }
    }
}