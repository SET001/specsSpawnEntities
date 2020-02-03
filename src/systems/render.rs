use specs::{System, ReadStorage};
use ggez::{Context};
use ggez::graphics;


use crate::componets::{Position, View};

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
  type SystemData = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, View>
  );

  fn run(&mut self, (position, view): Self::SystemData) {
    
  }
}