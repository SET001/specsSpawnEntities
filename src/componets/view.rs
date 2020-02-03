use specs::{Component, VecStorage};
use ggez::graphics;
use ggez::{GameResult, Context};
use cgmath::{Point2};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct View {
  pub mesh: graphics::Mesh
}

impl View {
  pub fn new(ctx: &mut Context)->View{
    let mesh = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      Point2::new(0.0, 0.0),
      3.0,
      0.1,
      graphics::WHITE
    ).unwrap();

    View {
      mesh
    }
  }
}


// let circle = graphics::Mesh::new_circle(
//     ctx,
//     graphics::DrawMode::fill(),
//     na::Point2::new(blob.position.x, blob.position.y),
//     blob.radius,
//     0.1,
//     graphics::WHITE,
//   )?;