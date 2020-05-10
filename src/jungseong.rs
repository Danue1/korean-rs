use crate::characters::{JungseongCharacter::*, *};
use crate::constants::*;
use crate::syllable::*;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Jungseong {
    Normal(JungseongCharacter),
    Compat(JungseongCharacter),
}

fn to_code(code: u32) -> u32 {
    if code.is_syllable() {
        let jongseong_code = (code - HANGEUL_OFFSET) % JUNGSEONG_COUNT;
        let value = ((code - HANGEUL_OFFSET - jongseong_code)
            % (JUNGSEONG_COUNT * JONGSEONG_COUNT))
            / JUNGSEONG_COUNT;
        value + JUNGSEONG_START
    } else {
        code
    }
}

pub trait JungseongInformation {
    fn is_jungseong(&self) -> bool;
    fn is_normal_jungseong(&self) -> bool;
    fn is_compat_jungseong(&self) -> bool;
    fn has_jungseong(&self) -> bool;
    fn has_normal_jungseong(&self) -> bool;
    fn has_compat_jungseong(&self) -> bool;
}

impl JungseongInformation for u32 {
    fn is_jungseong(&self) -> bool {
        Jungseong::try_from(*self).is_ok()
    }

    fn is_normal_jungseong(&self) -> bool {
        match Jungseong::try_from(*self) {
            Ok(jungseong) => match jungseong {
                Jungseong::Normal(_) => true,
                Jungseong::Compat(_) => false,
            },
            Err(_) => false,
        }
    }

    fn is_compat_jungseong(&self) -> bool {
        match Jungseong::try_from(*self) {
            Ok(jungseong) => match jungseong {
                Jungseong::Normal(_) => false,
                Jungseong::Compat(_) => true,
            },
            Err(_) => false,
        }
    }

    fn has_jungseong(&self) -> bool {
        to_code(*self).is_jungseong()
    }

    fn has_normal_jungseong(&self) -> bool {
        to_code(*self).is_normal_jungseong()
    }

    fn has_compat_jungseong(&self) -> bool {
        to_code(*self).is_compat_jungseong()
    }
}

impl JungseongInformation for char {
    fn is_jungseong(&self) -> bool {
        (*self as u32).is_jungseong()
    }

    fn is_normal_jungseong(&self) -> bool {
        (*self as u32).is_normal_jungseong()
    }

    fn is_compat_jungseong(&self) -> bool {
        (*self as u32).is_compat_jungseong()
    }

    fn has_jungseong(&self) -> bool {
        (*self as u32).has_jungseong()
    }

    fn has_normal_jungseong(&self) -> bool {
        (*self as u32).has_normal_jungseong()
    }

    fn has_compat_jungseong(&self) -> bool {
        (*self as u32).has_compat_jungseong()
    }
}

impl From<Jungseong> for JungseongCharacter {
    fn from(item: Jungseong) -> JungseongCharacter {
        match item {
            Jungseong::Normal(character) => character.clone(),
            Jungseong::Compat(character) => character.clone(),
        }
    }
}

impl JungseongCharacter {
    pub fn to_normal(&self) -> Jungseong {
        Jungseong::Normal(self.clone())
    }

    pub fn to_compat(&self) -> Jungseong {
        Jungseong::Compat(self.clone())
    }
}

impl TryFrom<u32> for Jungseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match to_code(item) {
            0x1161 => Jungseong::Normal(A),
            0x1162 => Jungseong::Normal(AE),
            0x1163 => Jungseong::Normal(YA),
            0x1164 => Jungseong::Normal(YAE),
            0x1165 => Jungseong::Normal(EO),
            0x1166 => Jungseong::Normal(E),
            0x1167 => Jungseong::Normal(YEO),
            0x1168 => Jungseong::Normal(YE),
            0x1169 => Jungseong::Normal(O),
            0x116A => Jungseong::Normal(WA),
            0x116B => Jungseong::Normal(WAE),
            0x116C => Jungseong::Normal(OE),
            0x116D => Jungseong::Normal(YO),
            0x116E => Jungseong::Normal(U),
            0x116F => Jungseong::Normal(WEO),
            0x1170 => Jungseong::Normal(WE),
            0x1171 => Jungseong::Normal(WI),
            0x1172 => Jungseong::Normal(YU),
            0x1173 => Jungseong::Normal(EU),
            0x1174 => Jungseong::Normal(YI),
            0x1175 => Jungseong::Normal(I),

