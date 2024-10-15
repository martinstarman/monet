#[derive(Clone, Copy, Debug)]
pub struct Color {
  pub red: u16,
  pub green: u16,
  pub blue: u16,
  pub alpha: u16,
}

impl Color {
  pub fn new(red: u16, green: u16, blue: u16, alpha: u16) -> Self {
    Color {
      red,
      green,
      blue,
      alpha,
    }
  }
}

impl Default for Color {
  fn default() -> Self {
    Color {
      red: 0,
      green: 0,
      blue: 0,
      alpha: 0,
    }
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    self.red == other.red
      && self.green == other.green
      && self.blue == other.blue
      && self.alpha == other.alpha
  }
}

#[cfg(test)]
mod base {
  use super::*;

  #[test]
  fn new() {
    let color = Color::new(20, 30, 40, 50);

    assert_eq!(color.red, 20);
    assert_eq!(color.green, 30);
    assert_eq!(color.blue, 40);
    assert_eq!(color.alpha, 50);
  }

  #[test]
  fn default() {
    let color = Color::default();

    assert_eq!(color.red, 0);
    assert_eq!(color.green, 0);
    assert_eq!(color.blue, 0);
    assert_eq!(color.red, 0);
  }

  #[test]
  fn eq() {
    let color1 = Color::new(1, 1, 1, 1);
    let color2 = Color::new(2, 2, 2, 2);
    let color3 = Color::new(1, 1, 1, 1);

    assert_ne!(color1, color2);
    assert_eq!(color1, color3);
  }
}
