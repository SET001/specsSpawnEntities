use specs::{World, WorldExt, Dispatcher, DispatcherBuilder};
use ggez::{GameResult, Context};
use ggez::event;
use ggez::timer;
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
  world: World,
  circle: graphics::Mesh,
  font: graphics::Font
}

impl MainState{ 
	pub fn new(circle: graphics::Mesh, font: graphics::Font) -> MainState{
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
      circle,
      font
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
    let a = (&view_comp, &position_comp).join();
    let count = (&view_comp, &position_comp).join().count();
    
    for (view, position) in a {
      if position.x < 2000.0{
        graphics::draw(
          ctx,
          &self.circle,
          (Point2::new(position.x, position.y),),
        ).unwrap();
      }
    }

    let dest_point = cgmath::Point2::new(1.0, 10.0);
    let stext = format!("Entities: {}\nFPS: {}", count, timer::fps(ctx).floor());
    let counterText = graphics::Text::new((stext, self.font, 48.0));
    graphics::draw(ctx, &counterText, (dest_point,))?;
    graphics::present(ctx)?;
		Ok(())
	}
}