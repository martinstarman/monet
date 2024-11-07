use std::u16;

use ::image::{DynamicImage, ImageBuffer, Rgba};
use color::Color;
use image::Image;

mod color;
mod image;
mod layer;

fn main() {
  let black = Color::new(0, 0, 0, u16::MAX);
  let red = Color::new(u16::MAX, 0, 0, u16::MAX);

  let mut image = Image::new(100, 100);

  image.append_layer();
  image.append_layer();

  image.set_active_layer(0);

  for y in 0..10 {
    for x in 0..10 {
      image.paint_at(x, y, black);
    }
  }

  image.set_active_layer(1);

  for y in 10..20 {
    for x in 10..20 {
      image.paint_at(x, y, red);
    }
  }

  let mut buff: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(100, 100);

  for i in 0..2 {
    for x in 0..100 {
      for y in 0..100 {
        let pixel = Rgba(image.layers[i].data.get_pixel(x, y).0);

        if pixel[3] != 0 {
          buff.put_pixel(x, y, pixel);
        }
      }
    }
  }

  let i = DynamicImage::ImageRgba16(buff);
  let _ = i.save("out.png");
}