            0x314F => Jungseong::Compat(A),
            0x3150 => Jungseong::Compat(AE),
            0x3151 => Jungseong::Compat(YA),
            0x3152 => Jungseong::Compat(YAE),
            0x3153 => Jungseong::Compat(EO),
            0x3154 => Jungseong::Compat(E),
            0x3155 => Jungseong::Compat(YEO),
            0x3156 => Jungseong::Compat(YE),
            0x3157 => Jungseong::Compat(O),
            0x3158 => Jungseong::Compat(WA),
            0x3159 => Jungseong::Compat(WAE),
            0x315A => Jungseong::Compat(OE),
            0x315B => Jungseong::Compat(YO),
            0x315C => Jungseong::Compat(U),
            0x315D => Jungseong::Compat(WEO),
            0x315E => Jungseong::Compat(WE),
            0x315F => Jungseong::Compat(WI),
            0x3160 => Jungseong::Compat(YU),
            0x3161 => Jungseong::Compat(EU),
            0x3162 => Jungseong::Compat(YI),
            0x3163 => Jungseong::Compat(I),
            _ => return Err(()),
        };

        Ok(character)
    }
}

impl From<Jungseong> for u32 {
    fn from(item: Jungseong) -> Self {
        match item {
            Jungseong::Normal(character) => match character {
                A => 0x1161,
                AE => 0x1162,
                YA => 0x1163,
                YAE => 0x1164,
                EO => 0x1165,
                E => 0x1166,
                YEO => 0x1167,
                YE => 0x1168,
                O => 0x1169,
                WA => 0x116A,
                WAE => 0x116B,
                OE => 0x116C,
                YO => 0x116D,
                U => 0x116E,
                WEO => 0x116F,
                WE => 0x1170,
                WI => 0x1171,
                YU => 0x1172,
                EU => 0x1173,
                YI => 0x1174,
                I => 0x1175,
            },
            Jungseong::Compat(character) => match character {
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
            },
        }
    }
}

impl TryFrom<char> for Jungseong {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        Jungseong::try_from(item as u32)
    }
}

impl From<Jungseong> for char {
    fn from(item: Jungseong) -> char {
        match item {
            Jungseong::Normal(character) => match character {
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
            },
            Jungseong::Compat(character) => match character {
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
            },
        }
    }
}

impl CharacterInformation for Jungseong {
    fn is_jaeum(&self) -> bool {
        match self {
            Jungseong::Normal(character) => character.is_jaeum(),
            Jungseong::Compat(character) => character.is_jaeum(),
        }
    }

    fn is_moeum(&self) -> bool {
        match self {
            Jungseong::Normal(character) => character.is_moeum(),
            Jungseong::Compat(character) => character.is_moeum(),
        }
    }

