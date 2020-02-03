use specs::{World, WorldExt, Dispatcher, DispatcherBuilder};
use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics;
use cgmath::{Point2};

use crate::componets::*;
use crate::systems::{
  ZombieSpawner, LinearMovement
  // RenderSystem\
};
// use components::*;

pub struct MainState{
	dispatcher: Dispatcher<'static, 'static>,
	world: World
}

impl MainState{ 
	pub fn new() -> MainState{
		let mut world = World::new();
		world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Target>();
    world.register::<View>();

    let dispatcher = DispatcherBuilder::new()
      .with(LinearMovement, "LinearMovement", &[])
      .with(ZombieSpawner, "ZombieSpawner", &[])
			.build();

		MainState {
			world,
			dispatcher,
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
    graphics::present(ctx)?;
		Ok(())
	}
}