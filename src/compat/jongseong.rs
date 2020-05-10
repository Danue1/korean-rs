use crate::characters::{JongseongCharacter::*, *};
use std::convert::{From, TryFrom};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompatJongseong(JongseongCharacter);

pub trait CompatJongseongInformation {
    fn is_compat_jongseong(&self) -> bool;
    fn has_compat_jongseong(&self) -> bool;
}

impl CompatJongseongInformation for u32 {
    fn is_compat_jongseong(&self) -> bool {
        CompatJongseong::try_from(*self).is_ok()
    }

    fn has_compat_jongseong(&self) -> bool {
        JongseongCharacter::to_code(*self).is_compat_jongseong()
    }
}

impl CompatJongseongInformation for char {
    fn is_compat_jongseong(&self) -> bool {
        (*self as u32).is_compat_jongseong()
    }

    fn has_compat_jongseong(&self) -> bool {
        (*self as u32).has_compat_jongseong()
    }
}

impl From<&CompatJongseong> for JongseongCharacter {
    fn from(item: &CompatJongseong) -> JongseongCharacter {
        item.0.clone()
    }
}

impl From<&JongseongCharacter> for CompatJongseong {
    fn from(item: &JongseongCharacter) -> CompatJongseong {
        CompatJongseong(item.clone())
    }
}

impl TryFrom<u32> for CompatJongseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match JongseongCharacter::to_code(item) {
            0x3131 => Giyeok,
            0x3132 => SsangGiyeok,
            0x3133 => GiyeokSiot,
            0x3134 => Nieun,
            0x3135 => NieunJieut,
            0x3136 => NieunHieuh,
            0x3137 => Digeut,
            0x3139 => Rieul,
            0x313A => RieulGiyeok,
            0x313B => RieulMieum,
            0x313C => RieulBieup,
            0x313D => RieulSiot,
            0x313E => RieulTieut,
            0x313F => RieulPieup,
            0x3140 => RieulHieuh,
            0x3141 => Mieum,
            0x3142 => Bieup,
            0x3144 => BieupSiot,
            0x3145 => Siot,
            0x3146 => SsangSiot,
            0x3147 => Ieung,
            0x3148 => Jieut,
            0x314A => Chieut,
            0x314B => Kieuk,
            0x314C => Tieut,
            0x314D => Pieup,
            0x314E => Hieuh,
            _ => return Err(()),
        };

        Ok(CompatJongseong(character))
    }
}

impl From<&CompatJongseong> for u32 {
    fn from(item: &CompatJongseong) -> u32 {
        match item.0 {
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
        }
    }
}

impl TryFrom<char> for CompatJongseong {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        CompatJongseong::try_from(item as u32)
    }
}

impl From<&CompatJongseong> for char {
    fn from(item: &CompatJongseong) -> char {
        match item.0 {
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
        }
    }
}

impl CharacterInformation for CompatJongseong {
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
