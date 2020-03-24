use bracket_lib::prelude::*;

#[derive(Clone, Debug)]
pub struct GameCell {
    x: i32,
    y: i32,
    symbol: char,
    color: RGB,
    selected: bool,
}

impl GameCell {
    pub fn new(x: i32, y: i32, symbol: char, color: RGB) -> Self {
        Self {
            x,
            y,
            symbol,
            color,
            selected: false,
        }
    }

    pub fn move_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn select(&mut self) {
        self.selected = !self.selected;
    }
    pub fn deselect(&mut self) {
        self.selected = false
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn symbol(&self) -> char {
        self.symbol
    }
    pub fn color(&self) -> RGB {
        self.color
    }
    pub fn bg_color(&self) -> RGB {
        if self.selected {
            RGB::from_u8(255, 255, 255)
        } else {
            RGB::from_u8(0, 0, 0)
        }
    }
    pub fn selected(&self) -> bool {
        self.selected
    }
}
