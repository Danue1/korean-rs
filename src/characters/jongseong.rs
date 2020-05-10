use self::JongseongCharacter::*;
use crate::constants::*;
use crate::syllable::*;
use crate::CharacterInformation;

/// See:
/// * https://en.wikipedia.org/wiki/Hangul_Jamo_(Unicode_block)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum JongseongCharacter {
    /// ᆨ
    Giyeok,
    /// ᆩ
    SsangGiyeok,
    /// ᆪ
    GiyeokSiot,
    /// ᆫ
    Nieun,
    /// ᆬ
    NieunJieut,
    /// ᆭ
    NieunHieuh,
    /// ᆮ
    Digeut,
    /// ᆯ
    Rieul,
    /// ᆰ
    RieulGiyeok,
    /// ᆱ
    RieulMieum,
    /// ᆲ
    RieulBieup,
    /// ᆳ
    RieulSiot,
    /// ᆴ
    RieulTieut,
    /// ᆵ
    RieulPieup,
    /// ᆶ
    RieulHieuh,
    /// ᆷ
    Mieum,
    /// ᆸ
    Bieup,
    /// ᆹ
    BieupSiot,
    /// ᆺ
    Siot,
    /// ᆻ
    SsangSiot,
    /// ᆼ
    Ieung,
    /// ᆽ
    Jieut,
    /// ᆾ
    Chieut,
    /// ᆿ
    Kieuk,
    /// ᇀ
    Tieut,
    /// ᇁ
    Pieup,
    /// ᇂ
    Hieuh,
}

impl JongseongCharacter {
    pub(crate) fn to_code(code: u32) -> u32 {
        if code.is_syllable() {
            let value = (code - HANGEUL_OFFSET) % JONGSEONG_COUNT;
            value + JONGSEONG_START - 1
        } else {
            code
        }
    }

    pub fn to_index(&self) -> u32 {
        match self {
            Giyeok => 1,
            SsangGiyeok => 2,
            GiyeokSiot => 3,
            Nieun => 4,
            NieunJieut => 5,
            NieunHieuh => 6,
            Digeut => 7,
            Rieul => 8,
            RieulGiyeok => 9,
            RieulMieum => 10,
            RieulBieup => 11,
            RieulSiot => 12,
            RieulTieut => 13,
            RieulPieup => 14,
            RieulHieuh => 15,
            Mieum => 16,
            Bieup => 17,
            BieupSiot => 18,
            Siot => 19,
            SsangSiot => 20,
            Ieung => 21,
            Jieut => 22,
            Chieut => 23,
            Kieuk => 24,
            Tieut => 25,
            Pieup => 26,
            Hieuh => 27,
        }
    }
}

impl CharacterInformation for JongseongCharacter {
    fn is_jaeum(&self) -> bool {
        false
    }

    fn is_moeum(&self) -> bool {
        true
    }

