use crate::layer::Layer;

pub struct Image {
  pub width: usize,
  pub height: usize,
  pub layers: Vec<Layer>,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    Image {
      width,
      height,
      layers: vec![Layer::new(width, height, 0)],
    }
  }

  pub fn append_layer(&mut self) {
    self
      .layers
      .push(Layer::new(self.width, self.height, self.layers.len()));
  }
}

#[cfg(test)]
mod base {
  use super::*;

  #[test]
  fn new() {
    let image = Image::new(10, 5);

    assert_eq!(image.width, 10);
    assert_eq!(image.height, 5);
    assert_eq!(image.layers.len(), 1);
  }

  #[test]
  fn append_layer() {
    let mut image = Image::new(10, 5);
    image.append_layer();

    assert_eq!(image.layers.len(), 2);
  }
}
