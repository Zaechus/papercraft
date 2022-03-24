use bracket_lib::prelude::*;

use papercraft::State;

fn main() {
    let ctx = BTermBuilder::simple(80, 40)
        .unwrap()
        .with_tile_dimensions(32, 32)
        .with_title("PaperCraft")
        .build()
        .unwrap();
    let gs = State::new(80, 40);

    main_loop(ctx, gs).unwrap();
}
