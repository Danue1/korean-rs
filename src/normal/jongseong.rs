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
    }
}
