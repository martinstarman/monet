use crate::color::Color;

#[derive(Clone)]
pub struct Layer {
  pub width: usize,
  pub height: usize,
  pub index: usize,
  pub active: bool,
  pub data: Vec<Color>,
}

impl Layer {
  pub fn new(width: usize, height: usize, index: usize) -> Self {
    Layer {
      width,
      height,
      index,
      active: false,
      data: vec![Color::default(); width * height],
    }
  }

  pub fn paint_at(&mut self, x: usize, y: usize, color: Color) {
    assert!(x < self.width, "x exceed width");
    assert!(y < self.height, "y exceed height");

    let position = y * self.width + x;
    self.data[position] = color;
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
    let color = Color::new(1, 1, 1, 1);

    layer.paint_at(2, 2, color);

    assert_eq!(layer.data[22], color);
  }
}
