use self::JungseongCharacter::*;
use crate::constants::*;
use crate::syllable::*;
use crate::CharacterInformation;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum JungseongCharacter {
  /// ㅏ
  A,
  /// ㅐ
  AE,
  /// ㅑ
  YA,
  /// ㅒ
  YAE,
  /// ㅓ
  EO,
  /// ㅔ
  E,
  /// ㅕ
  YEO,
  /// ㅖ
  YE,
  /// ㅗ
  O,
  /// ㅘ
  WA,
  /// ㅙ
  WAE,
  /// ㅚ
  OE,
  /// ㅛ
  YO,
  /// ㅜ
  U,
  /// ㅝ
  WEO,
  /// ㅞ
  WE,
  /// ㅟ
  WI,
  /// ㅠ
  YU,
  /// ㅡ
  EU,
  /// ㅢ
  YI,
  /// ㅣ
  I,
}

impl JungseongCharacter {
  pub(crate) fn to_code(code: u32) -> u32 {
    if code.is_syllable() {
      let jongseong_code = (code - HANGEUL_OFFSET) % JUNGSEONG_COUNT;
      let value = ((code - HANGEUL_OFFSET - jongseong_code) % (JUNGSEONG_COUNT * JONGSEONG_COUNT))
        / JUNGSEONG_COUNT;
      value + JUNGSEONG_START
    } else {
      code
    }
  }

  pub fn to_index(&self) -> u32 {
    match self {
      A => 0,
      AE => 1,
      YA => 2,
      YAE => 3,
      EO => 4,
      E => 5,
      YEO => 6,
      YE => 7,
      O => 8,
      WA => 9,
      WAE => 10,
      OE => 11,
      YO => 12,
      U => 13,
      WEO => 14,
      WE => 15,
      WI => 16,
      YU => 17,
      EU => 18,
      YI => 19,
      I => 20,
    }
  }
}

impl CharacterInformation for JungseongCharacter {
  fn is_jaeum(&self) -> bool {
    true
  }

  fn is_moeum(&self) -> bool {
    false
  }

  fn to_composable(&self) -> u32 {
    self.to_index() * JUNGSEONG_COUNT
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_jaeum() {
    assert_eq!(A.is_jaeum(), true);
    assert_eq!(AE.is_jaeum(), true);
    assert_eq!(YA.is_jaeum(), true);
    assert_eq!(YAE.is_jaeum(), true);
    assert_eq!(EO.is_jaeum(), true);
    assert_eq!(E.is_jaeum(), true);
    assert_eq!(YEO.is_jaeum(), true);
    assert_eq!(YE.is_jaeum(), true);
    assert_eq!(O.is_jaeum(), true);
    assert_eq!(WA.is_jaeum(), true);
    assert_eq!(WAE.is_jaeum(), true);
    assert_eq!(OE.is_jaeum(), true);
    assert_eq!(YO.is_jaeum(), true);
    assert_eq!(U.is_jaeum(), true);
    assert_eq!(WEO.is_jaeum(), true);
    assert_eq!(WE.is_jaeum(), true);
    assert_eq!(WI.is_jaeum(), true);
    assert_eq!(YU.is_jaeum(), true);
    assert_eq!(EU.is_jaeum(), true);
    assert_eq!(YI.is_jaeum(), true);
    assert_eq!(I.is_jaeum(), true);
  }

  #[test]
  fn is_moeum() {
    assert_eq!(A.is_moeum(), false);
    assert_eq!(AE.is_moeum(), false);
    assert_eq!(YA.is_moeum(), false);
    assert_eq!(YAE.is_moeum(), false);
    assert_eq!(EO.is_moeum(), false);
    assert_eq!(E.is_moeum(), false);
    assert_eq!(YEO.is_moeum(), false);
    assert_eq!(YE.is_moeum(), false);
    assert_eq!(O.is_moeum(), false);
    assert_eq!(WA.is_moeum(), false);
    assert_eq!(WAE.is_moeum(), false);
    assert_eq!(OE.is_moeum(), false);
    assert_eq!(YO.is_moeum(), false);
    assert_eq!(U.is_moeum(), false);
    assert_eq!(WEO.is_moeum(), false);
    assert_eq!(WE.is_moeum(), false);
    assert_eq!(WI.is_moeum(), false);
    assert_eq!(YU.is_moeum(), false);
    assert_eq!(EU.is_moeum(), false);
    assert_eq!(YI.is_moeum(), false);
    assert_eq!(I.is_moeum(), false);
  }

  #[test]
  fn to_index() {
    assert_eq!(A.to_index(), 0);
    assert_eq!(AE.to_index(), 1);
    assert_eq!(YA.to_index(), 2);
    assert_eq!(YAE.to_index(), 3);
    assert_eq!(EO.to_index(), 4);
    assert_eq!(E.to_index(), 5);
    assert_eq!(YEO.to_index(), 6);
    assert_eq!(YE.to_index(), 7);
    assert_eq!(O.to_index(), 8);
    assert_eq!(WA.to_index(), 9);
    assert_eq!(WAE.to_index(), 10);
    assert_eq!(OE.to_index(), 11);
    assert_eq!(YO.to_index(), 12);
    assert_eq!(U.to_index(), 13);
    assert_eq!(WEO.to_index(), 14);
    assert_eq!(WE.to_index(), 15);
    assert_eq!(WI.to_index(), 16);
    assert_eq!(YU.to_index(), 17);
    assert_eq!(EU.to_index(), 18);
    assert_eq!(YI.to_index(), 19);
    assert_eq!(I.to_index(), 20);
  }

  #[test]
  fn to_composable() {
    assert_eq!(A.to_composable(), 0);
    assert_eq!(AE.to_composable(), 21);
    assert_eq!(YA.to_composable(), 42);
    assert_eq!(YAE.to_composable(), 63);
    assert_eq!(EO.to_composable(), 84);
    assert_eq!(E.to_composable(), 105);
    assert_eq!(YEO.to_composable(), 126);
    assert_eq!(YE.to_composable(), 147);
    assert_eq!(O.to_composable(), 168);
    assert_eq!(WA.to_composable(), 189);
    assert_eq!(WAE.to_composable(), 210);
    assert_eq!(OE.to_composable(), 231);
    assert_eq!(YO.to_composable(), 252);
    assert_eq!(U.to_composable(), 273);
    assert_eq!(WEO.to_composable(), 294);
    assert_eq!(WE.to_composable(), 315);
    assert_eq!(WI.to_composable(), 336);
    assert_eq!(YU.to_composable(), 357);
    assert_eq!(EU.to_composable(), 378);
    assert_eq!(YI.to_composable(), 399);
    assert_eq!(I.to_composable(), 420);
  }
}
