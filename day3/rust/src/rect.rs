#[derive(Debug, Clone)]
pub struct Rect {
  pub id: u16,
  pub top: u16,
  pub left: u16,
  pub width: u8,
  pub height: u8,
}

impl Rect {
  pub fn get_bounds(&self) -> (u16, u16, u16, u16) {
    (self.left, self.top, self.left + (self.width as u16), self.top + (self.height as u16))
  }
}