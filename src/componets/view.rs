use specs::{Component, VecStorage};
use ggez::graphics::{Mesh, DrawMode, Color, WHITE};
use ggez::{Context};
use cgmath::{Point2};

#[derive(Debug)]
pub struct ViewArgs{
  pub drawMode: DrawMode,
  pub point: Point2<f32>,
  pub radius: f32,
  pub tolerance: f32,
  pub color: Color
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct View {
  pub args: ViewArgs,
  pub mesh: Option<Mesh>
}

impl View {
  pub fn new()->View{
    View {
      args: ViewArgs {
        drawMode: DrawMode::fill(),
        point: Point2::new(0.0, 0.0),
        radius: 3.0,
        tolerance: 0.1,
        color: WHITE
      },
      mesh: None
    }
  }
}