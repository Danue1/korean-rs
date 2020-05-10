use crate::characters::{ChoseongCharacter::*, *};
use std::convert::{From, TryFrom};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompatChoseong(ChoseongCharacter);

pub trait CompatChoseongInformation {
    fn is_compat_choseong(&self) -> bool;
    fn has_compat_choseong(&self) -> bool;
}

impl CompatChoseongInformation for u32 {
    fn is_compat_choseong(&self) -> bool {
        CompatChoseong::try_from(*self).is_ok()
    }

    fn has_compat_choseong(&self) -> bool {
        ChoseongCharacter::to_code(*self).is_compat_choseong()
    }
}

impl CompatChoseongInformation for char {
    fn is_compat_choseong(&self) -> bool {
        (*self as u32).is_compat_choseong()
    }

    fn has_compat_choseong(&self) -> bool {
        (*self as u32).has_compat_choseong()
    }
}

impl From<&CompatChoseong> for ChoseongCharacter {
    fn from(item: &CompatChoseong) -> ChoseongCharacter {
        item.0.clone()
    }
}

impl From<&ChoseongCharacter> for CompatChoseong {
    fn from(item: &ChoseongCharacter) -> CompatChoseong {
        CompatChoseong(item.clone())
    }
}

impl TryFrom<u32> for CompatChoseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match ChoseongCharacter::to_code(item) {
            0x3131 => Giyeok,
            0x3132 => SsangGiyeok,
            0x3134 => Nieun,
            0x3137 => Digeut,
            0x3138 => SsangDigeut,
            0x3139 => Rieul,
            0x3141 => Mieum,
            0x3142 => Bieup,
            0x3143 => SsangBieup,
            0x3145 => Siot,
            0x3146 => SsangSiot,
            0x3147 => Ieung,
            0x3148 => Jieut,
            0x3149 => SsangJieut,
            0x313A => Chieut,
            0x314B => Kiyeok,
            0x314C => Tieut,
            0x314D => Pieup,
            0x314E => Hieuh,
            _ => return Err(()),
        };

        Ok(CompatChoseong(character))
    }
}

impl From<&CompatChoseong> for u32 {
    fn from(item: &CompatChoseong) -> Self {
        match item.0 {
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
        }
    }
}

impl TryFrom<char> for CompatChoseong {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        CompatChoseong::try_from(item as u32)
    }
}

impl From<&CompatChoseong> for char {
    fn from(item: &CompatChoseong) -> char {
        match item.0 {
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
        }
    }
}

impl CharacterInformation for CompatChoseong {
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
