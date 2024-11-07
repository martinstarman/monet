use image::{ImageBuffer, Rgba};

use crate::color::Color;

#[derive(Clone)]
pub struct Layer {
  pub width: u32,
  pub height: u32,
  pub index: u32,
  pub active: bool,
  pub data: ImageBuffer<Rgba<u16>, Vec<u16>>,
}

impl Layer {
  pub fn new(width: u32, height: u32, index: u32) -> Self {
    Layer {
      width,
      height,
      index,
      active: false,
      data: ImageBuffer::new(100, 100),
    }
  }

  pub fn paint_at(&mut self, x: u32, y: u32, color: Color) {
    assert!(x < self.width, "x exceed width");
    assert!(y < self.height, "y exceed height");

    self.data.put_pixel(
      x,
      y,
      Rgba([color.red, color.green, color.blue, color.alpha]),
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let layer = Layer::new(10, 5, 0);

    assert_eq!(layer.width, 10);
    assert_eq!(layer.height, 5);
    assert_eq!(layer.index, 0);
    assert_eq!(layer.active, false);
    assert_eq!(layer.data.len(), 50);
  }

  #[test]
  fn paint_at() {
    let mut layer = Layer::new(10, 5, 0);
    let color = Color::new(1, 2, 3, 4);

    layer.paint_at(2, 2, color);

    let pixel = layer.data.get_pixel(2, 2).0;

    assert_eq!(pixel[0], 1);
    assert_eq!(pixel[1], 2);
    assert_eq!(pixel[2], 3);
    assert_eq!(pixel[3], 4);
  }
}
