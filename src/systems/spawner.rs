use specs::{System, ReadStorage, Entities, Read, LazyUpdate};
use ggez::{Context};
use rand::Rng;

use crate::componets::{Position, View, Target};

pub struct ZombieSpawner;

impl<'a> System<'a> for ZombieSpawner {
  type SystemData = (
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  fn run(&mut self, (entities, updater): Self::SystemData) {
    let mut rng = rand::thread_rng();

    let enemy = entities.create();
    updater.insert(enemy, Position {
      x: rng.gen_range(0.0, 300.0),
      y: rng.gen_range(0.0, 300.0)
    });
    updater.insert(enemy, View::new());
    updater.insert(enemy, Target{
      x: 200.0,
      y: 200.0
    });
  }
}
