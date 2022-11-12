#![allow(unused)]

use std::{collections::HashMap, hash::Hash};
pub struct State {
    qlist: (usize, usize),
    hp: (u8, u8),
    saturation: (u8, u8),
    water: (u8, u8),
    day: (u8, u8),
    dead: bool,
}

impl State {
    pub fn new(q: usize) -> Self {
        Self {
            qlist: (0, q),
            hp: (10, 10),
            saturation: (5, 10),
            water: (5, 10),
            day: (1, 5),
            dead: false,
        }
    }

    pub fn q(&self) -> usize {
        self.qlist.0
    }

    pub fn q_next(&mut self) {
        self.qlist.0 += 1
    }

    pub fn max_q(&self) -> usize {
        self.qlist.1
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn day(&self) -> u8 {
        self.day.0
    }

    pub fn max_day(&self) -> u8 {
        self.day.1
    }

    pub fn next_day(&mut self) {
        if self.day.0 + 1 >= self.day.1 {
            self.day.0 = self.day.1;
        } else {
            self.day.0 += 1;
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
            self.dead = true;
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
            self.dead = true;
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

    pub fn saturation(&self) -> u8 {
        self.saturation.0
    }

    pub fn max_saturation(&self) -> u8 {
        self.saturation.1
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
            self.dead = true;
        } else {
            self.saturation.0 -= saturation;
        }
    }
}
