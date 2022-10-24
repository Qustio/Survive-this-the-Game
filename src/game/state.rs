pub struct State {
    pub hp: (u8, u8),
    pub saturation: (u8, u8),
    pub water: (u8, u8),
    pub day: (u8, u8),
}

impl State {
    pub fn new() -> Self {
        Self {
            hp: (10, 10),
            saturation: (5, 5),
            water: (3, 3),
            day: (0, 5),
        }
    }
}