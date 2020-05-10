use crate::characters::{JongseongCharacter::*, *};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Jongseong {
    Normal(JongseongCharacter),
    Compat(JongseongCharacter),
}

pub trait JongseongInformation {
    fn is_jongseong(&self) -> bool;
    fn is_normal_jongseong(&self) -> bool;
    fn is_compat_jongseong(&self) -> bool;
    fn has_jongseong(&self) -> bool;
    fn has_normal_jongseong(&self) -> bool;
    fn has_compat_jongseong(&self) -> bool;
}

impl JongseongInformation for u32 {
    fn is_jongseong(&self) -> bool {
        Jongseong::try_from(*self).is_ok()
    }

    fn is_normal_jongseong(&self) -> bool {
        match Jongseong::try_from(*self) {
            Ok(jongseong) => match jongseong {
                Jongseong::Normal(_) => true,
                Jongseong::Compat(_) => false,
            },
            Err(_) => false,
        }
    }

    fn is_compat_jongseong(&self) -> bool {
        match Jongseong::try_from(*self) {
            Ok(jongseong) => match jongseong {
                Jongseong::Normal(_) => false,
                Jongseong::Compat(_) => true,
            },
            Err(_) => false,
        }
    }

    fn has_jongseong(&self) -> bool {
        JongseongCharacter::to_code(*self).is_jongseong()
    }

    fn has_normal_jongseong(&self) -> bool {
        JongseongCharacter::to_code(*self).is_normal_jongseong()
    }

    fn has_compat_jongseong(&self) -> bool {
        JongseongCharacter::to_code(*self).is_compat_jongseong()
    }
}

impl JongseongInformation for char {
    fn is_jongseong(&self) -> bool {
        (*self as u32).is_jongseong()
    }

    fn is_normal_jongseong(&self) -> bool {
        (*self as u32).is_normal_jongseong()
    }

    fn is_compat_jongseong(&self) -> bool {
        (*self as u32).is_compat_jongseong()
    }

    fn has_jongseong(&self) -> bool {
        (*self as u32).has_jongseong()
    }

    fn has_normal_jongseong(&self) -> bool {
        (*self as u32).has_normal_jongseong()
    }

    fn has_compat_jongseong(&self) -> bool {
        (*self as u32).has_compat_jongseong()
    }
}

impl From<&Jongseong> for JongseongCharacter {
    fn from(item: &Jongseong) -> JongseongCharacter {
        match item {
            Jongseong::Normal(character) => character.clone(),
            Jongseong::Compat(character) => character.clone(),
        }
    }
}

impl JongseongCharacter {
    pub fn to_normal(&self) -> Jongseong {
        Jongseong::Normal(self.clone())
    }

    pub fn to_compat(&self) -> Jongseong {
        Jongseong::Compat(self.clone())
    }
}

impl TryFrom<u32> for Jongseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match JongseongCharacter::to_code(item) {
            0x11A8 => Jongseong::Normal(Giyeok),
            0x11A9 => Jongseong::Normal(SsangGiyeok),
            0x11AA => Jongseong::Normal(GiyeokSiot),
            0x11AB => Jongseong::Normal(Nieun),
            0x11AC => Jongseong::Normal(NieunJieut),
            0x11AD => Jongseong::Normal(NieunHieuh),
            0x11AE => Jongseong::Normal(Digeut),
            0x11AF => Jongseong::Normal(Rieul),
            0x11B0 => Jongseong::Normal(RieulGiyeok),
            0x11B1 => Jongseong::Normal(RieulMieum),
            0x11B2 => Jongseong::Normal(RieulBieup),
            0x11B3 => Jongseong::Normal(RieulSiot),
            0x11B4 => Jongseong::Normal(RieulTieut),
            0x11B5 => Jongseong::Normal(RieulPieup),
            0x11B6 => Jongseong::Normal(RieulHieuh),
            0x11B7 => Jongseong::Normal(Mieum),
            0x11B8 => Jongseong::Normal(Bieup),
            0x11B9 => Jongseong::Normal(BieupSiot),
            0x11BA => Jongseong::Normal(Siot),
            0x11BB => Jongseong::Normal(SsangSiot),
            0x11BC => Jongseong::Normal(Ieung),
            0x11BD => Jongseong::Normal(Jieut),
            0x11BE => Jongseong::Normal(Chieut),
            0x11BF => Jongseong::Normal(Kieuk),
            0x11C0 => Jongseong::Normal(Tieut),
            0x11C1 => Jongseong::Normal(Pieup),
            0x11C2 => Jongseong::Normal(Hieuh),

