use crate::characters::{JungseongCharacter::*, *};
use crate::constants::*;
use std::convert::{From, TryFrom};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Jungseong(JungseongCharacter);

pub trait JungseongInformation {
    fn is_jungseong(&self) -> bool;
    fn has_jungseong(&self) -> bool;
}

impl JungseongInformation for u32 {
    fn is_jungseong(&self) -> bool {
        Jungseong::try_from(*self).is_ok()
    }

    fn has_jungseong(&self) -> bool {
        JungseongCharacter::to_code(*self).is_jungseong()
    }
}

impl JungseongInformation for char {
    fn is_jungseong(&self) -> bool {
        (*self as u32).is_jungseong()
    }

    fn has_jungseong(&self) -> bool {
        (*self as u32).has_jungseong()
    }
}

impl From<&Jungseong> for JungseongCharacter {
    fn from(item: &Jungseong) -> JungseongCharacter {
        item.0.clone()
    }
}

impl From<&JungseongCharacter> for Jungseong {
    fn from(item: &JungseongCharacter) -> Jungseong {
        Jungseong(item.clone())
    }
}

impl TryFrom<u32> for Jungseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match JungseongCharacter::to_code(item) {
            0x1161 => A,
            0x1162 => AE,
            0x1163 => YA,
            0x1164 => YAE,
            0x1165 => EO,
            0x1166 => E,
            0x1167 => YEO,
            0x1168 => YE,
            0x1169 => O,
            0x116A => WA,
            0x116B => WAE,
            0x116C => OE,
            0x116D => YO,
            0x116E => U,
            0x116F => WEO,
            0x1170 => WE,
            0x1171 => WI,
            0x1172 => YU,
            0x1173 => EU,
            0x1174 => YI,
            0x1175 => I,
            _ => return Err(()),
        };

        Ok(Jungseong(character))
    }
}

impl From<&Jungseong> for u32 {
    fn from(item: &Jungseong) -> u32 {
        item.0.to_index() + JUNGSEONG_START
    }
}

impl TryFrom<char> for Jungseong {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        Jungseong::try_from(item as u32)
    }
}

impl From<&Jungseong> for char {
    fn from(item: &Jungseong) -> char {
        match item.0 {
            A => '\u{1161}',
            AE => '\u{1162}',
            YA => '\u{1163}',
            YAE => '\u{1164}',
            EO => '\u{1165}',
            E => '\u{1166}',
            YEO => '\u{1167}',
            YE => '\u{1168}',
            O => '\u{1169}',
            WA => '\u{116A}',
            WAE => '\u{116B}',
            OE => '\u{116C}',
            YO => '\u{116D}',
            U => '\u{116E}',
            WEO => '\u{116F}',
            WE => '\u{1170}',
            WI => '\u{1171}',
            YU => '\u{1172}',
            EU => '\u{1173}',
            YI => '\u{1174}',
            I => '\u{1175}',
        }
    }
}

impl CharacterInformation for Jungseong {
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
        assert_eq!(0x1160.is_jungseong(), false);
        assert_eq!(0x1161.is_jungseong(), true);
        assert_eq!(0x1162.is_jungseong(), true);
        assert_eq!(0x1163.is_jungseong(), true);
        assert_eq!(0x1164.is_jungseong(), true);
        assert_eq!(0x1165.is_jungseong(), true);
        assert_eq!(0x1166.is_jungseong(), true);
        assert_eq!(0x1167.is_jungseong(), true);
        assert_eq!(0x1168.is_jungseong(), true);
        assert_eq!(0x1169.is_jungseong(), true);
        assert_eq!(0x116A.is_jungseong(), true);
        assert_eq!(0x116B.is_jungseong(), true);
        assert_eq!(0x116C.is_jungseong(), true);
        assert_eq!(0x116D.is_jungseong(), true);
        assert_eq!(0x116E.is_jungseong(), true);
        assert_eq!(0x116F.is_jungseong(), true);
        assert_eq!(0x1170.is_jungseong(), true);
        assert_eq!(0x1171.is_jungseong(), true);
        assert_eq!(0x1172.is_jungseong(), true);
        assert_eq!(0x1173.is_jungseong(), true);
        assert_eq!(0x1174.is_jungseong(), true);
        assert_eq!(0x1175.is_jungseong(), true);
        assert_eq!(0x1176.is_jungseong(), false);
    }

    #[test]
    fn is_choseong_with_char() {
        assert_eq!('\u{1160}'.is_jungseong(), false);
        assert_eq!('ᅡ'.is_jungseong(), true); // 0x1161
        assert_eq!('ᅢ'.is_jungseong(), true); // 0x1162
        assert_eq!('ᅣ'.is_jungseong(), true); // 0x1163
        assert_eq!('ᅤ'.is_jungseong(), true); // 0x1164
        assert_eq!('ᅥ'.is_jungseong(), true); // 0x1165
        assert_eq!('ᅦ'.is_jungseong(), true); // 0x1166
        assert_eq!('ᅧ'.is_jungseong(), true); // 0x1167
        assert_eq!('ᅨ'.is_jungseong(), true); // 0x1168
        assert_eq!('ᅩ'.is_jungseong(), true); // 0x1169
        assert_eq!('ᅪ'.is_jungseong(), true); // 0x116A
        assert_eq!('ᅫ'.is_jungseong(), true); // 0x116B
        assert_eq!('ᅬ'.is_jungseong(), true); // 0x116C
        assert_eq!('ᅭ'.is_jungseong(), true); // 0x116D
        assert_eq!('ᅮ'.is_jungseong(), true); // 0x116E
        assert_eq!('ᅯ'.is_jungseong(), true); // 0x116F
        assert_eq!('ᅰ'.is_jungseong(), true); // 0x1170
        assert_eq!('ᅱ'.is_jungseong(), true); // 0x1171
        assert_eq!('ᅲ'.is_jungseong(), true); // 0x1172
        assert_eq!('ᅳ'.is_jungseong(), true); // 0x1173
        assert_eq!('ᅴ'.is_jungseong(), true); // 0x1174
        assert_eq!('ᅵ'.is_jungseong(), true); // 0x1175
        assert_eq!('\u{1176}'.is_jungseong(), false);
    }
}
