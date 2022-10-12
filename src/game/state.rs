pub struct State {
    pub hp: (u8, u8),
    pub saturation: (u8, u8),
    pub water: (u8, u8),
    pub day: (u8, u8),
    pub questions: Vec<Question>,
}

impl State {
    pub fn new() -> Self {
        Self {
            hp: (10, 10),
            saturation: (5, 5),
            water: (3, 3),
            day: (0, 5),
            questions: Question::get_vec(),
        }
    }
}

type Result = (fn(&mut State), fn(&mut State));

pub struct Question {
    pub text: &'static str,
    pub answers: (String, String),
    pub result: Result,
}

impl Question {
    pub fn get_vec() -> Vec<Question> {
        vec![
            Question {
                text: "Ваш взгляд устремлён на\n\
                неопознанное животное,\n\
                затаившиееся в кустах.\n\
                Ваша логика твердит вам\n\
                о том, что его дикие\n\
                инстинкты закончат ваше\n\
                жалкое существование, но\n\
                всё же вы не можете отказать\n\
                себе в том, чтобы развеять\n\
                ещё одну тайну в данном лесу.\n\n\
                Подойти к животному?",
                answers: ("Да".to_owned(), "Нет".to_owned()),
                result: (|s: &mut State| s.hp.0 -= 3, |_| {}),
            },
            Question {
                text: "Жили были три коровы одна\n\
                из них заболела. А потом\n\
                пришёл Jeff и починил\n\
                овервотч.",
                answers: ("Да".to_owned(), "Нет".to_owned()),
                result: (
                    |s: &mut State| s.water.0 += 1,
                    |s: &mut State| s.water.0 -= 1,
                ),
            },
            Question {
                text: "Жили были три коровы одна\n\
                из них заболела. А потом\n\
                пришёл Jeff и починил\n\
                овервотч.",
                answers: ("Да".to_owned(), "Нет".to_owned()),
                result: (
                    |s: &mut State| s.water.0 += 1,
                    |s: &mut State| s.water.0 -= 1,
                ),
            },
        ]
    }
}
