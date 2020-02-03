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
    let a = (&view_comp, &position_comp).join();
    let count = (&view_comp, &position_comp).join().count();
    for (view, position) in a {
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

    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
    let dest_point = cgmath::Point2::new(1.0, 10.0);
    let stext = format!("Entities: {}", count);
    let counterText = graphics::Text::new((stext, font, 48.0));
    graphics::draw(ctx, &counterText, (dest_point,))?;

    let fpsText = graphics::Text::new((
      format!("FPS: {}", timer::fps(ctx)),
      font,
      48.0));
    graphics::draw(ctx, &fpsText, (cgmath::Point2::new(1.0, 60.0),))?;
    graphics::present(ctx)?;
		Ok(())
	}
}