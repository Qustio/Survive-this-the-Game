use super::state::State;
use config::Config;
use rng::Rng;

pub struct Questions {
    list: Vec<Question>,
    pub current: Option<Question>,
    pub state: Qstate,
}

#[derive(Copy, Clone)]
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
                        Answer("Да", |s: &mut State| s.remove_hp(3), "Неправильно"),
                        Answer("Нет", |_| {}, "Правильно"),
                    ]),
                },
                Question {
                    text: "Вопрос с большим \n\
                        выбором ответа",
                    answers: Answers::new(vec![
                        Answer("Да", |s: &mut State| s.remove_hp(3), "-3 воды"),
                        Answer("Нет", |s: &mut State| s.remove_water(1), "-1 вода"),
                        Answer(
                            "Незнаю",
                            |s: &mut State| s.remove_saturation(2),
                            "-2 здоровья",
                        ),
                        Answer("НАВЕРНОЕ", |s: &mut State| s.remove_hp(3), "-3 здоровья"),
                    ]),
                },
                Question {
                    text: "Жили были три коровы одна из них заболела. А потом пришёл Jeff и починил овервотч.",
                    answers: Answers::new(vec![
                        Answer("Да", |s: &mut State| s.remove_hp(3), "Неправильно"),
                        Answer("Очень длинный ответ, ну прям очень", |_| {}, "Правильно"),
                    ]),
                },
            ],
            current: Option::None,
            state: Qstate::None,
        }
    }

    pub fn get_random_question(&mut self) -> Result<(), ()> {
        let mut r = rng::thread_rng();
        if self.list.is_empty() {
            return Err(());
        }
        let rusize: usize = r.gen_range(0..self.list.len());
        self.current = Some(self.list[rusize].clone());
        self.list.remove(rusize);
        self.state = Qstate::Answering;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Answer(&'static str, fn(&mut State), &'static str);

impl Answer {
    pub fn get_answer(&self) -> &str {
        self.0
    }
    pub fn get_hint(&self) -> &str {
        self.2
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
}

#[derive(Clone)]
pub struct Question {
    pub text: &'static str,
    pub answers: Answers,
}
