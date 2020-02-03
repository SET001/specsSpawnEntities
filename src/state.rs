use specs::{World, WorldExt, Builder, Dispatcher, DispatcherBuilder};
use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics;
use cgmath::{Point2};
use rand::Rng;

use crate::componets::*;
use crate::systems::{
  ZombieSpawner, LinearMovement
  // RenderSystem\
};
// use components::*;

pub struct MainState{
	frame: i32,
	dispatcher: Dispatcher<'static, 'static>,
	world: World
}

struct Foo<'a>{
  ctx: &'a mut Context
}

impl MainState{ 
	pub fn new(ctx: &mut Context) -> MainState{
		let mut world = World::new();
		world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Target>();
    world.register::<View>();
  
    let mut rng = rand::thread_rng();
    let spawnChance = rng.gen_range(0.0, 10.0);

    let mut dispatcher = DispatcherBuilder::new()
      .with(LinearMovement, "LinearMovement", &[])
      .with(ZombieSpawner, "ZombieSpawner", &[])
			.build();

		MainState {
			world,
			dispatcher,
			frame: 0
		}
	}
}


impl event::EventHandler for MainState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
    self.dispatcher.dispatch(&mut self.world);
    self.world.maintain();
		Ok(())
  }
  
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
    use specs::Join;

    let view_comp = self.world.read_storage::<View>();
    let position_comp = self.world.read_storage::<Position>();
    
    graphics::clear(ctx, graphics::BLACK);
    for (view, position) in (&view_comp, &position_comp).join() {
      let circle = graphics::Mesh::new_circle(
        ctx,
        view.args.drawMode,
        Point2::new(position.x, position.y),
        view.args.radius,
        view.args.tolerance,
        view.args.color,
      )?;
      graphics::draw(
        ctx,
        &circle,
        (Point2::new(position.x, position.y),),
      ).unwrap();
    }
    graphics::present(ctx);
		Ok(())
	}
}