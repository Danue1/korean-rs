use crate::characters::{ChoseongCharacter::*, *};
use crate::constants::*;
use std::convert::{From, TryFrom};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Choseong(ChoseongCharacter);

pub trait ChoseongInformation {
    fn is_choseong(&self) -> bool;
    fn has_choseong(&self) -> bool;
}

impl ChoseongInformation for u32 {
    fn is_choseong(&self) -> bool {
        Choseong::try_from(*self).is_ok()
    }

    fn has_choseong(&self) -> bool {
        ChoseongCharacter::to_code(*self).is_choseong()
    }
}

impl ChoseongInformation for char {
    fn is_choseong(&self) -> bool {
        (*self as u32).is_choseong()
    }

    fn has_choseong(&self) -> bool {
        (*self as u32).has_choseong()
    }
}

impl From<&Choseong> for ChoseongCharacter {
    fn from(item: &Choseong) -> ChoseongCharacter {
        item.0.clone()
    }
}

impl From<&ChoseongCharacter> for Choseong {
    fn from(item: &ChoseongCharacter) -> Choseong {
        Choseong(item.clone())
    }
}

impl TryFrom<u32> for Choseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match ChoseongCharacter::to_code(item) {
            0x1100 => Giyeok,
            0x1101 => SsangGiyeok,
            0x1102 => Nieun,
            0x1103 => Digeut,
            0x1104 => SsangDigeut,
            0x1105 => Rieul,
            0x1106 => Mieum,
            0x1107 => Bieup,
            0x1108 => SsangBieup,
            0x1109 => Siot,
            0x110A => SsangSiot,
            0x110B => Ieung,
            0x110C => Jieut,
            0x110D => SsangJieut,
            0x110E => Chieut,
            0x110F => Kiyeok,
            0x1110 => Tieut,
            0x1111 => Pieup,
            0x1112 => Hieuh,
            _ => return Err(()),
        };

        Ok(Choseong(character))
    }
}

impl From<&Choseong> for u32 {
    fn from(item: &Choseong) -> Self {
        item.0.to_index() + CHOSEONG_START
    }
}

impl TryFrom<char> for Choseong {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        Choseong::try_from(item as u32)
    }
}

impl From<&Choseong> for char {
    fn from(item: &Choseong) -> char {
        match item.0 {
            Giyeok => '\u{1100}',
            SsangGiyeok => '\u{1101}',
            Nieun => '\u{1102}',
            Digeut => '\u{1103}',
            SsangDigeut => '\u{1104}',
            Rieul => '\u{1105}',
            Mieum => '\u{1106}',
            Bieup => '\u{1107}',
            SsangBieup => '\u{1108}',
            Siot => '\u{1109}',
            SsangSiot => '\u{110A}',
            Ieung => '\u{110B}',
            Jieut => '\u{110C}',
            SsangJieut => '\u{110D}',
            Chieut => '\u{110E}',
            Kiyeok => '\u{110F}',
            Tieut => '\u{1110}',
            Pieup => '\u{1111}',
            Hieuh => '\u{1112}',
        }
    }
}

impl CharacterInformation for Choseong {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_choseong_with_u32() {
        assert_eq!(0x1099.is_choseong(), false);
        assert_eq!(0x1100.is_choseong(), true);
        assert_eq!(0x1101.is_choseong(), true);
        assert_eq!(0x1102.is_choseong(), true);
        assert_eq!(0x1103.is_choseong(), true);
        assert_eq!(0x1104.is_choseong(), true);
        assert_eq!(0x1105.is_choseong(), true);
        assert_eq!(0x1106.is_choseong(), true);
        assert_eq!(0x1107.is_choseong(), true);
        assert_eq!(0x1108.is_choseong(), true);
        assert_eq!(0x1109.is_choseong(), true);
        assert_eq!(0x110A.is_choseong(), true);
        assert_eq!(0x110B.is_choseong(), true);
        assert_eq!(0x110C.is_choseong(), true);
        assert_eq!(0x110D.is_choseong(), true);
        assert_eq!(0x110E.is_choseong(), true);
        assert_eq!(0x110F.is_choseong(), true);
        assert_eq!(0x1110.is_choseong(), true);
        assert_eq!(0x1111.is_choseong(), true);
        assert_eq!(0x1112.is_choseong(), true);
        assert_eq!(0x1113.is_choseong(), false);
    }

    #[test]
    fn is_choseong_with_char() {
        assert_eq!('\u{1099}'.is_choseong(), false);
        assert_eq!('ᄁ'.is_choseong(), true); // 0x1100
        assert_eq!('ᄀ'.is_choseong(), true); // 0x1101
        assert_eq!('ᄂ'.is_choseong(), true); // 0x1102
        assert_eq!('ᄃ'.is_choseong(), true); // 0x1103
        assert_eq!('ᄄ'.is_choseong(), true); // 0x1104
        assert_eq!('ᄅ'.is_choseong(), true); // 0x1105
        assert_eq!('ᄆ'.is_choseong(), true); // 0x1106
        assert_eq!('ᄇ'.is_choseong(), true); // 0x1107
        assert_eq!('ᄈ'.is_choseong(), true); // 0x1108
        assert_eq!('ᄉ'.is_choseong(), true); // 0x1109
        assert_eq!('ᄊ'.is_choseong(), true); // 0x110A
        assert_eq!('ᄋ'.is_choseong(), true); // 0x110B
        assert_eq!('ᄌ'.is_choseong(), true); // 0x110C
        assert_eq!('ᄍ'.is_choseong(), true); // 0x110D
        assert_eq!('ᄎ'.is_choseong(), true); // 0x110E
        assert_eq!('ᄏ'.is_choseong(), true); // 0x110F
        assert_eq!('ᄐ'.is_choseong(), true); // 0x1110
        assert_eq!('ᄑ'.is_choseong(), true); // 0x1111
        assert_eq!('ᄒ'.is_choseong(), true); // 0x1112
        assert_eq!('\u{1113}'.is_choseong(), false);
    }
}
