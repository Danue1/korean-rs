use crate::characters::{JongseongCharacter::*, *};
use crate::constants::*;
use std::convert::{From, TryFrom};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Jongseong(JongseongCharacter);

pub trait JongseongInformation {
  fn is_jongseong(&self) -> bool;
  fn has_jongseong(&self) -> bool;
}

impl JongseongInformation for u32 {
  fn is_jongseong(&self) -> bool {
    Jongseong::try_from(*self).is_ok()
  }

  fn has_jongseong(&self) -> bool {
    JongseongCharacter::to_code(*self).is_jongseong()
  }
}

impl JongseongInformation for char {
  fn is_jongseong(&self) -> bool {
    (*self as u32).is_jongseong()
  }

  fn has_jongseong(&self) -> bool {
    (*self as u32).has_jongseong()
  }
}

impl From<&Jongseong> for JongseongCharacter {
  fn from(item: &Jongseong) -> JongseongCharacter {
    item.0.clone()
  }
}

impl From<&JongseongCharacter> for Jongseong {
  fn from(item: &JongseongCharacter) -> Jongseong {
    Jongseong(item.clone())
  }
}

impl TryFrom<u32> for Jongseong {
  type Error = ();

  fn try_from(item: u32) -> Result<Self, Self::Error> {
    let character = match JongseongCharacter::to_code(item) {
      0x11A8 => Giyeok,
      0x11A9 => SsangGiyeok,
      0x11AA => GiyeokSiot,
      0x11AB => Nieun,
      0x11AC => NieunJieut,
      0x11AD => NieunHieuh,
      0x11AE => Digeut,
      0x11AF => Rieul,
      0x11B0 => RieulGiyeok,
      0x11B1 => RieulMieum,
      0x11B2 => RieulBieup,
      0x11B3 => RieulSiot,
      0x11B4 => RieulTieut,
      0x11B5 => RieulPieup,
      0x11B6 => RieulHieuh,
      0x11B7 => Mieum,
      0x11B8 => Bieup,
      0x11B9 => BieupSiot,
      0x11BA => Siot,
      0x11BB => SsangSiot,
      0x11BC => Ieung,
      0x11BD => Jieut,
      0x11BE => Chieut,
      0x11BF => Kieuk,
      0x11C0 => Tieut,
      0x11C1 => Pieup,
      0x11C2 => Hieuh,
      _ => return Err(()),
    };

    Ok(Jongseong(character))
  }
}

impl From<&Jongseong> for u32 {
  fn from(item: &Jongseong) -> u32 {
    item.0.to_index() + JONGSEONG_START
  }
}

impl TryFrom<char> for Jongseong {
  type Error = ();

  fn try_from(item: char) -> Result<Self, Self::Error> {
    Jongseong::try_from(item as u32)
  }
}

impl From<&Jongseong> for char {
  fn from(item: &Jongseong) -> char {
    match item.0 {
      Giyeok => '\u{11A8}',
      SsangGiyeok => '\u{11A9}',
      GiyeokSiot => '\u{11AA}',
      Nieun => '\u{11AB}',
      NieunJieut => '\u{11AC}',
      NieunHieuh => '\u{11AD}',
      Digeut => '\u{11AE}',
      Rieul => '\u{11AF}',
      RieulGiyeok => '\u{11B0}',
      RieulMieum => '\u{11B1}',
      RieulBieup => '\u{11B2}',
      RieulSiot => '\u{11B3}',
      RieulTieut => '\u{11B4}',
      RieulPieup => '\u{11B5}',
      RieulHieuh => '\u{11B6}',
      Mieum => '\u{11B7}',
      Bieup => '\u{11B8}',
      BieupSiot => '\u{11B9}',
      Siot => '\u{11BA}',
      SsangSiot => '\u{11BB}',
      Ieung => '\u{11BC}',
      Jieut => '\u{11BD}',
      Chieut => '\u{11BE}',
      Kieuk => '\u{11BF}',
      Tieut => '\u{11C0}',
      Pieup => '\u{11C1}',
      Hieuh => '\u{11C2}',
    }
  }
}

impl CharacterInformation for Jongseong {
  fn is_jaeum(&self) -> bool {
    self.0.is_jaeum()
  }

  fn is_moeum(&self) -> bool {
    self.0.is_moeum()
  }

  fn to_composable(&self) -> u32 {
    self.0.to_composable()
  }
}
