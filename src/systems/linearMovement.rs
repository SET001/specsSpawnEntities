use specs::{System, ReadStorage};

use crate::componets::{Position, Target};

pub struct LinearMovement;

impl<'a> System<'a> for LinearMovement {
  type SystemData = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Target>
  );

  fn run(&mut self, position: Self::SystemData) {
    
  }
}