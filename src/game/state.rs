#![allow(unused)]

use std::{collections::HashMap, hash::Hash};
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
}
