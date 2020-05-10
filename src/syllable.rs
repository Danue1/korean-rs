use crate::characters::*;
use crate::compat::*;
use crate::constants::*;
use crate::normal::*;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Syllable(u32);

impl TryFrom<u32> for Syllable {
    type Error = ();

    fn try_from(item: u32) -> Result<Self, Self::Error> {
        if item.is_syllable() {
            Ok(Syllable(item))
        } else {
            Err(())
        }
    }
}

impl TryFrom<&char> for Syllable {
    type Error = ();

    fn try_from(item: &char) -> Result<Self, Self::Error> {
        if item.is_syllable() {
            Ok(Syllable(*item as u32))
        } else {
            Err(())
        }
    }
}

impl From<&Syllable> for u32 {
    fn from(item: &Syllable) -> u32 {
        item.0
    }
}

impl From<&Syllable> for char {
    fn from(item: &Syllable) -> char {
        match std::char::from_u32(item.0) {
            Some(character) => character,
            None => unreachable!(),
        }
    }
}

impl TryFrom<(&u32, &u32, Option<u32>)> for Syllable {
    type Error = ();

    fn try_from(item: (&u32, &u32, Option<u32>)) -> Result<Self, Self::Error> {
        let choseong = if let Ok(character) = Choseong::try_from(*item.0) {
            character.to_composable()
        } else if let Ok(character) = CompatChoseong::try_from(*item.0) {
            character.to_composable()
        } else {
            return Err(());
        };
        let jungseong = if let Ok(character) = Jungseong::try_from(*item.1) {
            character.to_composable()
        } else if let Ok(character) = CompatJungseong::try_from(*item.1) {
            character.to_composable()
        } else {
            return Err(());
        };
        let jongseong = match item.2 {
            Some(code) => {
                if let Ok(character) = Jongseong::try_from(code) {
                    character.to_composable()
                } else if let Ok(character) = CompatJongseong::try_from(code) {
                    character.to_composable()
                } else {
                    return Err(());
                }
            }
            None => 0,
        };

        Syllable::try_from(choseong + jungseong + jongseong + HANGEUL_OFFSET)
    }
}

impl TryFrom<(&char, &char, Option<char>)> for Syllable {
    type Error = ();

    fn try_from(item: (&char, &char, Option<char>)) -> Result<Self, Self::Error> {
        Syllable::try_from((
            &(*item.0 as u32),
            &(*item.1 as u32),
            item.2.map(|code| code as u32),
        ))
    }
}

impl TryFrom<(&Choseong, &Jungseong, Option<&Jongseong>)> for Syllable {
    type Error = ();

    fn try_from(item: (&Choseong, &Jungseong, Option<&Jongseong>)) -> Result<Self, Self::Error> {
        let choseong = item.0.to_composable();
        let jungseong = item.1.to_composable();
        let jongseong = match item.2 {
            Some(character) => character.to_composable(),
            None => 0,
        };

        Syllable::try_from(choseong + jungseong + jongseong + HANGEUL_OFFSET)
    }
}

impl TryFrom<(&CompatChoseong, &CompatJungseong, Option<&CompatJongseong>)> for Syllable {
    type Error = ();

    fn try_from(
        item: (&CompatChoseong, &CompatJungseong, Option<&CompatJongseong>),
    ) -> Result<Self, Self::Error> {
        let choseong = item.0.to_composable();
        let jungseong = item.1.to_composable();
        let jongseong = match item.2 {
            Some(character) => character.to_composable(),
            None => 0,
        };

        Syllable::try_from(choseong + jungseong + jongseong + HANGEUL_OFFSET)
    }
}

impl From<&Syllable> for (u32, u32, Option<u32>) {
    fn from(item: &Syllable) -> (u32, u32, Option<u32>) {
        let (choseong, jungseong, jongseong): (char, char, Option<char>) = item.into();

        (
            choseong as u32,
            jungseong as u32,
            jongseong.map(|code| code as u32),
        )
    }
}

impl From<&Syllable> for (char, char, Option<char>) {
    fn from(item: &Syllable) -> (char, char, Option<char>) {
        let choseong: char = if let Ok(character) = &Choseong::try_from(item.0) {
            character.into()
        } else if let Ok(character) = &CompatChoseong::try_from(item.0) {
            character.into()
        } else {
            unreachable!();
        };
        let jungseong: char = if let Ok(character) = &Jungseong::try_from(item.0) {
            character.into()
        } else if let Ok(character) = &CompatJungseong::try_from(item.0) {
            character.into()
        } else {
            unreachable!();
        };
        let jongseong: Option<char> = if let Ok(character) = &Jongseong::try_from(item.0) {
            Some(character.into())
        } else if let Ok(character) = &CompatJongseong::try_from(item.0) {
            Some(character.into())
        } else {
            None
        };

        (choseong, jungseong, jongseong)
    }
}

