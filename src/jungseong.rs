use crate::characters::{JungseongCharacter::*, *};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Jungseong {
    Normal(JungseongCharacter),
    Compat(JungseongCharacter),
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
        JungseongCharacter::to_code(*self).is_jungseong()
    }

    fn has_normal_jungseong(&self) -> bool {
        JungseongCharacter::to_code(*self).is_normal_jungseong()
    }

    fn has_compat_jungseong(&self) -> bool {
        JungseongCharacter::to_code(*self).is_compat_jungseong()
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

impl From<&Jungseong> for JungseongCharacter {
    fn from(item: &Jungseong) -> JungseongCharacter {
        match item {
            Jungseong::Normal(character) => character.clone(),
            Jungseong::Compat(character) => character.clone(),
        }
    }
}

impl From<&JungseongCharacter> for Jungseong {
    fn from(item: &JungseongCharacter) -> Jungseong {
        Jungseong::Normal(item.clone())
    }
}

impl TryFrom<u32> for Jungseong {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        let character = match JungseongCharacter::to_code(item) {
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

impl From<&Jungseong> for u32 {
    fn from(item: &Jungseong) -> Self {
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

impl From<&Jungseong> for char {
    fn from(item: &Jungseong) -> char {
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

    #[test]
    fn is_jungseong_with_u32() {
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

        assert_eq!(0x1160.is_jungseong(), false);
        assert_eq!(0x314F.is_jungseong(), true);
        assert_eq!(0x3150.is_jungseong(), true);
        assert_eq!(0x3151.is_jungseong(), true);
        assert_eq!(0x3152.is_jungseong(), true);
        assert_eq!(0x3153.is_jungseong(), true);
        assert_eq!(0x3154.is_jungseong(), true);
        assert_eq!(0x3155.is_jungseong(), true);
        assert_eq!(0x3156.is_jungseong(), true);
        assert_eq!(0x3157.is_jungseong(), true);
        assert_eq!(0x3158.is_jungseong(), true);
        assert_eq!(0x3159.is_jungseong(), true);
        assert_eq!(0x315A.is_jungseong(), true);
        assert_eq!(0x315B.is_jungseong(), true);
        assert_eq!(0x315C.is_jungseong(), true);
        assert_eq!(0x315D.is_jungseong(), true);
        assert_eq!(0x315E.is_jungseong(), true);
        assert_eq!(0x315F.is_jungseong(), true);
        assert_eq!(0x3160.is_jungseong(), true);
        assert_eq!(0x3161.is_jungseong(), true);
        assert_eq!(0x3162.is_jungseong(), true);
        assert_eq!(0x3163.is_jungseong(), true);
        assert_eq!(0x1176.is_jungseong(), false);
    }

    #[test]
    fn is_jungseong_with_char() {
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

        assert_eq!('\u{1160}'.is_jungseong(), false);
        assert_eq!('ㅏ'.is_jungseong(), true); // 0x314F
        assert_eq!('ㅐ'.is_jungseong(), true); // 0x3150
        assert_eq!('ㅑ'.is_jungseong(), true); // 0x3151
        assert_eq!('ㅒ'.is_jungseong(), true); // 0x3152
        assert_eq!('ㅓ'.is_jungseong(), true); // 0x3153
        assert_eq!('ㅔ'.is_jungseong(), true); // 0x3154
        assert_eq!('ㅕ'.is_jungseong(), true); // 0x3155
        assert_eq!('ㅖ'.is_jungseong(), true); // 0x3156
        assert_eq!('ㅗ'.is_jungseong(), true); // 0x3157
        assert_eq!('ㅘ'.is_jungseong(), true); // 0x3158
        assert_eq!('ㅙ'.is_jungseong(), true); // 0x3159
        assert_eq!('ㅚ'.is_jungseong(), true); // 0x315A
        assert_eq!('ㅛ'.is_jungseong(), true); // 0x315B
        assert_eq!('ㅜ'.is_jungseong(), true); // 0x315C
        assert_eq!('ㅝ'.is_jungseong(), true); // 0x315D
        assert_eq!('ㅞ'.is_jungseong(), true); // 0x315E
        assert_eq!('ㅟ'.is_jungseong(), true); // 0x315F
        assert_eq!('ㅠ'.is_jungseong(), true); // 0x3160
        assert_eq!('ㅡ'.is_jungseong(), true); // 0x3161
        assert_eq!('ㅢ'.is_jungseong(), true); // 0x3162
        assert_eq!('ㅣ'.is_jungseong(), true); // 0x3163
        assert_eq!('\u{1176}'.is_jungseong(), false);
    }

    #[test]
    fn is_normal_jungseong_with_u32() {
        assert_eq!(0x1160.is_normal_jungseong(), false);
        assert_eq!(0x1161.is_normal_jungseong(), true);
        assert_eq!(0x1162.is_normal_jungseong(), true);
        assert_eq!(0x1163.is_normal_jungseong(), true);
        assert_eq!(0x1164.is_normal_jungseong(), true);
        assert_eq!(0x1165.is_normal_jungseong(), true);
        assert_eq!(0x1166.is_normal_jungseong(), true);
        assert_eq!(0x1167.is_normal_jungseong(), true);
        assert_eq!(0x1168.is_normal_jungseong(), true);
        assert_eq!(0x1169.is_normal_jungseong(), true);
        assert_eq!(0x116A.is_normal_jungseong(), true);
        assert_eq!(0x116B.is_normal_jungseong(), true);
        assert_eq!(0x116C.is_normal_jungseong(), true);
        assert_eq!(0x116D.is_normal_jungseong(), true);
        assert_eq!(0x116E.is_normal_jungseong(), true);
        assert_eq!(0x116F.is_normal_jungseong(), true);
        assert_eq!(0x1170.is_normal_jungseong(), true);
        assert_eq!(0x1171.is_normal_jungseong(), true);
        assert_eq!(0x1172.is_normal_jungseong(), true);
        assert_eq!(0x1173.is_normal_jungseong(), true);
        assert_eq!(0x1174.is_normal_jungseong(), true);
        assert_eq!(0x1175.is_normal_jungseong(), true);
        assert_eq!(0x1176.is_normal_jungseong(), false);

        assert_eq!(0x1160.is_normal_jungseong(), false);
        assert_eq!(0x314F.is_normal_jungseong(), false);
        assert_eq!(0x3150.is_normal_jungseong(), false);
        assert_eq!(0x3151.is_normal_jungseong(), false);
        assert_eq!(0x3152.is_normal_jungseong(), false);
        assert_eq!(0x3153.is_normal_jungseong(), false);
        assert_eq!(0x3154.is_normal_jungseong(), false);
        assert_eq!(0x3155.is_normal_jungseong(), false);
        assert_eq!(0x3156.is_normal_jungseong(), false);
        assert_eq!(0x3157.is_normal_jungseong(), false);
        assert_eq!(0x3158.is_normal_jungseong(), false);
        assert_eq!(0x3159.is_normal_jungseong(), false);
        assert_eq!(0x315A.is_normal_jungseong(), false);
        assert_eq!(0x315B.is_normal_jungseong(), false);
        assert_eq!(0x315C.is_normal_jungseong(), false);
        assert_eq!(0x315D.is_normal_jungseong(), false);
        assert_eq!(0x315E.is_normal_jungseong(), false);
        assert_eq!(0x315F.is_normal_jungseong(), false);
        assert_eq!(0x3160.is_normal_jungseong(), false);
        assert_eq!(0x3161.is_normal_jungseong(), false);
        assert_eq!(0x3162.is_normal_jungseong(), false);
        assert_eq!(0x3163.is_normal_jungseong(), false);
        assert_eq!(0x1176.is_normal_jungseong(), false);
    }

    #[test]
    fn is_normal_jungseong_with_char() {
        assert_eq!('\u{1160}'.is_normal_jungseong(), false);
        assert_eq!('ᅡ'.is_normal_jungseong(), true); // 0x1161
        assert_eq!('ᅢ'.is_normal_jungseong(), true); // 0x1162
        assert_eq!('ᅣ'.is_normal_jungseong(), true); // 0x1163
        assert_eq!('ᅤ'.is_normal_jungseong(), true); // 0x1164
        assert_eq!('ᅥ'.is_normal_jungseong(), true); // 0x1165
        assert_eq!('ᅦ'.is_normal_jungseong(), true); // 0x1166
        assert_eq!('ᅧ'.is_normal_jungseong(), true); // 0x1167
        assert_eq!('ᅨ'.is_normal_jungseong(), true); // 0x1168
        assert_eq!('ᅩ'.is_normal_jungseong(), true); // 0x1169
        assert_eq!('ᅪ'.is_normal_jungseong(), true); // 0x116A
        assert_eq!('ᅫ'.is_normal_jungseong(), true); // 0x116B
        assert_eq!('ᅬ'.is_normal_jungseong(), true); // 0x116C
        assert_eq!('ᅭ'.is_normal_jungseong(), true); // 0x116D
        assert_eq!('ᅮ'.is_normal_jungseong(), true); // 0x116E
        assert_eq!('ᅯ'.is_normal_jungseong(), true); // 0x116F
        assert_eq!('ᅰ'.is_normal_jungseong(), true); // 0x1170
        assert_eq!('ᅱ'.is_normal_jungseong(), true); // 0x1171
        assert_eq!('ᅲ'.is_normal_jungseong(), true); // 0x1172
        assert_eq!('ᅳ'.is_normal_jungseong(), true); // 0x1173
        assert_eq!('ᅴ'.is_normal_jungseong(), true); // 0x1174
        assert_eq!('ᅵ'.is_normal_jungseong(), true); // 0x1175
        assert_eq!('\u{1176}'.is_normal_jungseong(), false);

        assert_eq!('\u{1160}'.is_normal_jungseong(), false);
        assert_eq!('ㅏ'.is_normal_jungseong(), false); // 0x314F
        assert_eq!('ㅐ'.is_normal_jungseong(), false); // 0x3150
        assert_eq!('ㅑ'.is_normal_jungseong(), false); // 0x3151
        assert_eq!('ㅒ'.is_normal_jungseong(), false); // 0x3152
        assert_eq!('ㅓ'.is_normal_jungseong(), false); // 0x3153
        assert_eq!('ㅔ'.is_normal_jungseong(), false); // 0x3154
        assert_eq!('ㅕ'.is_normal_jungseong(), false); // 0x3155
        assert_eq!('ㅖ'.is_normal_jungseong(), false); // 0x3156
        assert_eq!('ㅗ'.is_normal_jungseong(), false); // 0x3157
        assert_eq!('ㅘ'.is_normal_jungseong(), false); // 0x3158
        assert_eq!('ㅙ'.is_normal_jungseong(), false); // 0x3159
        assert_eq!('ㅚ'.is_normal_jungseong(), false); // 0x315A
        assert_eq!('ㅛ'.is_normal_jungseong(), false); // 0x315B
        assert_eq!('ㅜ'.is_normal_jungseong(), false); // 0x315C
        assert_eq!('ㅝ'.is_normal_jungseong(), false); // 0x315D
        assert_eq!('ㅞ'.is_normal_jungseong(), false); // 0x315E
        assert_eq!('ㅟ'.is_normal_jungseong(), false); // 0x315F
        assert_eq!('ㅠ'.is_normal_jungseong(), false); // 0x3160
        assert_eq!('ㅡ'.is_normal_jungseong(), false); // 0x3161
        assert_eq!('ㅢ'.is_normal_jungseong(), false); // 0x3162
        assert_eq!('ㅣ'.is_normal_jungseong(), false); // 0x3163
        assert_eq!('\u{1176}'.is_normal_jungseong(), false);
    }

    #[test]
    fn is_compat_jungseong_with_u32() {
        assert_eq!(0x1160.is_compat_jungseong(), false);
        assert_eq!(0x1161.is_compat_jungseong(), false);
        assert_eq!(0x1162.is_compat_jungseong(), false);
        assert_eq!(0x1163.is_compat_jungseong(), false);
        assert_eq!(0x1164.is_compat_jungseong(), false);
        assert_eq!(0x1165.is_compat_jungseong(), false);
        assert_eq!(0x1166.is_compat_jungseong(), false);
        assert_eq!(0x1167.is_compat_jungseong(), false);
        assert_eq!(0x1168.is_compat_jungseong(), false);
        assert_eq!(0x1169.is_compat_jungseong(), false);
        assert_eq!(0x116A.is_compat_jungseong(), false);
        assert_eq!(0x116B.is_compat_jungseong(), false);
        assert_eq!(0x116C.is_compat_jungseong(), false);
        assert_eq!(0x116D.is_compat_jungseong(), false);
        assert_eq!(0x116E.is_compat_jungseong(), false);
        assert_eq!(0x116F.is_compat_jungseong(), false);
        assert_eq!(0x1170.is_compat_jungseong(), false);
        assert_eq!(0x1171.is_compat_jungseong(), false);
        assert_eq!(0x1172.is_compat_jungseong(), false);
        assert_eq!(0x1173.is_compat_jungseong(), false);
        assert_eq!(0x1174.is_compat_jungseong(), false);
        assert_eq!(0x1175.is_compat_jungseong(), false);
        assert_eq!(0x1176.is_compat_jungseong(), false);

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
    fn is_compat_jungseong_with_char() {
        assert_eq!('\u{1160}'.is_compat_jungseong(), false);
        assert_eq!('ᅡ'.is_compat_jungseong(), false); // 0x1161
        assert_eq!('ᅢ'.is_compat_jungseong(), false); // 0x1162
        assert_eq!('ᅣ'.is_compat_jungseong(), false); // 0x1163
        assert_eq!('ᅤ'.is_compat_jungseong(), false); // 0x1164
        assert_eq!('ᅥ'.is_compat_jungseong(), false); // 0x1165
        assert_eq!('ᅦ'.is_compat_jungseong(), false); // 0x1166
        assert_eq!('ᅧ'.is_compat_jungseong(), false); // 0x1167
        assert_eq!('ᅨ'.is_compat_jungseong(), false); // 0x1168
        assert_eq!('ᅩ'.is_compat_jungseong(), false); // 0x1169
        assert_eq!('ᅪ'.is_compat_jungseong(), false); // 0x116A
        assert_eq!('ᅫ'.is_compat_jungseong(), false); // 0x116B
        assert_eq!('ᅬ'.is_compat_jungseong(), false); // 0x116C
        assert_eq!('ᅭ'.is_compat_jungseong(), false); // 0x116D
        assert_eq!('ᅮ'.is_compat_jungseong(), false); // 0x116E
        assert_eq!('ᅯ'.is_compat_jungseong(), false); // 0x116F
        assert_eq!('ᅰ'.is_compat_jungseong(), false); // 0x1170
        assert_eq!('ᅱ'.is_compat_jungseong(), false); // 0x1171
        assert_eq!('ᅲ'.is_compat_jungseong(), false); // 0x1172
        assert_eq!('ᅳ'.is_compat_jungseong(), false); // 0x1173
        assert_eq!('ᅴ'.is_compat_jungseong(), false); // 0x1174
        assert_eq!('ᅵ'.is_compat_jungseong(), false); // 0x1175
        assert_eq!('\u{1176}'.is_compat_jungseong(), false);

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
