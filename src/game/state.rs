pub struct State {
    pub hp: u8,
    pub water: u8,
    pub day: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            hp: 255,
            water: 255,
            day: 0,
        }
    }
}
