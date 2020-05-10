use crate::characters::{JongseongCharacter::*, *};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Jongseong {
    Normal(JongseongCharacter),
    Compat(JongseongCharacter),
}

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
        match item {
            Jongseong::Normal(character) => character.clone(),
            Jongseong::Compat(character) => character.clone(),
        }
    }
}

impl From<&JongseongCharacter> for Jongseong {
    fn from(item: &JongseongCharacter) -> Jongseong {
        Jongseong::Normal(item.clone())
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

    #[test]
    fn is_choseong_with_u32() {
        assert_eq!(0x11A7.is_jongseong(), false);
        assert_eq!(0x11A8.is_jongseong(), true);
        assert_eq!(0x11A9.is_jongseong(), true);
        assert_eq!(0x11AA.is_jongseong(), true);
        assert_eq!(0x11AB.is_jongseong(), true);
        assert_eq!(0x11AC.is_jongseong(), true);
        assert_eq!(0x11AD.is_jongseong(), true);
        assert_eq!(0x11AE.is_jongseong(), true);
        assert_eq!(0x11AF.is_jongseong(), true);
        assert_eq!(0x11B0.is_jongseong(), true);
        assert_eq!(0x11B1.is_jongseong(), true);
        assert_eq!(0x11B2.is_jongseong(), true);
        assert_eq!(0x11B3.is_jongseong(), true);
        assert_eq!(0x11B4.is_jongseong(), true);
        assert_eq!(0x11B5.is_jongseong(), true);
        assert_eq!(0x11B6.is_jongseong(), true);
        assert_eq!(0x11B7.is_jongseong(), true);
        assert_eq!(0x11B8.is_jongseong(), true);
        assert_eq!(0x11B9.is_jongseong(), true);
        assert_eq!(0x11BA.is_jongseong(), true);
        assert_eq!(0x11BB.is_jongseong(), true);
        assert_eq!(0x11BC.is_jongseong(), true);
        assert_eq!(0x11BD.is_jongseong(), true);
        assert_eq!(0x11BE.is_jongseong(), true);
        assert_eq!(0x11BF.is_jongseong(), true);
        assert_eq!(0x11C0.is_jongseong(), true);
        assert_eq!(0x11C1.is_jongseong(), true);
        assert_eq!(0x11C2.is_jongseong(), true);
        assert_eq!(0x11C3.is_jongseong(), false);

        assert_eq!(0x11A7.is_jongseong(), false);
        assert_eq!(0x3131.is_jongseong(), true);
        assert_eq!(0x3132.is_jongseong(), true);
        assert_eq!(0x3133.is_jongseong(), true);
        assert_eq!(0x3134.is_jongseong(), true);
        assert_eq!(0x3135.is_jongseong(), true);
        assert_eq!(0x3136.is_jongseong(), true);
        assert_eq!(0x3137.is_jongseong(), true);
        assert_eq!(0x3139.is_jongseong(), true);
        assert_eq!(0x313A.is_jongseong(), true);
        assert_eq!(0x313B.is_jongseong(), true);
        assert_eq!(0x313C.is_jongseong(), true);
        assert_eq!(0x313D.is_jongseong(), true);
        assert_eq!(0x313E.is_jongseong(), true);
        assert_eq!(0x313F.is_jongseong(), true);
        assert_eq!(0x3140.is_jongseong(), true);
        assert_eq!(0x3141.is_jongseong(), true);
        assert_eq!(0x3142.is_jongseong(), true);
        assert_eq!(0x3144.is_jongseong(), true);
        assert_eq!(0x3145.is_jongseong(), true);
        assert_eq!(0x3146.is_jongseong(), true);
        assert_eq!(0x3147.is_jongseong(), true);
        assert_eq!(0x3148.is_jongseong(), true);
        assert_eq!(0x314A.is_jongseong(), true);
        assert_eq!(0x314B.is_jongseong(), true);
        assert_eq!(0x314C.is_jongseong(), true);
        assert_eq!(0x314D.is_jongseong(), true);
        assert_eq!(0x314E.is_jongseong(), true);
        assert_eq!(0x11C3.is_jongseong(), false);
    }

    #[test]
    fn is_choseong_with_char() {
        assert_eq!('\u{11A7}'.is_jongseong(), false);
        assert_eq!('ᆨ'.is_jongseong(), true); // 0x11A8
        assert_eq!('ᆩ'.is_jongseong(), true); // 0x11A9
        assert_eq!('ᆪ'.is_jongseong(), true); // 0x11AA
        assert_eq!('ᆫ'.is_jongseong(), true); // 0x11AB
        assert_eq!('ᆬ'.is_jongseong(), true); // 0x11AC
        assert_eq!('ᆭ'.is_jongseong(), true); // 0x11AD
        assert_eq!('ᆮ'.is_jongseong(), true); // 0x11AE
        assert_eq!('ᆯ'.is_jongseong(), true); // 0x11AF
        assert_eq!('ᆰ'.is_jongseong(), true); // 0x11B0
        assert_eq!('ᆱ'.is_jongseong(), true); // 0x11B1
        assert_eq!('ᆲ'.is_jongseong(), true); // 0x11B2
        assert_eq!('ᆳ'.is_jongseong(), true); // 0x11B3
        assert_eq!('ᆴ'.is_jongseong(), true); // 0x11B4
        assert_eq!('ᆵ'.is_jongseong(), true); // 0x11B5
        assert_eq!('ᆶ'.is_jongseong(), true); // 0x11B6
        assert_eq!('ᆷ'.is_jongseong(), true); // 0x11B7
        assert_eq!('ᆸ'.is_jongseong(), true); // 0x11B8
        assert_eq!('ᆹ'.is_jongseong(), true); // 0x11B9
        assert_eq!('ᆺ'.is_jongseong(), true); // 0x11BA
        assert_eq!('ᆻ'.is_jongseong(), true); // 0x11BB
        assert_eq!('ᆼ'.is_jongseong(), true); // 0x11BC
        assert_eq!('ᆽ'.is_jongseong(), true); // 0x11BD
        assert_eq!('ᆾ'.is_jongseong(), true); // 0x11BE
        assert_eq!('ᆿ'.is_jongseong(), true); // 0x11BF
        assert_eq!('ᇀ'.is_jongseong(), true); // 0x11C0
        assert_eq!('ᇁ'.is_jongseong(), true); // 0x11C1
        assert_eq!('ᇂ'.is_jongseong(), true); // 0x11C2
        assert_eq!('\u{11C3}'.is_jongseong(), false);

        assert_eq!('\u{11A7}'.is_jongseong(), false);
        assert_eq!('ㄱ'.is_jongseong(), true); // 0x3131
        assert_eq!('ㄲ'.is_jongseong(), true); // 0x3132
        assert_eq!('ㄳ'.is_jongseong(), true); // 0x3133
        assert_eq!('ㄴ'.is_jongseong(), true); // 0x3134
        assert_eq!('ㄵ'.is_jongseong(), true); // 0x3135
        assert_eq!('ㄶ'.is_jongseong(), true); // 0x3136
        assert_eq!('ㄷ'.is_jongseong(), true); // 0x3137
        assert_eq!('ㄹ'.is_jongseong(), true); // 0x3139
        assert_eq!('ㄺ'.is_jongseong(), true); // 0x313A
        assert_eq!('ㄻ'.is_jongseong(), true); // 0x313B
        assert_eq!('ㄼ'.is_jongseong(), true); // 0x313C
        assert_eq!('ㄽ'.is_jongseong(), true); // 0x313D
        assert_eq!('ㄾ'.is_jongseong(), true); // 0x313E
        assert_eq!('ㄿ'.is_jongseong(), true); // 0x313F
        assert_eq!('ㅀ'.is_jongseong(), true); // 0x3140
        assert_eq!('ㅁ'.is_jongseong(), true); // 0x3141
        assert_eq!('ㅂ'.is_jongseong(), true); // 0x3142
        assert_eq!('ㅄ'.is_jongseong(), true); // 0x3144
        assert_eq!('ㅅ'.is_jongseong(), true); // 0x3145
        assert_eq!('ㅆ'.is_jongseong(), true); // 0x3146
        assert_eq!('ㅇ'.is_jongseong(), true); // 0x3147
        assert_eq!('ㅈ'.is_jongseong(), true); // 0x3148
        assert_eq!('ㅊ'.is_jongseong(), true); // 0x314A
        assert_eq!('ㅋ'.is_jongseong(), true); // 0x314B
        assert_eq!('ㅌ'.is_jongseong(), true); // 0x314C
        assert_eq!('ㅍ'.is_jongseong(), true); // 0x314D
        assert_eq!('ㅎ'.is_jongseong(), true); // 0x314E
        assert_eq!('\u{11C3}'.is_jongseong(), false);
    }
}
