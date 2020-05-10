use crate::characters::{JungseongCharacter::*, *};
use std::convert::{From, TryFrom};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompatJungseong(JungseongCharacter);

pub trait CompatJungseongInformation {
    fn is_compat_jungseong(&self) -> bool;
    fn has_compat_jungseong(&self) -> bool;
}

impl CompatJungseongInformation for u32 {
    fn is_compat_jungseong(&self) -> bool {
        CompatJungseong::try_from(*self).is_ok()
    }

    fn has_compat_jungseong(&self) -> bool {
        JungseongCharacter::to_code(*self).is_compat_jungseong()
    }
}

impl CompatJungseongInformation for char {
    fn is_compat_jungseong(&self) -> bool {
        (*self as u32).is_compat_jungseong()
    }

    fn has_compat_jungseong(&self) -> bool {
        (*self as u32).has_compat_jungseong()
    }
}

impl From<&CompatJungseong> for JungseongCharacter {
    fn from(item: &CompatJungseong) -> JungseongCharacter {
        item.0.clone()
    }
}

impl From<&JungseongCharacter> for CompatJungseong {
    fn from(item: &JungseongCharacter) -> CompatJungseong {
        CompatJungseong(item.clone())
    }
}

impl TryFrom<u32> for CompatJungseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match JungseongCharacter::to_code(item) {
            0x314F => A,
            0x3150 => AE,
            0x3151 => YA,
            0x3152 => YAE,
            0x3153 => EO,
            0x3154 => E,
            0x3155 => YEO,
            0x3156 => YE,
            0x3157 => O,
            0x3158 => WA,
            0x3159 => WAE,
            0x315A => OE,
            0x315B => YO,
            0x315C => U,
            0x315D => WEO,
            0x315E => WE,
            0x315F => WI,
            0x3160 => YU,
            0x3161 => EU,
            0x3162 => YI,
            0x3163 => I,
            _ => return Err(()),
        };

        Ok(CompatJungseong(character))
    }
}

impl From<&CompatJungseong> for u32 {
    fn from(item: &CompatJungseong) -> u32 {
        match item.0 {
            A => 0x314F,
            AE => 0x3150,
            YA => 0x3151,
            YAE => 0x3152,
            EO => 0x3153,
            E => 0x3154,
            YEO => 0x3155,
            YE => 0x3156,
            O => 0x3157,
            WA => 0x3158,
            WAE => 0x3159,
            OE => 0x315A,
            YO => 0x315B,
            U => 0x315C,
            WEO => 0x315D,
            WE => 0x315E,
            WI => 0x315F,
            YU => 0x3160,
            EU => 0x3161,
            YI => 0x3162,
            I => 0x3163,
        }
    }
}

impl TryFrom<char> for CompatJungseong {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        CompatJungseong::try_from(item as u32)
    }
}

impl From<&CompatJungseong> for char {
    fn from(item: &CompatJungseong) -> char {
        match item.0 {
            A => '\u{314F}',
            AE => '\u{3150}',
            YA => '\u{3151}',
            YAE => '\u{3152}',
            EO => '\u{3153}',
            E => '\u{3154}',
            YEO => '\u{3155}',
            YE => '\u{3156}',
            O => '\u{3157}',
            WA => '\u{3158}',
            WAE => '\u{3159}',
            OE => '\u{315A}',
            YO => '\u{315B}',
            U => '\u{315C}',
            WEO => '\u{315D}',
            WE => '\u{315E}',
            WI => '\u{315F}',
            YU => '\u{3160}',
            EU => '\u{3161}',
            YI => '\u{3162}',
            I => '\u{3163}',
        }
    }
}

impl CharacterInformation for CompatJungseong {
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
        assert_eq!(0x1160.is_compat_jungseong(), false);
        assert_eq!(0x314F.is_compat_jungseong(), true);
        assert_eq!(0x3150.is_compat_jungseong(), true);
        assert_eq!(0x3151.is_compat_jungseong(), true);
        assert_eq!(0x3152.is_compat_jungseong(), true);
        assert_eq!(0x3153.is_compat_jungseong(), true);
        assert_eq!(0x3154.is_compat_jungseong(), true);
        assert_eq!(0x3155.is_compat_jungseong(), true);
        assert_eq!(0x3156.is_compat_jungseong(), true);
        assert_eq!(0x3157.is_compat_jungseong(), true);
        assert_eq!(0x3158.is_compat_jungseong(), true);
        assert_eq!(0x3159.is_compat_jungseong(), true);
        assert_eq!(0x315A.is_compat_jungseong(), true);
        assert_eq!(0x315B.is_compat_jungseong(), true);
        assert_eq!(0x315C.is_compat_jungseong(), true);
        assert_eq!(0x315D.is_compat_jungseong(), true);
        assert_eq!(0x315E.is_compat_jungseong(), true);
        assert_eq!(0x315F.is_compat_jungseong(), true);
        assert_eq!(0x3160.is_compat_jungseong(), true);
        assert_eq!(0x3161.is_compat_jungseong(), true);
        assert_eq!(0x3162.is_compat_jungseong(), true);
        assert_eq!(0x3163.is_compat_jungseong(), true);
        assert_eq!(0x1176.is_compat_jungseong(), false);
    }

    #[test]
    fn is_choseong_with_char() {
        assert_eq!('\u{1160}'.is_compat_jungseong(), false);
        assert_eq!('ㅏ'.is_compat_jungseong(), true); // 0x314F
        assert_eq!('ㅐ'.is_compat_jungseong(), true); // 0x3150
        assert_eq!('ㅑ'.is_compat_jungseong(), true); // 0x3151
        assert_eq!('ㅒ'.is_compat_jungseong(), true); // 0x3152
        assert_eq!('ㅓ'.is_compat_jungseong(), true); // 0x3153
        assert_eq!('ㅔ'.is_compat_jungseong(), true); // 0x3154
        assert_eq!('ㅕ'.is_compat_jungseong(), true); // 0x3155
        assert_eq!('ㅖ'.is_compat_jungseong(), true); // 0x3156
        assert_eq!('ㅗ'.is_compat_jungseong(), true); // 0x3157
        assert_eq!('ㅘ'.is_compat_jungseong(), true); // 0x3158
        assert_eq!('ㅙ'.is_compat_jungseong(), true); // 0x3159
        assert_eq!('ㅚ'.is_compat_jungseong(), true); // 0x315A
        assert_eq!('ㅛ'.is_compat_jungseong(), true); // 0x315B
        assert_eq!('ㅜ'.is_compat_jungseong(), true); // 0x315C
        assert_eq!('ㅝ'.is_compat_jungseong(), true); // 0x315D
        assert_eq!('ㅞ'.is_compat_jungseong(), true); // 0x315E
        assert_eq!('ㅟ'.is_compat_jungseong(), true); // 0x315F
        assert_eq!('ㅠ'.is_compat_jungseong(), true); // 0x3160
        assert_eq!('ㅡ'.is_compat_jungseong(), true); // 0x3161
        assert_eq!('ㅢ'.is_compat_jungseong(), true); // 0x3162
        assert_eq!('ㅣ'.is_compat_jungseong(), true); // 0x3163
        assert_eq!('\u{1176}'.is_compat_jungseong(), false);
    }
}