            0x3131 => Jongseong::Compat(Giyeok),
            0x3132 => Jongseong::Compat(SsangGiyeok),
            0x3133 => Jongseong::Compat(GiyeokSiot),
            0x3134 => Jongseong::Compat(Nieun),
            0x3135 => Jongseong::Compat(NieunJieut),
            0x3136 => Jongseong::Compat(NieunHieuh),
            0x3137 => Jongseong::Compat(Digeut),
            0x3139 => Jongseong::Compat(Rieul),
            0x313A => Jongseong::Compat(RieulGiyeok),
            0x313B => Jongseong::Compat(RieulMieum),
            0x313C => Jongseong::Compat(RieulBieup),
            0x313D => Jongseong::Compat(RieulSiot),
            0x313E => Jongseong::Compat(RieulTieut),
            0x313F => Jongseong::Compat(RieulPieup),
            0x3140 => Jongseong::Compat(RieulHieuh),
            0x3141 => Jongseong::Compat(Mieum),
            0x3142 => Jongseong::Compat(Bieup),
            0x3144 => Jongseong::Compat(BieupSiot),
            0x3145 => Jongseong::Compat(Siot),
            0x3146 => Jongseong::Compat(SsangSiot),
            0x3147 => Jongseong::Compat(Ieung),
            0x3148 => Jongseong::Compat(Jieut),
            0x314A => Jongseong::Compat(Chieut),
            0x314B => Jongseong::Compat(Kieuk),
            0x314C => Jongseong::Compat(Tieut),
            0x314D => Jongseong::Compat(Pieup),
            0x314E => Jongseong::Compat(Hieuh),
            _ => return Err(()),
        };

        Ok(character)
    }
}

impl From<&Jongseong> for u32 {
    fn from(item: &Jongseong) -> Self {
        match item {
            Jongseong::Normal(character) => match character {
                Giyeok => 0x11A8,
                SsangGiyeok => 0x11A9,
                GiyeokSiot => 0x11AA,
                Nieun => 0x11AB,
                NieunJieut => 0x11AC,
                NieunHieuh => 0x11AD,
                Digeut => 0x11AE,
                Rieul => 0x11AF,
                RieulGiyeok => 0x11B0,
                RieulMieum => 0x11B1,
                RieulBieup => 0x11B2,
                RieulSiot => 0x11B3,
                RieulTieut => 0x11B4,
                RieulPieup => 0x11B5,
                RieulHieuh => 0x11B6,
                Mieum => 0x11B7,
                Bieup => 0x11B8,
                BieupSiot => 0x11B9,
                Siot => 0x11BA,
                SsangSiot => 0x11BB,
                Ieung => 0x11BC,
                Jieut => 0x11BD,
                Chieut => 0x11BE,
                Kieuk => 0x11BF,
                Tieut => 0x11C0,
                Pieup => 0x11C1,
                Hieuh => 0x11C2,
            },
            Jongseong::Compat(character) => match character {
                Giyeok => 0x3131,
                SsangGiyeok => 0x3132,
                GiyeokSiot => 0x3133,
                Nieun => 0x3134,
                NieunJieut => 0x3135,
                NieunHieuh => 0x3136,
                Digeut => 0x3137,
                Rieul => 0x3139,
                RieulGiyeok => 0x313A,
                RieulMieum => 0x313B,
                RieulBieup => 0x313C,
                RieulSiot => 0x313D,
                RieulTieut => 0x313E,
                RieulPieup => 0x313F,
                RieulHieuh => 0x3140,
                Mieum => 0x3141,
                Bieup => 0x3142,
                BieupSiot => 0x3144,
                Siot => 0x3145,
                SsangSiot => 0x3146,
                Ieung => 0x3147,
                Jieut => 0x3148,
                Chieut => 0x314A,
                Kieuk => 0x314B,
                Tieut => 0x314C,
                Pieup => 0x314D,
                Hieuh => 0x314E,
            },
        }
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
        match item {
            Jongseong::Normal(character) => match character {
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
            },
            Jongseong::Compat(character) => match character {
                Giyeok => '\u{3131}',
                SsangGiyeok => '\u{3132}',
                GiyeokSiot => '\u{3133}',
                Nieun => '\u{3134}',
                NieunJieut => '\u{3135}',
                NieunHieuh => '\u{3136}',
                Digeut => '\u{3137}',
                Rieul => '\u{3139}',
                RieulGiyeok => '\u{313A}',
                RieulMieum => '\u{313B}',
                RieulBieup => '\u{313C}',
                RieulSiot => '\u{313D}',
                RieulTieut => '\u{313E}',
                RieulPieup => '\u{313F}',
                RieulHieuh => '\u{3140}',
                Mieum => '\u{3141}',
                Bieup => '\u{3142}',
                BieupSiot => '\u{3144}',
                Siot => '\u{3145}',
                SsangSiot => '\u{3146}',
                Ieung => '\u{3147}',
                Jieut => '\u{3148}',
                Chieut => '\u{314A}',
                Kieuk => '\u{314B}',
                Tieut => '\u{314C}',
                Pieup => '\u{314D}',
                Hieuh => '\u{314E}',
            },
        }
    }
}

