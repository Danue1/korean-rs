pub(crate) const HANGEUL_OFFSET: u32 = 0xAC00;

pub(crate) const SYLLABLE_START: u32 = 0xAC00;
pub(crate) const SYLLABLE_END: u32 = 0xD7A3;

pub(crate) const JUNGSEONG_COUNT: u32 = 21;
pub(crate) const JONGSEONG_COUNT: u32 = 28;

// normal
pub(crate) const JAMO_START: u32 = 0x1100;
pub(crate) const JAMO_END: u32 = 0x11ff;

pub(crate) const CHOSEONG_START: u32 = 0x1100;
// pub(crate) const CHOSEONG_END: u32 = 0x1112;

pub(crate) const JUNGSEONG_START: u32 = 0x1161;
// pub(crate) const JUNGSEONG_END: u32 = 0x1175;

pub(crate) const JONGSEONG_START: u32 = 0x11A8;
// pub(crate) const JONGSEONG_END: u32 = 0x11C2;

// compat
pub(crate) const COMPAT_JAMO_START: u32 = COMPAT_CHOSEONG_START;
pub(crate) const COMPAT_JAMO_END: u32 = COMPAT_JONGSEONG_END;

pub(crate) const COMPAT_CHOSEONG_START: u32 = 0x3131;
// pub(crate) const COMPAT_CHOSEONG_END: u32 = 0x314E;

// pub(crate) const COMPAT_JUNGSEONG_START: u32 = 0x314F;
// pub(crate) const COMPAT_JUNGSEONG_END: u32 = 0x3163;

// pub(crate) const COMPAT_JONGSEONG_START: u32 = 0x3165;
pub(crate) const COMPAT_JONGSEONG_END: u32 = 0x318E;
