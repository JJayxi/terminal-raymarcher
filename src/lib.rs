pub struct Screen {
    pub width: u32,
    pub height: u32,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        Screen { width, height }
    }
}
