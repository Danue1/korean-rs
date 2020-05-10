use crate::characters::{ChoseongCharacter::*, *};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Choseong {
    Normal(ChoseongCharacter),
    Compat(ChoseongCharacter),
}

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
        match item {
            Choseong::Normal(character) => character.clone(),
            Choseong::Compat(character) => character.clone(),
        }
    }
}

impl From<&ChoseongCharacter> for Choseong {
    fn from(item: &ChoseongCharacter) -> Choseong {
        Choseong::Normal(item.clone())
    }
}

impl TryFrom<u32> for Choseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match ChoseongCharacter::to_code(item) {
            0x1100 => Choseong::Normal(Giyeok),
            0x1101 => Choseong::Normal(SsangGiyeok),
            0x1102 => Choseong::Normal(Nieun),
            0x1103 => Choseong::Normal(Digeut),
            0x1104 => Choseong::Normal(SsangDigeut),
            0x1105 => Choseong::Normal(Rieul),
            0x1106 => Choseong::Normal(Mieum),
            0x1107 => Choseong::Normal(Bieup),
            0x1108 => Choseong::Normal(SsangBieup),
            0x1109 => Choseong::Normal(Siot),
            0x110A => Choseong::Normal(SsangSiot),
            0x110B => Choseong::Normal(Ieung),
            0x110C => Choseong::Normal(Jieut),
            0x110D => Choseong::Normal(SsangJieut),
            0x110E => Choseong::Normal(Chieut),
            0x110F => Choseong::Normal(Kiyeok),
            0x1110 => Choseong::Normal(Tieut),
            0x1111 => Choseong::Normal(Pieup),
            0x1112 => Choseong::Normal(Hieuh),

            0x3131 => Choseong::Compat(Giyeok),
            0x3132 => Choseong::Compat(SsangGiyeok),
            0x3134 => Choseong::Compat(Nieun),
            0x3137 => Choseong::Compat(Digeut),
            0x3138 => Choseong::Compat(SsangDigeut),
            0x3139 => Choseong::Compat(Rieul),
            0x3141 => Choseong::Compat(Mieum),
            0x3142 => Choseong::Compat(Bieup),
            0x3143 => Choseong::Compat(SsangBieup),
            0x3145 => Choseong::Compat(Siot),
            0x3146 => Choseong::Compat(SsangSiot),
            0x3147 => Choseong::Compat(Ieung),
            0x3148 => Choseong::Compat(Jieut),
            0x3149 => Choseong::Compat(SsangJieut),
            0x313A => Choseong::Compat(Chieut),
            0x314B => Choseong::Compat(Kiyeok),
            0x314C => Choseong::Compat(Tieut),
            0x314D => Choseong::Compat(Pieup),
            0x314E => Choseong::Compat(Hieuh),
            _ => return Err(()),
        };

        Ok(character)
    }
}

impl From<&Choseong> for u32 {
    fn from(item: &Choseong) -> Self {
        match item {
            Choseong::Normal(character) => match character {
                Giyeok => 0x1100,
                SsangGiyeok => 0x1101,
                Nieun => 0x1102,
                Digeut => 0x1103,
                SsangDigeut => 0x1104,
                Rieul => 0x1105,
                Mieum => 0x1106,
                Bieup => 0x1107,
                SsangBieup => 0x1108,
                Siot => 0x1109,
                SsangSiot => 0x110A,
                Ieung => 0x110B,
                Jieut => 0x110C,
                SsangJieut => 0x110D,
                Chieut => 0x110E,
                Kiyeok => 0x110F,
                Tieut => 0x1110,
                Pieup => 0x1111,
                Hieuh => 0x1112,
            },
            Choseong::Compat(character) => match character {
                Giyeok => 0x3131,
                SsangGiyeok => 0x3132,
                Nieun => 0x3134,
                Digeut => 0x3137,
                SsangDigeut => 0x3138,
                Rieul => 0x3139,
                Mieum => 0x3141,
                Bieup => 0x3142,
                SsangBieup => 0x3143,
                Siot => 0x3145,
                SsangSiot => 0x3146,
                Ieung => 0x3147,
                Jieut => 0x3148,
                SsangJieut => 0x3149,
                Chieut => 0x313A,
                Kiyeok => 0x314B,
                Tieut => 0x314C,
                Pieup => 0x314D,
                Hieuh => 0x314E,
            },
        }
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
        match item {
            Choseong::Normal(character) => match character {
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
            },
            Choseong::Compat(character) => match character {
                Giyeok => '\u{3131}',
                SsangGiyeok => '\u{3132}',
                Nieun => '\u{3134}',
                Digeut => '\u{3137}',
                SsangDigeut => '\u{3138}',
                Rieul => '\u{3139}',
                Mieum => '\u{3141}',
                Bieup => '\u{3142}',
                SsangBieup => '\u{3143}',
                Siot => '\u{3145}',
                SsangSiot => '\u{3146}',
                Ieung => '\u{3147}',
                Jieut => '\u{3148}',
                SsangJieut => '\u{3149}',
                Chieut => '\u{313A}',
                Kiyeok => '\u{314B}',
                Tieut => '\u{314C}',
                Pieup => '\u{314D}',
                Hieuh => '\u{314E}',
            },
        }
    }
}

