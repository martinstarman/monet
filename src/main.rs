use std::u16;

use color::Color;
use image::Image;

mod color;
mod image;
mod layer;

fn main() {
  let black = Color::new(0, 0, 0, u16::MAX);
  let mut image = Image::new(10, 10);

  image.append_layer();

  for y in 0..10 {
    for x in 0..10 {
      image.paint_at(x, y, black);
    }
  }
}
