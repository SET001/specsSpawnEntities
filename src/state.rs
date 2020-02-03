use specs::{World, WorldExt, Builder, Dispatcher, DispatcherBuilder};
use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics;
use cgmath::{Point2};

use crate::componets::*;
use crate::systems::{
  ZombieSpawner,
  // RenderSystem\
};
// use components::*;

pub struct MainState{
	frame: i32,
	dispatcher: Dispatcher<'static, 'static>,
	world: World
}

// struct Foo<'a>{
//   ctx: &'a Context
// }

impl MainState{ 
	pub fn new(ctx: &mut Context) -> MainState{
		let mut world = World::new();
		world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Target>();
    world.register::<View>();
	
    world.create_entity()
      .with(Position { x: 40.0, y: 70.0 })
      .with(Velocity { x: 0.1, y: 0.2 })
      .with(Target { x: 200.0, y: 200.0 })
      .with(View::new(ctx))
      .build();

		let mut dispatcher = DispatcherBuilder::new()
      // .with(ZombieSpawner, "ZombieSpawner", &[])
      // .with(RenderSystem{ctx}, "RenderSystem", &[])
			.build();
	
		dispatcher.dispatch(&mut world);
		world.maintain();

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
		Ok(())
	}
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let view_comp = self.world.read_storage::<View>();
    let position_comp = self.world.read_storage::<Position>();
    use specs::Join;
    for (view, position) in (&view_comp, &position_comp).join() {
      graphics::draw(
        ctx,
        &view.mesh,
        (Point2::new(position.x, position.y),),
      ).unwrap();
    }
    graphics::present(ctx);
		Ok(())
	}
}