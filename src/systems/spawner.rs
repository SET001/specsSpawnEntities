use specs::{System, ReadStorage};

use crate::componets::{Position};

pub struct ZombieSpawner;

impl<'a> System<'a> for ZombieSpawner {
  type SystemData = ReadStorage<'a, Position>;

  fn run(&mut self, position: Self::SystemData) {
    
  }
}