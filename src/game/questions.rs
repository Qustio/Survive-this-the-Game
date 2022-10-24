use super::state::State;
use rng::Rng;

pub struct Questions {
    list: Vec<Question>,
    pub current: Option<Question>,
    pub state: Qstate,
}

pub enum Qstate {
    None,
    Answering,
}

impl Questions {
    pub fn new() -> Questions {
        Questions {
            list: vec![
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
                    answers: Answers::new(vec![
                        Answer {
                            0: "Да",
                            1: |s: &mut State| s.hp.0 -= 3,
                        },
                        Answer {
                            0: "Нет",
                            1: |_| {},
                        },
                    ]),
                },
                Question {
                    text: "Жили были три коровы одна\n\
                из них заболела. А потом\n\
                пришёл Jeff и починил\n\
                овервотч.",
                    answers: Answers::new(vec![
                        Answer {
                            0: "Да",
                            1: |s: &mut State| s.hp.0 -= 3,
                        },
                        Answer {
                            0: "Нет",
                            1: |_| {},
                        },
                    ]),
                },
                Question {
                    text: "Жили были три коровы одна\n\
                из них заболела. А потом\n\
                пришёл Jeff и починил\n\
                овервотч.",
                    answers: Answers::new(vec![
                        Answer {
                            0: "Да",
                            1: |s: &mut State| s.hp.0 -= 3,
                        },
                        Answer {
                            0: "Нет",
                            1: |_| {},
                        },
                    ]),
                },
            ],
            current: Option::None,
            state: Qstate::None,
        }
    }

    pub fn get_random_question(&mut self) {
        match self.state {
            Qstate::Answering => (),
            Qstate::None => {
                let mut r = rng::thread_rng();
                if self.list.len() == 0 {
                    return;
                }
                let rusize: usize = r.gen_range(0..self.list.len());
                self.current = Some(self.list[rusize].clone());
                self.list.remove(rusize);
                self.state = Qstate::Answering;
            }
        }
    }
}

#[derive(Clone)]
pub struct Answer(&'static str, fn(&mut State));

impl Answer {
    pub fn get_answer(&self) -> &str {
        self.0
    }
    pub fn make_choice(&self, s: &mut State) {
        self.1(s)
    }
}

#[derive(Clone)]
pub struct Answers {
    pub answers: Vec<Answer>,
}

impl Answers {
    fn new(answers: Vec<Answer>) -> Self {
        Self { answers }
    }

    fn count(&self) -> usize {
        self.answers.len()
    }
}

#[derive(Clone)]
pub struct Question {
    pub text: &'static str,
    pub answers: Answers,
}
