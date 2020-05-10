use self::ChoseongCharacter::*;
use crate::constants::*;
use crate::syllable::*;
use crate::CharacterInformation;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ChoseongCharacter {
    /// ㄱ
    Giyeok,
    /// ㄲ
    SsangGiyeok,
    /// ㄴ
    Nieun,
    /// ㄷ
    Digeut,
    /// ㄸ
    SsangDigeut,
    /// ㄹ
    Rieul,
    /// ㅁ
    Mieum,
    /// ㅂ
    Bieup,
    /// ㅃ
    SsangBieup,
    /// ㅅ
    Siot,
    /// ㅆ
    SsangSiot,
    /// ㅇ
    Ieung,
    /// ㅈ
    Jieut,
    /// ㅉ
    SsangJieut,
    /// ㅊ
    Chieut,
    /// ㅋ
    Kiyeok,
    /// ㅌ
    Tieut,
    /// ㅍ
    Pieup,
    /// ㅎ
    Hieuh,
}

impl ChoseongCharacter {
    pub(crate) fn to_code(code: u32) -> u32 {
        if code.is_syllable() {
            let value = (code - HANGEUL_OFFSET) / (JUNGSEONG_COUNT * JONGSEONG_COUNT);
            value + CHOSEONG_START
        } else {
            code
        }
    }

    pub fn to_index(&self) -> u32 {
        match self {
            Giyeok => 0,
            SsangGiyeok => 1,
            Nieun => 2,
            Digeut => 3,
            SsangDigeut => 4,
            Rieul => 5,
            Mieum => 6,
            Bieup => 7,
            SsangBieup => 8,
            Siot => 9,
            SsangSiot => 10,
            Ieung => 11,
            Jieut => 12,
            SsangJieut => 13,
            Chieut => 14,
            Kiyeok => 15,
            Tieut => 16,
            Pieup => 17,
            Hieuh => 18,
        }
    }
}

impl CharacterInformation for ChoseongCharacter {
    fn is_jaeum(&self) -> bool {
        true
    }

    fn is_moeum(&self) -> bool {
        false
    }

    fn to_composable(&self) -> u32 {
        self.to_index() * JUNGSEONG_COUNT * JONGSEONG_COUNT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_jaeum() {
        assert_eq!(Giyeok.is_jaeum(), true);
        assert_eq!(SsangGiyeok.is_jaeum(), true);
        assert_eq!(Nieun.is_jaeum(), true);
        assert_eq!(Digeut.is_jaeum(), true);
        assert_eq!(SsangDigeut.is_jaeum(), true);
        assert_eq!(Rieul.is_jaeum(), true);
        assert_eq!(Mieum.is_jaeum(), true);
        assert_eq!(Bieup.is_jaeum(), true);
        assert_eq!(SsangBieup.is_jaeum(), true);
        assert_eq!(Siot.is_jaeum(), true);
        assert_eq!(SsangSiot.is_jaeum(), true);
        assert_eq!(Ieung.is_jaeum(), true);
        assert_eq!(Jieut.is_jaeum(), true);
        assert_eq!(SsangJieut.is_jaeum(), true);
        assert_eq!(Chieut.is_jaeum(), true);
        assert_eq!(Kiyeok.is_jaeum(), true);
        assert_eq!(Tieut.is_jaeum(), true);
        assert_eq!(Pieup.is_jaeum(), true);
        assert_eq!(Hieuh.is_jaeum(), true);
    }

    #[test]
    fn is_moeum() {
        assert_eq!(Giyeok.is_moeum(), false);
        assert_eq!(SsangGiyeok.is_moeum(), false);
        assert_eq!(Nieun.is_moeum(), false);
        assert_eq!(Digeut.is_moeum(), false);
        assert_eq!(SsangDigeut.is_moeum(), false);
        assert_eq!(Rieul.is_moeum(), false);
        assert_eq!(Mieum.is_moeum(), false);
        assert_eq!(Bieup.is_moeum(), false);
        assert_eq!(SsangBieup.is_moeum(), false);
        assert_eq!(Siot.is_moeum(), false);
        assert_eq!(SsangSiot.is_moeum(), false);
        assert_eq!(Ieung.is_moeum(), false);
        assert_eq!(Jieut.is_moeum(), false);
        assert_eq!(SsangJieut.is_moeum(), false);
        assert_eq!(Chieut.is_moeum(), false);
        assert_eq!(Kiyeok.is_moeum(), false);
        assert_eq!(Tieut.is_moeum(), false);
        assert_eq!(Pieup.is_moeum(), false);
        assert_eq!(Hieuh.is_moeum(), false);
    }

    #[test]
    fn to_index() {
        assert_eq!(Giyeok.to_index(), 0);
        assert_eq!(SsangGiyeok.to_index(), 1);
        assert_eq!(Nieun.to_index(), 2);
        assert_eq!(Digeut.to_index(), 3);
        assert_eq!(SsangDigeut.to_index(), 4);
        assert_eq!(Rieul.to_index(), 5);
        assert_eq!(Mieum.to_index(), 6);
        assert_eq!(Bieup.to_index(), 7);
        assert_eq!(SsangBieup.to_index(), 8);
        assert_eq!(Siot.to_index(), 9);
        assert_eq!(SsangSiot.to_index(), 10);
        assert_eq!(Ieung.to_index(), 11);
        assert_eq!(Jieut.to_index(), 12);
        assert_eq!(SsangJieut.to_index(), 13);
        assert_eq!(Chieut.to_index(), 14);
        assert_eq!(Kiyeok.to_index(), 15);
        assert_eq!(Tieut.to_index(), 16);
        assert_eq!(Pieup.to_index(), 17);
        assert_eq!(Hieuh.to_index(), 18);
    }

    #[test]
    fn to_composable() {
        assert_eq!(Giyeok.to_composable(), 0);
        assert_eq!(SsangGiyeok.to_composable(), 588);
        assert_eq!(Nieun.to_composable(), 1176);
        assert_eq!(Digeut.to_composable(), 1764);
        assert_eq!(SsangDigeut.to_composable(), 2352);
        assert_eq!(Rieul.to_composable(), 2940);
        assert_eq!(Mieum.to_composable(), 3528);
        assert_eq!(Bieup.to_composable(), 4116);
        assert_eq!(SsangBieup.to_composable(), 4704);
        assert_eq!(Siot.to_composable(), 5292);
        assert_eq!(SsangSiot.to_composable(), 5880);
        assert_eq!(Ieung.to_composable(), 6468);
        assert_eq!(Jieut.to_composable(), 7056);
        assert_eq!(SsangJieut.to_composable(), 7644);
        assert_eq!(Chieut.to_composable(), 8232);
        assert_eq!(Kiyeok.to_composable(), 8820);
        assert_eq!(Tieut.to_composable(), 9408);
        assert_eq!(Pieup.to_composable(), 9996);
        assert_eq!(Hieuh.to_composable(), 10584);
    }
}
