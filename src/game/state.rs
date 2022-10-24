#![allow(unused)]

use std::{collections::HashMap, hash::Hash};
pub struct State {
    hp: (u8, u8),
    saturation: (u8, u8),
    water: (u8, u8),
    day: (u8, u8),
    pub questions: QuestionPool,
}

impl State {
    pub fn new() -> Self {
        Self {
            hp: (10, 10),
            saturation: (5, 5),
            water: (3, 3),
            day: (0, 5),
            questions: QuestionPool::new(),
        }
    }

    pub fn add_hp(&mut self, hp: u8) {
        if self.hp.0 + hp >= self.hp.1 {
            self.hp.0 = self.hp.1;
        } else {
            self.hp.0 += hp;
        }
    }

    pub fn remove_hp(&mut self, hp: u8) {
        if self.hp.0 <= hp {
            self.hp.0 = 0;
        } else {
            self.hp.0 -= hp;
        }
    }

    pub fn hp(&self) -> u8 {
        self.hp.0
    }

    pub fn max_hp(&self) -> u8 {
        self.hp.1
    }

    pub fn add_water(&mut self, water: u8) {
        if self.water.0 + water >= self.water.1 {
            self.water.0 = self.water.1;
        } else {
            self.water.0 += water;
        }
    }

    pub fn remove_water(&mut self, water: u8) {
        if self.water.0 <= water {
            self.water.0 = 0;
        } else {
            self.water.0 -= water;
        }
    }

    pub fn water(&self) -> u8 {
        self.water.0
    }

    pub fn max_water(&self) -> u8 {
        self.water.1
    }

    pub fn add_saturation(&mut self, saturation: u8) {
        if self.saturation.0 + saturation >= self.saturation.1 {
            self.saturation.0 = self.saturation.1;
        } else {
            self.saturation.0 += saturation;
        }
    }

    pub fn remove_saturation(&mut self, saturation: u8) {
        if self.saturation.0 <= saturation {
            self.saturation.0 = 0;
        } else {
            self.saturation.0 -= saturation;
        }
    }

    pub fn saturation(&self) -> u8 {
        self.saturation.0
    }

    pub fn max_saturation(&self) -> u8 {
        self.saturation.1
    }

    pub fn next_day(&mut self, day: u8) {
        if self.day.0 + day >= self.day.1 {
            self.day.0 = self.day.1;
        } else {
            self.day.0 += day;
        }
    }

    pub fn day(&self) -> u8 {
        self.day.0
    }

    pub fn max_day(&self) -> u8 {
        self.day.1
    }
}

pub struct QuestionPool {
    question_pool: HashMap<u8, Question>,
}

impl QuestionPool {
    pub fn new() -> Self {
        Self {
            question_pool: HashMap::new()
                .insert(
                    0,
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
                )
                .insert(
                    1,
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
                ),
        }
    }
    pub fn get_question(&mut self) -> Question {
        self.question_pool[0]
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
        todo!();
    }
}
