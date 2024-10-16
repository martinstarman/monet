use crate::{color::Color, layer::Layer};

pub struct Image {
  pub width: usize,
  pub height: usize,
  pub layers: Vec<Layer>,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    let mut image = Image {
      width,
      height,
      layers: vec![],
    };

    image.append_layer();
    image.set_active_layer(0);

    image
  }

  pub fn append_layer(&mut self) {
    self
      .layers
      .push(Layer::new(self.width, self.height, self.layers.len()));
  }

  pub fn set_active_layer(&mut self, index: usize) {
    for layer in self.layers.iter_mut() {
      layer.active = layer.index == index;
    }
  }

  pub fn paint_at(&mut self, x: usize, y: usize, color: Color) {
    for layer in self.layers.iter_mut() {
      if layer.active {
        layer.paint_at(x, y, color);
      }
    }
  }
}

#[cfg(test)]
mod tests {
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
    let mut image = Image::new(1, 1);
    image.append_layer();

    assert_eq!(image.layers.len(), 2);
  }

  #[test]
  fn set_active_layer() {
    let mut image = Image::new(1, 1);

    image.append_layer();
    image.append_layer();
    image.append_layer();

    image.set_active_layer(1);

    assert_eq!(image.layers[0].active, false);
    assert_eq!(image.layers[1].active, true);
    assert_eq!(image.layers[2].active, false);
  }

  #[test]
  fn paint_at() {
    let mut image = Image::new(1, 1);
    let color1 = Color::new(1, 1, 1, 1);
    let color2 = Color::new(2, 2, 2, 2);

    image.append_layer();
    image.append_layer();
    image.set_active_layer(0);
    image.paint_at(0, 0, color1);
    image.set_active_layer(1);
    image.paint_at(0, 0, color2);

    assert_eq!(image.layers[0].data[0], color1);
    assert_eq!(image.layers[1].data[0], color2);
  }
}