impl CharacterInformation for Jongseong {
    fn is_jaeum(&self) -> bool {
        match self {
            Jongseong::Normal(character) => character.is_jaeum(),
            Jongseong::Compat(character) => character.is_jaeum(),
        }
    }

    fn is_moeum(&self) -> bool {
        match self {
            Jongseong::Normal(character) => character.is_moeum(),
            Jongseong::Compat(character) => character.is_moeum(),
        }
    }

    fn to_composable(&self) -> u32 {
        match self {
            Jongseong::Normal(character) => character.to_composable(),
            Jongseong::Compat(character) => character.to_composable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NON_NORMAL_JONGSEONG_START_U32: u32 = 0x11A7;
    const NON_NORMAL_JONGSEONG_END_U32: u32 = 0x11C3;
    const NORMAL_JONGSEONG_U32_LIST: [u32; 27] = [
        0x11A8, 0x11A9, 0x11AA, 0x11AB, 0x11AC, 0x11AD, 0x11AE, 0x11AF, 0x11B0, 0x11B1, 0x11B2,
        0x11B3, 0x11B4, 0x11B5, 0x11B6, 0x11B7, 0x11B8, 0x11B9, 0x11BA, 0x11BB, 0x11BC, 0x11BD,
        0x11BE, 0x11BF, 0x11C0, 0x11C1, 0x11C2,
    ];

    const NON_COMPAT_JONGSEONG_START_U32: u32 = 0x11A7;
    const NON_COMPAT_JONGSEONG_END_U32: u32 = 0x11C3;
    const COMPAT_JONGSEONG_U32_LIST: [u32; 27] = [
        0x3131, 0x3132, 0x3133, 0x3134, 0x3135, 0x3136, 0x3137, 0x3139, 0x313A, 0x313B, 0x313C,
        0x313D, 0x313E, 0x313F, 0x3140, 0x3141, 0x3142, 0x3144, 0x3145, 0x3146, 0x3147, 0x3148,
        0x314A, 0x314B, 0x314C, 0x314D, 0x314E,
    ];

    const NON_NORMAL_JONGSEONG_START_CHAR: char = '\u{11A7}';
    const NON_NORMAL_JONGSEONG_END_CHAR: char = '\u{11C3}';
    const NORMAL_JONGSEONG_CHAR_LIST: [char; 27] = [
        'ᆨ', 'ᆩ', 'ᆪ', 'ᆫ', 'ᆬ', 'ᆭ', 'ᆮ', 'ᆯ', 'ᆰ', 'ᆱ', 'ᆲ', 'ᆳ', 'ᆴ', 'ᆵ', 'ᆶ', 'ᆷ', 'ᆸ', 'ᆹ', 'ᆺ', 'ᆻ', 'ᆼ', 'ᆽ', 'ᆾ',
        'ᆿ', 'ᇀ', 'ᇁ', 'ᇂ',
    ];

    const NON_COMPAT_JONGSEONG_START_CHAR: char = '\u{11A7}';
    const NON_COMPAT_JONGSEONG_END_CHAR: char = '\u{11C3}';
    const COMPAT_JONGSEONG_CHAR_LIST: [char; 27] = [
        'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ',
        'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ];

    #[test]
    fn is_jongseong_with_u32() {
        assert_eq!(NON_NORMAL_JONGSEONG_START_U32.is_jongseong(), false);
        for jongseong in NORMAL_JONGSEONG_U32_LIST.iter() {
            assert_eq!(jongseong.is_jongseong(), true);
        }
        assert_eq!(NON_NORMAL_JONGSEONG_END_U32.is_jongseong(), false);

        assert_eq!(NON_COMPAT_JONGSEONG_START_U32.is_jongseong(), false);
        for jongseong in COMPAT_JONGSEONG_U32_LIST.iter() {
            assert_eq!(jongseong.is_jongseong(), true);
        }
        assert_eq!(NON_COMPAT_JONGSEONG_END_U32.is_jongseong(), false);
    }

    #[test]
    fn is_jongseong_with_char() {
        assert_eq!(NON_NORMAL_JONGSEONG_START_CHAR.is_jongseong(), false);
        for jongseong in NORMAL_JONGSEONG_CHAR_LIST.iter() {
            assert_eq!(jongseong.is_jongseong(), true);
        }
        assert_eq!(NON_NORMAL_JONGSEONG_END_CHAR.is_jongseong(), false);

        assert_eq!(NON_COMPAT_JONGSEONG_START_CHAR.is_jongseong(), false);
        for jongseong in COMPAT_JONGSEONG_CHAR_LIST.iter() {
            assert_eq!(jongseong.is_jongseong(), true);
        }
        assert_eq!(NON_COMPAT_JONGSEONG_END_CHAR.is_jongseong(), false);
    }

    #[test]
    fn is_normal_jongseong_with_u32() {
        assert_eq!(NON_NORMAL_JONGSEONG_START_U32.is_normal_jongseong(), false);
        for jongseong in NORMAL_JONGSEONG_U32_LIST.iter() {
            assert_eq!(jongseong.is_normal_jongseong(), true);
        }
        assert_eq!(NON_NORMAL_JONGSEONG_END_U32.is_normal_jongseong(), false);

        assert_eq!(NON_COMPAT_JONGSEONG_START_U32.is_normal_jongseong(), false);
        for jongseong in COMPAT_JONGSEONG_U32_LIST.iter() {
            assert_eq!(jongseong.is_normal_jongseong(), false);
        }
        assert_eq!(NON_COMPAT_JONGSEONG_END_U32.is_normal_jongseong(), false);
    }

    #[test]
    fn is_normal_jongseong_with_char() {
        assert_eq!(NON_NORMAL_JONGSEONG_START_CHAR.is_normal_jongseong(), false);
        for jongseong in NORMAL_JONGSEONG_CHAR_LIST.iter() {
            assert_eq!(jongseong.is_normal_jongseong(), true);
        }
        assert_eq!(NON_NORMAL_JONGSEONG_END_CHAR.is_normal_jongseong(), false);

        assert_eq!(NON_COMPAT_JONGSEONG_START_CHAR.is_normal_jongseong(), false);
        for jongseong in COMPAT_JONGSEONG_CHAR_LIST.iter() {
            assert_eq!(jongseong.is_normal_jongseong(), false);
        }
        assert_eq!(NON_COMPAT_JONGSEONG_END_CHAR.is_normal_jongseong(), false);
    }

    #[test]
    fn is_compat_jongseong_with_u32() {
        assert_eq!(NON_NORMAL_JONGSEONG_START_U32.is_compat_jongseong(), false);
        for jongseong in NORMAL_JONGSEONG_U32_LIST.iter() {
            assert_eq!(jongseong.is_compat_jongseong(), false);
        }
        assert_eq!(NON_NORMAL_JONGSEONG_END_U32.is_compat_jongseong(), false);

        assert_eq!(NON_COMPAT_JONGSEONG_START_U32.is_compat_jongseong(), false);
        for jongseong in COMPAT_JONGSEONG_U32_LIST.iter() {
            assert_eq!(jongseong.is_compat_jongseong(), true);
        }
        assert_eq!(NON_COMPAT_JONGSEONG_END_U32.is_compat_jongseong(), false);
    }

    #[test]
    fn is_compat_jongseong_with_char() {
        assert_eq!(NON_NORMAL_JONGSEONG_START_CHAR.is_compat_jongseong(), false);
        for jongseong in NORMAL_JONGSEONG_CHAR_LIST.iter() {
            assert_eq!(jongseong.is_compat_jongseong(), false);
        }
        assert_eq!(NON_NORMAL_JONGSEONG_END_CHAR.is_compat_jongseong(), false);

        assert_eq!(NON_COMPAT_JONGSEONG_START_CHAR.is_compat_jongseong(), false);
        for jongseong in COMPAT_JONGSEONG_CHAR_LIST.iter() {
            assert_eq!(jongseong.is_compat_jongseong(), true);
        }
        assert_eq!(NON_COMPAT_JONGSEONG_END_CHAR.is_compat_jongseong(), false);
    }
}
