use crate::types::Race;

#[derive(Clone, Debug)]

pub struct Unit {
    race: Race,
    hp: i32,
    num_moves: (i32, i32),
    move_dist: i32,
    damage: i32,
    num_attacks: (i32, i32),
    attack_range: i32,
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
        }
    }

    pub fn new_spider() -> Self {
        Self {
            race: Race::Bug,
            hp: 1,
            num_moves: (1, 1),
            move_dist: 3,
            damage: 1,
            num_attacks: (1, 1),
            attack_range: 1,
        }
    }

    pub fn with_hp(mut self, x: i32) -> Self {
        self.hp = x;
        self
    }
    pub fn with_num_moves(mut self, x: i32) -> Self {
        self.num_moves = (x, x);
        self
    }
    pub fn with_move_dist(mut self, x: i32) -> Self {
        self.move_dist = x;
        self
    }
    pub fn with_num_attacks(mut self, x: i32) -> Self {
        self.num_attacks = (x, x);
        self
    }
    pub fn with_attack_range(mut self, x: i32) -> Self {
        self.attack_range = x;
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
    pub fn num_moves(&self) -> i32 {
        self.num_moves.1
    }
    pub fn move_dist(&self) -> i32 {
        self.move_dist
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
}