    fn to_composable(&self) -> u32 {
        self.to_index()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_jaeum() {
        assert_eq!(Giyeok.is_jaeum(), false);
        assert_eq!(SsangGiyeok.is_jaeum(), false);
        assert_eq!(GiyeokSiot.is_jaeum(), false);
        assert_eq!(Nieun.is_jaeum(), false);
        assert_eq!(NieunJieut.is_jaeum(), false);
        assert_eq!(NieunHieuh.is_jaeum(), false);
        assert_eq!(Digeut.is_jaeum(), false);
        assert_eq!(Rieul.is_jaeum(), false);
        assert_eq!(RieulGiyeok.is_jaeum(), false);
        assert_eq!(RieulMieum.is_jaeum(), false);
        assert_eq!(RieulBieup.is_jaeum(), false);
        assert_eq!(RieulSiot.is_jaeum(), false);
        assert_eq!(RieulTieut.is_jaeum(), false);
        assert_eq!(RieulPieup.is_jaeum(), false);
        assert_eq!(RieulHieuh.is_jaeum(), false);
        assert_eq!(Mieum.is_jaeum(), false);
        assert_eq!(Bieup.is_jaeum(), false);
        assert_eq!(BieupSiot.is_jaeum(), false);
        assert_eq!(Siot.is_jaeum(), false);
        assert_eq!(SsangSiot.is_jaeum(), false);
        assert_eq!(Ieung.is_jaeum(), false);
        assert_eq!(Jieut.is_jaeum(), false);
        assert_eq!(Chieut.is_jaeum(), false);
        assert_eq!(Kieuk.is_jaeum(), false);
        assert_eq!(Tieut.is_jaeum(), false);
        assert_eq!(Pieup.is_jaeum(), false);
        assert_eq!(Hieuh.is_jaeum(), false);
    }

    #[test]
    fn is_moeum() {
        assert_eq!(Giyeok.is_moeum(), true);
        assert_eq!(SsangGiyeok.is_moeum(), true);
        assert_eq!(GiyeokSiot.is_moeum(), true);
        assert_eq!(Nieun.is_moeum(), true);
        assert_eq!(NieunJieut.is_moeum(), true);
        assert_eq!(NieunHieuh.is_moeum(), true);
        assert_eq!(Digeut.is_moeum(), true);
        assert_eq!(Rieul.is_moeum(), true);
        assert_eq!(RieulGiyeok.is_moeum(), true);
        assert_eq!(RieulMieum.is_moeum(), true);
        assert_eq!(RieulBieup.is_moeum(), true);
        assert_eq!(RieulSiot.is_moeum(), true);
        assert_eq!(RieulTieut.is_moeum(), true);
        assert_eq!(RieulPieup.is_moeum(), true);
        assert_eq!(RieulHieuh.is_moeum(), true);
        assert_eq!(Mieum.is_moeum(), true);
        assert_eq!(Bieup.is_moeum(), true);
        assert_eq!(BieupSiot.is_moeum(), true);
        assert_eq!(Siot.is_moeum(), true);
        assert_eq!(SsangSiot.is_moeum(), true);
        assert_eq!(Ieung.is_moeum(), true);
        assert_eq!(Jieut.is_moeum(), true);
        assert_eq!(Chieut.is_moeum(), true);
        assert_eq!(Kieuk.is_moeum(), true);
        assert_eq!(Tieut.is_moeum(), true);
        assert_eq!(Pieup.is_moeum(), true);
        assert_eq!(Hieuh.is_moeum(), true);
    }

    #[test]
    fn to_index() {
        assert_eq!(Giyeok.to_index(), 1);
        assert_eq!(SsangGiyeok.to_index(), 2);
        assert_eq!(GiyeokSiot.to_index(), 3);
        assert_eq!(Nieun.to_index(), 4);
        assert_eq!(NieunJieut.to_index(), 5);
        assert_eq!(NieunHieuh.to_index(), 6);
        assert_eq!(Digeut.to_index(), 7);
        assert_eq!(Rieul.to_index(), 8);
        assert_eq!(RieulGiyeok.to_index(), 9);
        assert_eq!(RieulMieum.to_index(), 10);
        assert_eq!(RieulBieup.to_index(), 11);
        assert_eq!(RieulSiot.to_index(), 12);
        assert_eq!(RieulTieut.to_index(), 13);
        assert_eq!(RieulPieup.to_index(), 14);
        assert_eq!(RieulHieuh.to_index(), 15);
        assert_eq!(Mieum.to_index(), 16);
        assert_eq!(Bieup.to_index(), 17);
        assert_eq!(BieupSiot.to_index(), 18);
        assert_eq!(Siot.to_index(), 19);
        assert_eq!(SsangSiot.to_index(), 20);
        assert_eq!(Ieung.to_index(), 21);
        assert_eq!(Jieut.to_index(), 22);
        assert_eq!(Chieut.to_index(), 23);
        assert_eq!(Kieuk.to_index(), 24);
        assert_eq!(Tieut.to_index(), 25);
        assert_eq!(Pieup.to_index(), 26);
        assert_eq!(Hieuh.to_index(), 27);
    }

    #[test]
    fn to_composable() {
        assert_eq!(Giyeok.to_composable(), 1);
        assert_eq!(SsangGiyeok.to_composable(), 2);
        assert_eq!(GiyeokSiot.to_composable(), 3);
        assert_eq!(Nieun.to_composable(), 4);
        assert_eq!(NieunJieut.to_composable(), 5);
        assert_eq!(NieunHieuh.to_composable(), 6);
        assert_eq!(Digeut.to_composable(), 7);
        assert_eq!(Rieul.to_composable(), 8);
        assert_eq!(RieulGiyeok.to_composable(), 9);
        assert_eq!(RieulMieum.to_composable(), 10);
        assert_eq!(RieulBieup.to_composable(), 11);
        assert_eq!(RieulSiot.to_composable(), 12);
        assert_eq!(RieulTieut.to_composable(), 13);
        assert_eq!(RieulPieup.to_composable(), 14);
        assert_eq!(RieulHieuh.to_composable(), 15);
        assert_eq!(Mieum.to_composable(), 16);
        assert_eq!(Bieup.to_composable(), 17);
        assert_eq!(BieupSiot.to_composable(), 18);
        assert_eq!(Siot.to_composable(), 19);
        assert_eq!(SsangSiot.to_composable(), 20);
        assert_eq!(Ieung.to_composable(), 21);
        assert_eq!(Jieut.to_composable(), 22);
        assert_eq!(Chieut.to_composable(), 23);
        assert_eq!(Kieuk.to_composable(), 24);
        assert_eq!(Tieut.to_composable(), 25);
        assert_eq!(Pieup.to_composable(), 26);
        assert_eq!(Hieuh.to_composable(), 27);
    }
}
