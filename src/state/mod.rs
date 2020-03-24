use bracket_lib::prelude::*;

use legion::prelude::*;

use crate::{
    components::{GameCell, Unit},
    types::Race,
};

enum Mode {
    Select,
    Move,
    Attack,
}

#[derive(Clone, Debug)]
pub enum CurrentState {
    Menu,
    Playing,
}

pub struct State {
    curr_state: CurrentState,
    world: World,
    window_size: (u32, u32),
    mouse: Point,
    mouse_pressed: bool,
    mouse_released: bool,
    turn: Race,
    selected: bool,
    mode: Mode,
}

impl State {
    pub fn new(w: u32, h: u32) -> Self {
        let universe = Universe::new();
        let mut world = universe.create_world();

        let units = vec![
            (
                GameCell::new(10, 10, '*', RGB::from_u8(170, 20, 0)),
                Unit::new_spider(),
            ),
            (
                GameCell::new(11, 10, '*', RGB::from_u8(170, 20, 0)),
                Unit::new_spider(),
            ),
            (
                GameCell::new(8, 9, 'Q', RGB::from_u8(170, 20, 0)),
                Unit::new(Race::Bug, 1).with_hp(2).with_move_dist(1),
            ),
        ];
        world.insert((), units.into_iter());

        let units = vec![
            (
                GameCell::new(14, 13, '@', RGB::from_u8(175, 175, 175)),
                Unit::new(Race::Human, 1).with_move_dist(1),
            ),
            (
                GameCell::new(16, 13, 'W', RGB::from_u8(175, 175, 175)),
                Unit::new(Race::Human, 1)
                    .with_hp(2)
                    .with_move_dist(1)
                    .with_num_attacks(3)
                    .with_attack_range(1),
            ),
        ];
        world.insert((), units.into_iter());

        let units = vec![
            (
                GameCell::new(20, 20, 'V', RGB::from_u8(0, 200, 0)),
                Unit::new(Race::Bionic, 1)
                    .with_hp(2)
                    .with_num_moves(2)
                    .with_num_attacks(2)
                    .with_attack_range(1),
            ),
            (
                GameCell::new(15, 20, 'Y', RGB::from_u8(0, 200, 0)),
                Unit::new(Race::Bionic, 1).with_hp(3).with_num_attacks(2),
            ),
        ];
        world.insert((), units.into_iter());

        Self {
            curr_state: CurrentState::Menu,
            world,
            window_size: (w, h),
            mouse: Point::new(0, 0),
            mouse_pressed: false,
            mouse_released: false,
            turn: Race::Bug,
            selected: false,
            mode: Mode::Select,
        }
    }

