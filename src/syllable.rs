use crate::characters::*;
use crate::choseong::*;
use crate::constants::*;
use crate::jongseong::*;
use crate::jungseong::*;
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
        std::char::from_u32(item.0).unwrap_or_else(|| unreachable!())
    }
}

impl TryFrom<(&u32, &u32, Option<u32>)> for Syllable {
    type Error = ();

    fn try_from(item: (&u32, &u32, Option<u32>)) -> Result<Self, Self::Error> {
        let choseong = Choseong::try_from(*item.0)?.to_composable();
        let jungseong = Jungseong::try_from(*item.1)?.to_composable();
        let jongseong = match item.2 {
            Some(code) => Jongseong::try_from(code)?.to_composable(),
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
        let jongseong = item
            .2
            .map(|character| character.to_composable())
            .unwrap_or(0);

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
        let choseong: char = match &Choseong::try_from(item.0) {
            Ok(character) => character.into(),
            Err(_) => unreachable!(),
        };
        let jungseong: char = match &Jungseong::try_from(item.0) {
            Ok(character) => character.into(),
            Err(_) => unreachable!(),
        };
        let jongseong: Option<char> = match &Jongseong::try_from(item.0) {
            Ok(character) => Some(character.into()),
            Err(_) => None,
        };

        (choseong, jungseong, jongseong)
    }
}

impl From<&Syllable> for Option<(Choseong, Jungseong, Option<Jongseong>)> {
    fn from(item: &Syllable) -> Option<(Choseong, Jungseong, Option<Jongseong>)> {
        let choseong: Choseong = match Choseong::try_from(item.0) {
            Ok(character) => character.into(),
            Err(_) => unreachable!(),
        };
        let jungseong: Jungseong = match Jungseong::try_from(item.0) {
            Ok(character) => character.into(),
            Err(_) => unreachable!(),
        };
        let jongseong: Option<Jongseong> = match Jongseong::try_from(item.0) {
            Ok(character) => Some(character.into()),
            Err(_) => None,
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