impl From<&Syllable> for Option<(Choseong, Jungseong, Option<Jongseong>)> {
    fn from(item: &Syllable) -> Option<(Choseong, Jungseong, Option<Jongseong>)> {
        let choseong: Choseong = if let Ok(character) = Choseong::try_from(item.0) {
            character.into()
        } else {
            return None;
        };
        let jungseong: Jungseong = if let Ok(character) = Jungseong::try_from(item.0) {
            character.into()
        } else {
            return None;
        };
        let jongseong: Option<Jongseong> = if let Ok(character) = Jongseong::try_from(item.0) {
            Some(character.into())
        } else {
            None
        };

        Some((choseong, jungseong, jongseong))
    }
}

impl From<&Syllable> for Option<(CompatChoseong, CompatJungseong, Option<CompatJongseong>)> {
    fn from(item: &Syllable) -> Option<(CompatChoseong, CompatJungseong, Option<CompatJongseong>)> {
        let choseong: CompatChoseong = if let Ok(character) = CompatChoseong::try_from(item.0) {
            character.into()
        } else {
            return None;
        };
        let jungseong: CompatJungseong = if let Ok(character) = CompatJungseong::try_from(item.0) {
            character.into()
        } else {
            return None;
        };
        let jongseong: Option<CompatJongseong> =
            if let Ok(character) = CompatJongseong::try_from(item.0) {
                Some(character.into())
            } else {
                None
            };

        Some((choseong, jungseong, jongseong))
    }
}

pub trait SyllableInformation {
    fn is_hangeul(&self) -> bool;
    fn is_syllable(&self) -> bool;
    fn is_jamo(&self) -> bool;
    fn is_compat_jamo(&self) -> bool;
}

impl SyllableInformation for u32 {
    fn is_hangeul(&self) -> bool {
        self.is_syllable() || self.is_jamo() || self.is_compat_jamo()
    }

    fn is_syllable(&self) -> bool {
        matches!(self, SYLLABLE_START..=SYLLABLE_END)
    }

    fn is_jamo(&self) -> bool {
        matches!(self, JAMO_START..=JAMO_END)
    }

    fn is_compat_jamo(&self) -> bool {
        matches!(self, COMPAT_JAMO_START..=COMPAT_JAMO_END)
    }
}

impl SyllableInformation for char {
    fn is_hangeul(&self) -> bool {
        (*self as u32).is_hangeul()
    }

    fn is_syllable(&self) -> bool {
        (*self as u32).is_syllable()
    }

    fn is_jamo(&self) -> bool {
        (*self as u32).is_jamo()
    }

    fn is_compat_jamo(&self) -> bool {
        (*self as u32).is_compat_jamo()
    }
}

impl ChoseongInformation for Syllable {
    fn is_choseong(&self) -> bool {
        self.0.is_choseong()
    }

    fn has_choseong(&self) -> bool {
        self.0.has_choseong()
    }
}

impl JungseongInformation for Syllable {
    fn is_jungseong(&self) -> bool {
        self.0.is_jungseong()
    }

    fn has_jungseong(&self) -> bool {
        self.0.has_jungseong()
    }
}

impl JongseongInformation for Syllable {
    fn is_jongseong(&self) -> bool {
        self.0.is_jongseong()
    }

    fn has_jongseong(&self) -> bool {
        self.0.has_jongseong()
    }
}

impl CompatChoseongInformation for Syllable {
    fn is_compat_choseong(&self) -> bool {
        self.0.is_compat_choseong()
    }

    fn has_compat_choseong(&self) -> bool {
        self.0.has_compat_choseong()
    }
}

impl CompatJungseongInformation for Syllable {
    fn is_compat_jungseong(&self) -> bool {
        self.0.is_compat_jungseong()
    }

    fn has_compat_jungseong(&self) -> bool {
        self.0.has_compat_jungseong()
    }
}

impl CompatJongseongInformation for Syllable {
    fn is_compat_jongseong(&self) -> bool {
        self.0.is_compat_jongseong()
    }

    fn has_compat_jongseong(&self) -> bool {
        self.0.has_compat_jongseong()
    }
}
