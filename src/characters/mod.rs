pub mod choseong;
pub mod jongseong;
pub mod jungseong;

pub use choseong::*;
pub use jongseong::*;
pub use jungseong::*;

pub trait CharacterInformation {
  fn is_jaeum(&self) -> bool;
  fn is_moeum(&self) -> bool;
  fn to_composable(&self) -> u32;
}
