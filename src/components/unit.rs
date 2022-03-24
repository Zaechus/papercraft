use bracket_lib::prelude::*;

use crate::{components::GameCell, types::Race};

#[derive(Clone, Debug)]

pub struct Unit {
    race: Race,
    hp: i32,
    num_moves: (i32, i32),
    move_dist: i32,
    damage: i32,
    num_attacks: (i32, i32),
    attack_range: i32,
    num_interceptors: (i32, i32),
    _num_interceptor_moves: i32,
    lifespan: Option<i32>,
}

impl Unit {
    pub fn new(race: Race, hp: i32) -> Self {
        Self {
            race,
            hp,
            num_moves: (1, 1),
            move_dist: 3,
            damage: 1,
            num_attacks: (1, 1),
            attack_range: 3,
            num_interceptors: (0, 0),
            _num_interceptor_moves: 0,
            lifespan: None,
        }
    }

    pub fn new_spider() -> Self {
        Unit::new(Race::Bug, 1).with_attack_range(1)
    }

    pub fn new_war_carrier() -> Self {
        Self {
            race: Race::Bionic,
            hp: 6,
            num_moves: (1, 1),
            move_dist: 1,
            damage: 1,
            num_attacks: (0, 0),
            attack_range: 0,
            num_interceptors: (2, 2),
            _num_interceptor_moves: 2,
            lifespan: None,
        }
    }

    pub fn with_num_moves(mut self, n: i32) -> Self {
        self.num_moves = (n, n);
        self
    }
    pub fn with_move_dist(mut self, x: i32) -> Self {
        self.move_dist = x;
        self
    }
    pub fn with_damage(mut self, x: i32) -> Self {
        self.damage = x;
        self
    }
    pub fn with_num_attacks(mut self, n: i32) -> Self {
        self.num_attacks = (n, n);
        self
    }
    pub fn with_attack_range(mut self, x: i32) -> Self {
        self.attack_range = x;
        self
    }
    pub fn with_lifespan(mut self, x: i32) -> Self {
        self.lifespan = Some(x);
        self
    }

    pub fn use_move(&mut self) {
        self.num_moves.0 -= 1;
    }
    pub fn use_attack(&mut self) {
        self.num_attacks.0 -= 1;
    }
    pub fn harm(&mut self, x: i32) {
        self.hp -= x;
    }
    pub fn recharge(&mut self) {
        self.num_moves.0 = self.num_moves.1;
        self.num_attacks.0 = self.num_attacks.1;
        self.num_interceptors.0 = self.num_interceptors.1;
        if let Some(ls) = self.lifespan {
            self.lifespan = Some(ls - 1);
            if ls - 1 <= 0 {
                self.hp = 0;
            }
        }
    }
    pub fn make_interceptor(&mut self, x: i32, y: i32) -> Option<(GameCell, Unit)> {
        if self.num_interceptors.0 > 0 {
            self.num_interceptors.0 -= 1;
            Some((
                GameCell::new(x, y, '^', RGB::from_u8(0, 200, 0)),
                Unit::new(Race::Bionic, 1)
                    .with_num_moves(2)
                    .with_num_attacks(2)
                    .with_attack_range(1)
                    .with_lifespan(2),
            ))
        } else {
            None
        }
    }

    pub fn can_move(&self) -> bool {
        self.num_moves.0 > 0
    }

    pub fn race(&self) -> Race {
        self.race
    }
    pub fn hp(&self) -> i32 {
        self.hp
    }
    pub fn num_moves(&self) -> (i32, i32) {
        self.num_moves
    }
    pub fn move_dist(&self) -> i32 {
        self.move_dist
    }
    pub fn num_attacks(&self) -> (i32, i32) {
        self.num_attacks
    }
    pub fn damage(&self) -> i32 {
        if self.num_attacks.0 > 0 {
            self.damage
        } else {
            0
        }
    }
    pub fn attack_range(&self) -> i32 {
        self.attack_range
    }
    pub fn num_interceptors(&self) -> (i32, i32) {
        self.num_interceptors
    }
}