    fn to_composable(&self) -> u32 {
        match self {
            Jungseong::Normal(character) => character.to_composable(),
            Jungseong::Compat(character) => character.to_composable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NON_NORMAL_JUNGSEONG_START_U32: u32 = 0x1160;
    const NON_NORMAL_JUNGSEONG_END_U32: u32 = 0x1176;
    const NORMAL_JUNGSEONG_U32_LIST: [u32; 21] = [
        0x1161, 0x1162, 0x1163, 0x1164, 0x1165, 0x1166, 0x1167, 0x1168, 0x1169, 0x116A, 0x116B,
        0x116C, 0x116D, 0x116E, 0x116F, 0x1170, 0x1171, 0x1172, 0x1173, 0x1174, 0x1175,
    ];

    const NON_COMPAT_JUNGSEONG_START_U32: u32 = 0x1160;
    const NON_COMPAT_JUNGSEONG_END_U32: u32 = 0x1176;
    const COMPAT_JUNGSEONG_U32_LIST: [u32; 21] = [
        0x314F, 0x3150, 0x3151, 0x3152, 0x3153, 0x3154, 0x3155, 0x3156, 0x3157, 0x3158, 0x3159,
        0x315A, 0x315B, 0x315C, 0x315D, 0x315E, 0x315F, 0x3160, 0x3161, 0x3162, 0x3163,
    ];

    const NON_NORMAL_JUNGSEONG_START_CHAR: char = '\u{1160}';
    const NON_NORMAL_JUNGSEONG_END_CHAR: char = '\u{1176}';
    const NORMAL_JUNGSEONG_CHAR_LIST: [char; 21] = [
        'ᅡ', 'ᅢ', 'ᅣ', 'ᅤ', 'ᅥ', 'ᅦ', 'ᅧ', 'ᅨ', 'ᅩ', 'ᅪ', 'ᅫ', 'ᅬ', 'ᅭ', 'ᅮ', 'ᅯ', 'ᅰ', 'ᅱ', 'ᅲ', 'ᅳ', 'ᅴ', 'ᅵ',
    ];

    const NON_COMPAT_JUNGSEONG_START_CHAR: char = '\u{1160}';
    const NON_COMPAT_JUNGSEONG_END_CHAR: char = '\u{1176}';
    const COMPAT_JUNGSEONG_CHAR_LIST: [char; 21] = [
        'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ',
        'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
    ];

    #[test]
    fn is_jungseong_with_u32() {
        assert_eq!(NON_NORMAL_JUNGSEONG_START_U32.is_jungseong(), false);
        for jungseong in NORMAL_JUNGSEONG_U32_LIST.iter() {
            assert_eq!(jungseong.is_jungseong(), true);
        }
        assert_eq!(NON_NORMAL_JUNGSEONG_END_U32.is_jungseong(), false);

        assert_eq!(NON_COMPAT_JUNGSEONG_START_U32.is_jungseong(), false);
        for jungseong in COMPAT_JUNGSEONG_U32_LIST.iter() {
            assert_eq!(jungseong.is_jungseong(), true);
        }
        assert_eq!(NON_COMPAT_JUNGSEONG_END_U32.is_jungseong(), false);
    }

    #[test]
    fn is_jungseong_with_char() {
        assert_eq!(NON_NORMAL_JUNGSEONG_START_CHAR.is_jungseong(), false);
        for jungseong in NORMAL_JUNGSEONG_CHAR_LIST.iter() {
            assert_eq!(jungseong.is_jungseong(), true);
        }
        assert_eq!(NON_NORMAL_JUNGSEONG_END_CHAR.is_jungseong(), false);

        assert_eq!(NON_COMPAT_JUNGSEONG_START_CHAR.is_jungseong(), false);
        for jungseong in COMPAT_JUNGSEONG_CHAR_LIST.iter() {
            assert_eq!(jungseong.is_jungseong(), true);
        }
        assert_eq!(NON_COMPAT_JUNGSEONG_END_CHAR.is_jungseong(), false);
    }

    #[test]
    fn is_normal_jungseong_with_u32() {
        assert_eq!(NON_NORMAL_JUNGSEONG_START_U32.is_normal_jungseong(), false);
        for jungseong in NORMAL_JUNGSEONG_U32_LIST.iter() {
            assert_eq!(jungseong.is_normal_jungseong(), true);
        }
        assert_eq!(NON_NORMAL_JUNGSEONG_END_U32.is_normal_jungseong(), false);

        assert_eq!(NON_COMPAT_JUNGSEONG_START_U32.is_normal_jungseong(), false);
        for jungseong in COMPAT_JUNGSEONG_U32_LIST.iter() {
            assert_eq!(jungseong.is_normal_jungseong(), false);
        }
        assert_eq!(NON_COMPAT_JUNGSEONG_END_U32.is_normal_jungseong(), false);
    }

    #[test]
    fn is_normal_jungseong_with_char() {
        assert_eq!(NON_NORMAL_JUNGSEONG_START_CHAR.is_normal_jungseong(), false);
        for jungseong in NORMAL_JUNGSEONG_CHAR_LIST.iter() {
            assert_eq!(jungseong.is_normal_jungseong(), true);
        }
        assert_eq!(NON_NORMAL_JUNGSEONG_END_CHAR.is_normal_jungseong(), false);

        assert_eq!(NON_COMPAT_JUNGSEONG_START_CHAR.is_normal_jungseong(), false);
        for jungseong in COMPAT_JUNGSEONG_CHAR_LIST.iter() {
            assert_eq!(jungseong.is_normal_jungseong(), false);
        }
        assert_eq!(NON_COMPAT_JUNGSEONG_END_CHAR.is_normal_jungseong(), false);
    }

    #[test]
    fn is_compat_jungseong_with_u32() {
        assert_eq!(NON_NORMAL_JUNGSEONG_START_U32.is_compat_jungseong(), false);
        for jungseong in NORMAL_JUNGSEONG_U32_LIST.iter() {
            assert_eq!(jungseong.is_compat_jungseong(), false);
        }
        assert_eq!(NON_NORMAL_JUNGSEONG_END_U32.is_compat_jungseong(), false);

        assert_eq!(NON_COMPAT_JUNGSEONG_START_U32.is_compat_jungseong(), false);
        for jungseong in COMPAT_JUNGSEONG_U32_LIST.iter() {
            assert_eq!(jungseong.is_compat_jungseong(), true);
        }
        assert_eq!(NON_COMPAT_JUNGSEONG_END_U32.is_compat_jungseong(), false);
    }

    #[test]
    fn is_compat_jungseong_with_char() {
        assert_eq!(NON_NORMAL_JUNGSEONG_START_CHAR.is_compat_jungseong(), false);
        for jungseong in NORMAL_JUNGSEONG_CHAR_LIST.iter() {
            assert_eq!(jungseong.is_compat_jungseong(), false);
        }
        assert_eq!(NON_NORMAL_JUNGSEONG_END_CHAR.is_compat_jungseong(), false);

        assert_eq!(NON_COMPAT_JUNGSEONG_START_CHAR.is_compat_jungseong(), false);
        for jungseong in COMPAT_JUNGSEONG_CHAR_LIST.iter() {
            assert_eq!(jungseong.is_compat_jungseong(), true);
        }
        assert_eq!(NON_COMPAT_JUNGSEONG_END_CHAR.is_compat_jungseong(), false);
    }
}
