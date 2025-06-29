// Allow copying bits, equality operators, hashmaps and can be formatted via {:?}
   #[allow(dead_code)]  
pub trait Token: Copy + Eq + std::hash::Hash + std::fmt::Debug {
  // includes assert to panick if value not within smaller T maximum value.
  fn from_u32(val: u32) -> Self;
  fn into_u32(self) -> u32;
}

impl Token for u8 {
  fn from_u32(val: u32) -> Self {
    assert!(val <= u8::MAX as u32);
    val as u8
  }

  fn into_u32(self) -> u32 {
    self as u32
  }
}

impl Token for u16 {
  fn from_u32(val: u32) -> Self {
    assert!(val <= u16::MAX as u32);
    val as u16
  }

  fn into_u32(self) -> u32 {
    self as u32
  }
}

// u32 necessary because std::char only has ::from_32()
impl Token for u32 {
  fn from_u32(val: u32) -> Self { val }
  fn into_u32(self) -> u32 { self }
}
