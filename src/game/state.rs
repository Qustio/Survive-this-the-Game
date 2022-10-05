pub struct State {
    pub hp: u8,
    pub water: u8,
    pub day: Option<u8>,
    max_day: u8,
    pub questions: Vec<Question>,
}

pub struct Question {
    pub text: &'static str,
    pub answers: (String, String),
    pub result: (fn(&mut State), fn(&mut State)),
}

impl State {
    pub fn new() -> Self {
        Self {
            hp: 255,
            water: 255,
            day: Some(0),
            max_day: 3,
            questions: vec![Question::new()],
        }
    }
}

impl Question {
    pub fn new() -> Question {
        Question {
            text: "Жили были три коровы одна из них заболела",
            answers: ("Да".to_owned(), "Нет".to_owned()),
            result: (
                |s: &mut State| s.water += 1,
                |s: &mut State| s.water -= 1,
            ),
        }
    }
}
