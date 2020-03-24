use bracket_lib::prelude::*;

use papercraft::State;

fn main() {
    let context = BTermBuilder::simple(160, 80)
        .with_title("PaperCraft")
        .build();
    let gs = State::new();

    main_loop(context, gs);
}