impl CharacterInformation for Choseong {
    fn is_jaeum(&self) -> bool {
        match self {
            Choseong::Normal(character) => character.is_jaeum(),
            Choseong::Compat(character) => character.is_jaeum(),
        }
    }

    fn is_moeum(&self) -> bool {
        match self {
            Choseong::Normal(character) => character.is_moeum(),
            Choseong::Compat(character) => character.is_moeum(),
        }
    }

    fn to_composable(&self) -> u32 {
        match self {
            Choseong::Normal(character) => character.to_composable(),
            Choseong::Compat(character) => character.to_composable(),
        }
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

        assert_eq!(0x1099.is_choseong(), false);
        assert_eq!(0x3131.is_choseong(), true);
        assert_eq!(0x3132.is_choseong(), true);
        assert_eq!(0x3134.is_choseong(), true);
        assert_eq!(0x3137.is_choseong(), true);
        assert_eq!(0x3138.is_choseong(), true);
        assert_eq!(0x3139.is_choseong(), true);
        assert_eq!(0x3141.is_choseong(), true);
        assert_eq!(0x3142.is_choseong(), true);
        assert_eq!(0x3143.is_choseong(), true);
        assert_eq!(0x3145.is_choseong(), true);
        assert_eq!(0x3146.is_choseong(), true);
        assert_eq!(0x3147.is_choseong(), true);
        assert_eq!(0x3148.is_choseong(), true);
        assert_eq!(0x3149.is_choseong(), true);
        assert_eq!(0x313A.is_choseong(), true);
        assert_eq!(0x314B.is_choseong(), true);
        assert_eq!(0x314C.is_choseong(), true);
        assert_eq!(0x314D.is_choseong(), true);
        assert_eq!(0x314E.is_choseong(), true);
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

        assert_eq!(0x1099.is_choseong(), false);
        assert_eq!('ㄱ'.is_choseong(), true); // 0x3131
        assert_eq!('ㄲ'.is_choseong(), true); // 0x3132
        assert_eq!('ㄴ'.is_choseong(), true); // 0x3134
        assert_eq!('ㄷ'.is_choseong(), true); // 0x3137
        assert_eq!('ㄸ'.is_choseong(), true); // 0x3138
        assert_eq!('ㄹ'.is_choseong(), true); // 0x3139
        assert_eq!('ㅁ'.is_choseong(), true); // 0x3141
        assert_eq!('ㅂ'.is_choseong(), true); // 0x3142
        assert_eq!('ㅃ'.is_choseong(), true); // 0x3143
        assert_eq!('ㅅ'.is_choseong(), true); // 0x3145
        assert_eq!('ㅆ'.is_choseong(), true); // 0x3146
        assert_eq!('ㅇ'.is_choseong(), true); // 0x3147
        assert_eq!('ㅈ'.is_choseong(), true); // 0x3148
        assert_eq!('ㅉ'.is_choseong(), true); // 0x3149
        assert_eq!('ㄺ'.is_choseong(), true); // 0x313A
        assert_eq!('ㅋ'.is_choseong(), true); // 0x314B
        assert_eq!('ㅌ'.is_choseong(), true); // 0x314C
        assert_eq!('ㅍ'.is_choseong(), true); // 0x314D
        assert_eq!('ㅎ'.is_choseong(), true); // 0x314E
        assert_eq!(0x1113.is_choseong(), false);
    }
}
