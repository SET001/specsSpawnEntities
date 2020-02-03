use ggez::{GameResult};
use ggez;
use ggez::event;
use ggez::graphics;
use cgmath::{Point2};

use std::path;
use std::env;


mod state;
mod componets;
mod systems;

use crate::state::MainState;


fn main() -> GameResult{
  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    path
  } else {
    path::PathBuf::from("./resources")
  };

  let cb = ggez::ContextBuilder::new("Rust Specs test", "ggez").add_resource_path(resource_dir);
  let (ctx, event_loop) = &mut cb.build()?;

  let circle = graphics::Mesh::new_circle(
    ctx,
    graphics::DrawMode::fill(),
    Point2::new(0.0, 0.0),
    3.0,
    0.1,
    graphics::WHITE,
  )?;
  let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();
  let state = &mut MainState::new(circle, font);
  event::run(ctx, event_loop, state)?;
  Ok(())
}