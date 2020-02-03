use ggez::{GameResult};
use ggez;
use ggez::event;

mod state;
mod componets;
mod systems;

use crate::state::MainState;


fn main() -> GameResult{
  let cb = ggez::ContextBuilder::new("Rust Specs test", "ggez");
  let (ctx, event_loop) = &mut cb.build()?;
  let state = &mut MainState::new(ctx);
  event::run(ctx, event_loop, state);
  Ok(())
}