    fn menu_state(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(self.window_size.1 as i32 / 2 - 1, "PaperCraft");
        ctx.print_centered(
            self.window_size.1 as i32 / 2 + 1,
            "Press the spacebar to start",
        );

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.curr_state = CurrentState::Playing;
        }
    }

    fn play_state(&mut self, ctx: &mut BTerm) {
        if ctx.left_click {
            if self.mouse_pressed {
                self.mouse_released = true;
            }
            self.mouse_pressed = !self.mouse_pressed;
        }

        self.print_mode(ctx);

        ctx.print_centered(1, &format!("{:?}", self.turn));

        let mut end_turn_box_rgb = RGB::from_u8(170, 0, 0);
        if self.mouse.x >= self.window_size.0 as i32 - 10
            && self.mouse.x <= self.window_size.0 as i32
            && self.mouse.y >= 0
            && self.mouse.y <= 2
        {
            end_turn_box_rgb = RGB::from_u8(200, 0, 0);
            if self.mouse_released {
                self.advance_turn();
            }
        }

        ctx.draw_box(
            self.window_size.0 as i32 - 10,
            0,
            9,
            2,
            end_turn_box_rgb,
            end_turn_box_rgb,
        );
        ctx.print_color(
            self.window_size.0 as i32 - 9,
            1,
            RGB::from_u8(255, 255, 255),
            end_turn_box_rgb,
            "End turn",
        );

        self.mouse = ctx.mouse_point();

        ctx.print(self.mouse.x, self.mouse.y, "^");

        self.print_cells(ctx);

        if self.mouse_released {
            match self.mode {
                Mode::Select => self.select_cells(),
                Mode::Move => self.move_cells(),
                Mode::Attack => self.attack_cells(),
            }
        }

        self.key_input(ctx);

        self.clear_cells();

        self.mouse_released = false;
    }

    fn key_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::M => {
                    if self.selected {
                        self.mode = Mode::Move
                    }
                }
                VirtualKeyCode::A => {
                    if self.selected {
                        self.mode = Mode::Attack
                    }
                }
                VirtualKeyCode::Escape => self.mode = Mode::Select,
                _ => (),
            }
        }
    }

    fn print_mode(&mut self, ctx: &mut BTerm) {
        match self.mode {
            Mode::Select => (),
            Mode::Move => {
                ctx.draw_box(0, 0, 5, 2, RGB::from_u8(0, 175, 0), RGB::from_u8(0, 175, 0));
                ctx.print_color(
                    1,
                    1,
                    RGB::from_u8(255, 255, 255),
                    RGB::from_u8(0, 175, 0),
                    "Move",
                )
            }
            Mode::Attack => {
                ctx.draw_box(0, 0, 7, 2, RGB::from_u8(175, 0, 0), RGB::from_u8(175, 0, 0));
                ctx.print_color(
                    1,
                    1,
                    RGB::from_u8(255, 255, 255),
                    RGB::from_u8(175, 0, 0),
                    "Attack",
                )
            }
        }
    }

    fn print_cells(&mut self, ctx: &mut BTerm) {
        let read_query = <(Read<GameCell>, Read<Unit>)>::query();

        for (cell, unit) in read_query.iter_immutable(&self.world) {
            if cell.selected() {
                match self.mode {
                    Mode::Move => {
                        for x in 1..=unit.num_moves() {
                            ctx.draw_hollow_box(
                                cell.x() - unit.move_dist() * x,
                                cell.y() - unit.move_dist() * x,
                                unit.move_dist() * 2 * x,
                                unit.move_dist() * 2 * x,
                                RGB::from_u8(0, 255, 0),
                                RGB::new(),
                            )
                        }
                    }
                    Mode::Attack => ctx.draw_hollow_box(
                        cell.x() - unit.attack_range(),
                        cell.y() - unit.attack_range(),
                        unit.attack_range() * 2,
                        unit.attack_range() * 2,
                        RGB::from_u8(255, 0, 0),
                        RGB::new(),
                    ),
                    _ => (),
                }
            }

            ctx.print_color(
                cell.x(),
                cell.y(),
                if self.mouse.x == cell.x() && self.mouse.y == cell.y() {
                    cell.color_bright()
                } else {
                    cell.color()
                },
                cell.bg_color(),
                &cell.symbol().to_string(),
            );
        }
    }

    fn select_cells(&mut self) {
        let query = <(Write<GameCell>,)>::query();

        let mut selected = false;
        for (mut cell,) in query.iter(&mut self.world) {
            if self.mouse.x == cell.x() && self.mouse.y == cell.y() {
                cell.select();
                selected = true;
            } else {
                cell.deselect();
                self.selected = false;
            }
        }
        self.selected = selected;
    }

    fn move_cells(&mut self) {
        let read_query = <(Read<GameCell>, Read<Unit>)>::query();
        let query = <(Write<GameCell>, Write<Unit>)>::query();

        let mut can_move = false;
        for (cell, unit) in read_query.iter_immutable(&self.world) {
            if unit.race() == self.turn
                && cell.selected()
                && unit.can_move()
                && Rect::with_exact(
                    cell.x() - unit.move_dist(),
                    cell.y() - unit.move_dist(),
                    cell.x() + unit.move_dist(),
                    cell.y() + unit.move_dist(),
                )
                .point_in_rect(Point::new(self.mouse.x, self.mouse.y))
            {
                can_move = true;
                for (cell, _) in read_query.iter_immutable(&self.world) {
                    if cell.x() == self.mouse.x && cell.y() == self.mouse.y {
                        can_move = false;
                    }
                }
            }
        }

        if can_move {
            for (mut cell, mut unit) in query.iter(&mut self.world) {
                if cell.selected() {
                    cell.move_pos(self.mouse.x, self.mouse.y);
                    cell.deselect();
                    unit.use_move();
                    self.mode = Mode::Select;
                    break;
                }
            }
        }
    }

    fn attack_cells(&mut self) {
        let read_query = <(Read<GameCell>, Read<Unit>)>::query();
        let query = <(Read<GameCell>, Write<Unit>)>::query();

        let mut damage = 0;
        for (cell, unit) in read_query.iter_immutable(&self.world) {
            if cell.selected()
                && Rect::with_exact(
                    cell.x() - unit.attack_range(),
                    cell.y() - unit.attack_range(),
                    cell.x() + unit.attack_range(),
                    cell.y() + unit.attack_range(),
                )
                .point_in_rect(Point::new(self.mouse.x, self.mouse.y))
            {
                for (cell2, unit2) in read_query.iter_immutable(&self.world) {
                    if unit.race() != unit2.race()
                        && cell2.x() == self.mouse.x
                        && cell2.y() == self.mouse.y
                    {
                        damage = unit.damage();
                    }
                }
            }
        }

        if damage > 0 {
            for (cell, mut unit) in query.iter(&mut self.world) {
                if cell.selected() {
                    unit.use_attack();
                } else if cell.x() == self.mouse.x && cell.y() == self.mouse.y {
                    unit.harm(damage);
                }
            }
        }
    }

    fn clear_cells(&mut self) {
        let query = <(Read<Unit>,)>::query().filter(changed::<Unit>());

        let mut deleted = Vec::new();
        for (e, (unit,)) in query.iter_entities_immutable(&self.world) {
            if unit.hp() <= 0 {
                deleted.push(e);
            }
        }
        for e in deleted {
            self.world.delete(e);
        }
    }

    fn advance_turn(&mut self) {
        self.turn = match self.turn {
            Race::Bug => Race::Human,
            Race::Human => Race::Bionic,
            Race::Bionic => {
                let query = <(Write<Unit>,)>::query().filter(changed::<Unit>());

                for (mut unit,) in query.iter(&mut self.world) {
                    unit.recharge();
                }
                Race::Bug
            }
        };
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match self.curr_state {
            CurrentState::Menu => self.menu_state(ctx),
            CurrentState::Playing => self.play_state(ctx),
        }
    }
}
