/// Disassemble Hangul letters into Hangul Compatibility Jamo
pub trait HangulDissassemble<T: Iterator<Item = char>> {
    /// Creates an iterator that yields Jamos of each hangul letter.
    ///
    /// # Examples:
    ///
    /// Hangul Syllable decomposes into its subparts:
    /// ```
    /// use korean::*;
    ///
    /// let hangul = "한글";
    ///
    /// let parts: String = hangul.disassemble().collect();
    ///
    /// assert_eq!("ㅎㅏㄴㄱㅡㄹ", parts);
    /// ```
    ///
    /// Hangul Compatibility Jamo can decompose into its subparts:
    /// ```
    /// use korean::*;
    ///
    /// let compound = "ㄳ";
    ///
    /// let parts: String = compound.disassemble().collect();
    ///
    /// assert_eq!("ㄱㅅ", parts);
    /// ```
    fn disassemble(self) -> Disassemble<T>;
}

/// An iterator that yields Jamos of hangul string.
///
/// This struct is created by [`disassemble`] method on [`HangulDisassemble`] trait. See its documentation for more.
///
/// [`disassemble`]: trait.HangulDissassemble.html#tymethod.disassemble
/// [`HangulDisassemble`]: trait.HangulDisassemble.html
pub struct Disassemble<T>
where
    T: Iterator<Item = char>,
{
    input: T,
    buffer: Vec<char>,
}

impl<'a, T: Iterator<Item = char> + 'a> Iterator for Disassemble<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            Some(self.buffer.remove(0))
        } else if let Some(letter) = self.input.next() {
            Some(decompose(letter, &mut self.buffer).unwrap_or(letter))
        } else {
            None
        }
    }
}

impl<'a> HangulDissassemble<std::str::Chars<'a>> for &'a str {
    fn disassemble(self) -> Disassemble<std::str::Chars<'a>> {
        Disassemble {
            input: self.chars(),
            buffer: Vec::new(),
        }
    }
}

impl<'a> HangulDissassemble<std::str::Chars<'a>> for &'a String {
    fn disassemble(self) -> Disassemble<std::str::Chars<'a>> {
        Disassemble {
            input: self.chars(),
            buffer: Vec::new(),
        }
    }
}

impl<'a, T: Iterator<Item = char>> HangulDissassemble<T> for T {
    fn disassemble(self) -> Disassemble<T> {
        Disassemble {
            input: self,
            buffer: Vec::new(),
        }
    }
}

/// Assemble Hangul Jamos into Hangul Syllables
pub trait HangulAssemble {
    /// Collect Hangul Jamos into a string of Hangul Syllables.
    ///
    /// # Example:
    /// ```
    /// use korean::*;
    ///
    /// let parts = "ㅋㅗㄷㅣㅇ";
    ///
    /// let assembled = parts.chars().assemble();
    ///
    /// assert_eq!("코딩", assembled);
    /// ```
    fn assemble(self) -> String;
}

#[derive(Clone, Copy)]
enum CombineStage {
    None,
    ChosungOnly(char),
    JungsungOnly(char),
    ChosungJungsung(char, char),
    NonCombinedFull(char, char, char),
    CombinedFull {
        cho: char,
        jung: char,
        jong: char,
        secondary_jong: char,
        combined_jong: char,
    },
}

impl<T: Iterator<Item = char>> HangulAssemble for T {
    fn assemble(self) -> String {
        let mut stage = CombineStage::None;
        let mut output = String::new();
        for letter in self {
            match (stage, letter) {
                (CombineStage::None, 'ㄱ'..='ㅎ') if !is_combined(letter) => {
                    stage = CombineStage::ChosungOnly(letter);
                }
                (CombineStage::None, 'ㅏ'..='ㅣ') => {
                    stage = CombineStage::JungsungOnly(letter);
                }
                (CombineStage::None, _) => {
                    output.push(letter);
                }
                (CombineStage::ChosungOnly(prev_cho), 'ㄱ'..='ㅎ') if !is_combined(letter) => {
                    stage = CombineStage::ChosungOnly(letter);
                    output.push(prev_cho);
                }
                (CombineStage::ChosungOnly(prev_cho), 'ㅏ'..='ㅣ') => {
                    stage = CombineStage::ChosungJungsung(prev_cho, letter);
                }
                (CombineStage::ChosungOnly(prev_cho), _) => {
                    stage = CombineStage::None;
                    output.push(prev_cho);
                    output.push(letter);
                }
                (CombineStage::JungsungOnly(prev_jung), 'ㄱ'..='ㅎ') if !is_combined(letter) => {
                    stage = CombineStage::ChosungOnly(letter);
                    output.push(prev_jung);
                }
                (CombineStage::JungsungOnly(prev_jung), 'ㅏ'..='ㅣ') => {
                    if let Some(combined) = combine_jungsung(prev_jung, letter) {
                        stage = CombineStage::JungsungOnly(combined);
                    } else {
                        stage = CombineStage::JungsungOnly(letter);
                        output.push(prev_jung);
                    }
                }
                (CombineStage::JungsungOnly(prev_jung), _) => {
                    stage = CombineStage::None;
                    output.push(prev_jung);
                    output.push(letter);
                }
                (CombineStage::ChosungJungsung(cho, jung), 'ㄱ'..='ㅎ') if !is_combined(letter) => {
                    if is_jongsung(letter) {
                        stage = CombineStage::NonCombinedFull(cho, jung, letter);
                    } else {
                        stage = CombineStage::ChosungOnly(letter);
                        output.push(build_hangul(cho, jung, None));
                    }
                }
                (CombineStage::ChosungJungsung(cho, jung), 'ㅏ'..='ㅣ') => {
                    if let Some(combined) = combine_jungsung(jung, letter) {
                        stage = CombineStage::ChosungJungsung(cho, combined);
                    } else {
                        stage = CombineStage::JungsungOnly(letter);
                        output.push(build_hangul(cho, jung, None));
                    }
                }
                (CombineStage::ChosungJungsung(cho, jung), _) => {
                    stage = CombineStage::None;
                    output.push(build_hangul(cho, jung, None));
                    output.push(letter);
                }
                (CombineStage::NonCombinedFull(cho, jung, jong), 'ㄱ'..='ㅎ')
                    if !is_combined(letter) =>
                {
                    if let Some(combined) = combine_jongsung(jong, letter) {
                        stage = CombineStage::CombinedFull {
                            cho,
                            jung,
                            jong,
                            secondary_jong: letter,
                            combined_jong: combined,
                        };
                    } else {
                        stage = CombineStage::ChosungOnly(letter);
                        output.push(build_hangul(cho, jung, Some(jong)));
                    }
                }
                (CombineStage::NonCombinedFull(cho, jung, jong), 'ㅏ'..='ㅣ') => {
                    stage = CombineStage::ChosungJungsung(jong, letter);
                    output.push(build_hangul(cho, jung, None));
                }
                (CombineStage::NonCombinedFull(cho, jung, jong), _) => {
                    stage = CombineStage::None;
                    output.push(build_hangul(cho, jung, Some(jong)));
                    output.push(letter);
                }
                (
                    CombineStage::CombinedFull {
                        cho,
                        jung,
                        combined_jong,
                        ..
                    },
                    'ㄱ'..='ㅎ',
                ) if !is_combined(letter) => {
                    stage = CombineStage::ChosungOnly(letter);
                    output.push(build_hangul(cho, jung, Some(combined_jong)));
                }
                (
                    CombineStage::CombinedFull {
                        cho,
                        jung,
                        jong,
                        secondary_jong,
                        ..
                    },
                    'ㅏ'..='ㅣ',
                ) => {
                    stage = CombineStage::ChosungJungsung(secondary_jong, letter);
                    output.push(build_hangul(cho, jung, Some(jong)));
                }
                (
                    CombineStage::CombinedFull {
                        cho,
                        jung,
                        combined_jong,
                        ..
                    },
                    _,
                ) => {
                    stage = CombineStage::None;
                    output.push(build_hangul(cho, jung, Some(combined_jong)));
                    output.push(letter);
                }
            }
        }
        match stage {
            CombineStage::None => {}
            CombineStage::ChosungOnly(cho) => {
                output.push(cho);
            }
            CombineStage::JungsungOnly(jung) => {
                output.push(jung);
            }
            CombineStage::ChosungJungsung(cho, jung) => {
                output.push(build_hangul(cho, jung, None));
            }
            CombineStage::NonCombinedFull(cho, jung, jong) => {
                output.push(build_hangul(cho, jung, Some(jong)));
            }
            CombineStage::CombinedFull {
                cho,
                jung,
                combined_jong,
                ..
            } => {
                output.push(build_hangul(cho, jung, Some(combined_jong)));
            }
        }
        output
    }
}

/// Represents a disassembled hangul letter
/// 
/// # Examples
/// 
/// Create with jamo parts:
/// ```
/// use korean::Hangul;
/// 
/// let word: char = Hangul::new('ㅁ', 'ㅏ', Some('ㄹ')).unwrap().into();
/// assert_eq!('말', word);
/// 
/// let error = Hangul::new('ㄱ', '*', Some('ㅅ'));
/// assert!(error.is_none());
/// ```
/// 
/// Create from char:
/// ```
/// use korean::Hangul;
/// use std::convert::TryInto;
/// 
/// let night: Hangul = '밤'.try_into().unwrap();
/// assert_eq!(('ㅂ', 'ㅏ', Some('ㅁ')), night.parts());
/// ```
#[derive(Debug)]
pub struct Hangul(u32, u32, u32);

impl Hangul {
    /// Creates a Hangul object with jamo parts.
    /// 
    /// Returns `None` when one of parts is invalid for its position.
    pub fn new(cho: char, jung: char, jong: Option<char>) -> Option<Self> {
        let cho = try_calculate_chosung_index(cho)?;
        let jong = try_calculate_jongsung_index(jong)?;
        if ('ㅏ'..='ㅣ').contains(&jung) {
            Some(Hangul(cho, jung as u32 - 0x314F, jong))
        } else {
            None
        }
    }

    /// Returns a tuple of parts which compose this Hangul object.
    /// 
    /// # Example
    /// ```
    /// use korean::Hangul;
    /// use std::convert::TryInto;
    ///
    /// let sheep: Hangul = '양'.try_into().unwrap();
    /// assert_eq!(('ㅇ', 'ㅑ', Some('ㅇ')), sheep.parts());
    /// ```
    pub fn parts(&self) -> (char, char, Option<char>) {
        (
            HANGUL_CHOSUNG_TO_COMPATIBILITY[self.0 as usize],
            HANGUL_JUNGSUNG_TO_COMPATIBILITY[self.1 as usize],
            if self.2 == 0 {
                None
            } else {
                Some(HANGUL_JONGSUNG_TO_COMPATIBILITY[self.2 as usize - 1])
            }
        )
    }

    /// Sets chosung of Hangul.
    /// 
    /// Returns `Err(cho)` when the parameter is not valid for chosung position.
    pub fn set_cho(&mut self, cho: char) -> Result<(), char> {
        self.0 = try_calculate_chosung_index(cho).ok_or(cho)?;
        Ok(())
    }

    /// Sets jungsung of Hangul.
    ///
    /// Returns `Err(jung)` when the parameter is not valid for jungsung position.
    pub fn set_jung(&mut self, jung: char) -> Result<(), char> {
        if ('ㅏ'..='ㅣ').contains(&jung) {
            self.1 = jung as u32 - 0x314F;
            Ok(())
        } else {
            Err(jung)
        }
    }

    /// Sets jongsung of Hangul.
    /// 
    /// Returns `Err(jong)` when the parameter is not valid for jongsung position.
    pub fn set_jong(&mut self, jong: Option<char>) -> Result<(), char> {
        if let Some(jong) = try_calculate_jongsung_index(jong) {
            self.1 = jong;
            Ok(())
        } else {
            Err(jong.unwrap())
        }
    }
}

impl std::convert::TryFrom<char> for Hangul {
    type Error = char;

    fn try_from(letter: char) -> Result<Self, char> {
        match letter {
            '가'..='힣' => {
                let mut code = letter as u32 - 0xAC00;
                let jongsung = code % 28;
                code /= 28;
                let jungsung = code % 21;
                code /= 21;
                let chosung = code;
                Ok(Hangul(
                    chosung,
                    jungsung,
                    jongsung,
                ))
            }
            _ => Err(letter),
        }
    }
}

impl From<Hangul> for char {
    fn from(hangul: Hangul) -> char {
        std::char::from_u32(0xAC00 + (hangul.0 * 21 + hangul.1) * 28 + hangul.2).unwrap()
    }
}

// Maps chosung (onset) to compatibility jamo
const HANGUL_CHOSUNG_TO_COMPATIBILITY: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];
// Maps jungsung (nucleus) to compatibility jamo
const HANGUL_JUNGSUNG_TO_COMPATIBILITY: [char; 21] = [
    'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
    'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
];
// Maps jongsung (coda) to compatibility jamo
const HANGUL_JONGSUNG_TO_COMPATIBILITY: [char; 27] = [
    'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ',
    'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
];

fn break_jungsung(letter: char) -> Option<[char; 2]> {
    match letter {
        'ㅘ' => Some(['ㅗ', 'ㅏ']),
        'ㅙ' => Some(['ㅗ', 'ㅐ']),
        'ㅚ' => Some(['ㅗ', 'ㅣ']),
        'ㅝ' => Some(['ㅜ', 'ㅓ']),
        'ㅞ' => Some(['ㅜ', 'ㅔ']),
        'ㅟ' => Some(['ㅜ', 'ㅣ']),
        'ㅢ' => Some(['ㅡ', 'ㅣ']),
        _ => None,
    }
}

fn break_jongsung(letter: char) -> Option<[char; 2]> {
    match letter {
        'ㄳ' => Some(['ㄱ', 'ㅅ']),
        'ㄵ' => Some(['ㄴ', 'ㅈ']),
        'ㄶ' => Some(['ㄴ', 'ㅎ']),
        'ㄺ' => Some(['ㄹ', 'ㄱ']),
        'ㄻ' => Some(['ㄹ', 'ㅁ']),
        'ㄼ' => Some(['ㄹ', 'ㅂ']),
        'ㄽ' => Some(['ㄹ', 'ㅅ']),
        'ㄾ' => Some(['ㄹ', 'ㅌ']),
        'ㄿ' => Some(['ㄹ', 'ㅍ']),
        'ㅀ' => Some(['ㄹ', 'ㅎ']),
        'ㅄ' => Some(['ㅂ', 'ㅅ']),
        _ => None,
    }
}

fn decompose(c: char, buffer: &mut Vec<char>) -> Option<char> {
    let letter = c as usize;
    match letter {
        // Hangul Compatibility Jamo (modern consonant)
        0x3131..=0x314E => {
            if let Some(parts) = break_jongsung(c) {
                buffer.push(parts[1]);
                Some(parts[0])
            } else {
                Some(c)
            }
        }
        // Hangul Compatibility Jamo (modern vowel)
        0x314F..=0x3163 => {
            if let Some(parts) = break_jungsung(c) {
                buffer.push(parts[1]);
                Some(parts[0])
            } else {
                Some(c)
            }
        }
        // Hangul Syllables
        0xAC00..=0xD7A3 => {
            let mut code = letter - 0xAC00;
            let jongsung = code % 28;
            code /= 28;
            let jungsung = code % 21;
            code /= 21;
            let chosung = code;
            {
                let jungsung = HANGUL_JUNGSUNG_TO_COMPATIBILITY[jungsung];
                if let Some(parts) = break_jungsung(jungsung) {
                    buffer.extend_from_slice(&parts);
                } else {
                    buffer.push(jungsung);
                }
            }
            if jongsung > 0 {
                let jongsung = HANGUL_JONGSUNG_TO_COMPATIBILITY[jongsung - 1];
                if let Some(parts) = break_jongsung(jongsung) {
                    buffer.extend_from_slice(&parts);
                } else {
                    buffer.push(jongsung);
                }
            }
            Some(HANGUL_CHOSUNG_TO_COMPATIBILITY[chosung])
        }
        _ => None,
    }
}

fn build_hangul(cho: char, jung: char, jong: Option<char>) -> char {
    Hangul::new(cho, jung, jong).unwrap().into()
}

fn combine_jungsung(a: char, b: char) -> Option<char> {
    match (a, b) {
        ('ㅗ', 'ㅏ') => Some('ㅘ'),
        ('ㅗ', 'ㅐ') => Some('ㅙ'),
        ('ㅗ', 'ㅣ') => Some('ㅚ'),
        ('ㅜ', 'ㅓ') => Some('ㅝ'),
        ('ㅜ', 'ㅔ') => Some('ㅞ'),
        ('ㅜ', 'ㅣ') => Some('ㅟ'),
        ('ㅡ', 'ㅣ') => Some('ㅢ'),
        _ => None,
    }
}

fn combine_jongsung(a: char, b: char) -> Option<char> {
    match (a, b) {
        ('ㄱ', 'ㅅ') => Some('ㄳ'),
        ('ㄴ', 'ㅈ') => Some('ㄵ'),
        ('ㄴ', 'ㅎ') => Some('ㄶ'),
        ('ㄹ', 'ㄱ') => Some('ㄺ'),
        ('ㄹ', 'ㅁ') => Some('ㄻ'),
        ('ㄹ', 'ㅂ') => Some('ㄼ'),
        ('ㄹ', 'ㅅ') => Some('ㄽ'),
        ('ㄹ', 'ㅌ') => Some('ㄾ'),
        ('ㄹ', 'ㅎ') => Some('ㅀ'),
        ('ㅂ', 'ㅅ') => Some('ㅄ'),
        _ => None,
    }
}

fn is_combined(a: char) -> bool {
    match a {
        'ㄳ' | 'ㄵ' | 'ㄶ' | 'ㄺ' | 'ㄻ' | 'ㄼ' | 'ㄽ' | 'ㄾ' | 'ㄿ' | 'ㅀ' | 'ㅄ' => {
            true
        }
        _ => false,
    }
}

fn is_jongsung(a: char) -> bool {
    match a {
        'ㄸ' | 'ㅃ' | 'ㅉ' => false,
        'ㄱ'..='ㅎ' => true,
        _ => false,
    }
}

fn try_calculate_chosung_index(cho: char) -> Option<u32> {
    let index = cho as u32;
    if index < 0x3131 {
        None
    } else {
        let index = index - 0x3131;
        match cho {
            'ㄱ'..='ㄲ' => Some(index),
            'ㄴ' => Some(index - 1),
            'ㄷ'..='ㄹ' => Some(index - 3),
            'ㅁ'..='ㅃ' => Some(index - 10),
            'ㅅ'..='ㅎ' => Some(index - 11),
            _ => None,
        }
    }
}

fn try_calculate_jongsung_index(jong: Option<char>) -> Option<u32> {
    if let Some(jong) = jong {
        let index = jong as u32;
        if index < 0x3131 {
            None
        } else { 
            let index = index - 0x3131 + 1;
            match jong {
                'ㄱ'..='ㄷ' => Some(index),
                'ㄹ'..='ㅂ' => Some(index - 1),
                'ㅄ'..='ㅈ' => Some(index - 2),
                'ㅊ'..='ㅎ' => Some(index - 3),
                _ => None,
            }
        }
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::*;

    #[test]
    fn disassemble_no_jongsung() {
        assert_eq!("ㅇㅣㅅㅏ", "이사".disassemble().collect::<String>());
    }

    #[test]
    fn disassemble_jongsung() {
        assert_eq!("ㅇㅣㅅㅏㅇ", "이상".disassemble().collect::<String>());
    }

    #[test]
    fn disassemble_compound_jongsung() {
        assert_eq!("ㅇㅏㄴㅈㄷㅏ", "앉다".disassemble().collect::<String>());
    }

    #[test]
    fn disassemble_compound_jungsung() {
        assert_eq!("ㅊㅏㅁㅇㅗㅣ", "참외".disassemble().collect::<String>());
    }

    #[test]
    fn disassmble_consonant() {
        assert_eq!(
            "ㄱㅅㄴㅈㄴㅎㄹㄱㄹㅁㄹㅂㄹㅅㄹㅌㄹㅍㄹㅎㅂㅅ",
            "ㄳㄵㄶㄺㄻㄼㄽㄾㄿㅀㅄ".disassemble().collect::<String>()
        );
    }

    #[test]
    fn disassmble_vowel() {
        assert_eq!(
            "ㅗㅏㅗㅐㅗㅣㅜㅓㅜㅔㅜㅣㅡㅣ",
            "ㅘㅙㅚㅝㅞㅟㅢ".disassemble().collect::<String>()
        );
    }

    #[test]
    fn assemble_no_jongsung() {
        assert_eq!("고구마", "ㄱㅗㄱㅜㅁㅏ".chars().assemble());
    }

    #[test]
    fn assemble_jongsung() {
        assert_eq!("감자", "ㄱㅏㅁㅈㅏ".chars().assemble());
    }

    #[test]
    fn assemble_compound_jungsung() {
        assert_eq!("휘발유", "ㅎㅜㅣㅂㅏㄹㅇㅠ".chars().assemble());
    }

    #[test]
    fn assemble_compound_jongsung() {
        assert_eq!("훑개", "ㅎㅜㄹㅌㄱㅐ".chars().assemble());
    }

    #[test]
    fn disassemble_single() {
        use std::convert::*;
        let disassembled: Hangul = '닭'.try_into().unwrap();
        assert_eq!(('ㄷ', 'ㅏ', Some('ㄺ')), disassembled.parts());
    }

    #[test]
    fn disassemble_single_no_jongsung() {
        use std::convert::*;
        let disassembled: Hangul = '깨'.try_into().unwrap();
        assert_eq!(('ㄲ', 'ㅐ', None), disassembled.parts());
    }

    #[test]
    fn disassemble_nonhangul_error() {
        use std::convert::*;
        let disassembled: Result<Hangul, _> = 'a'.try_into();
        assert!(disassembled.is_err());
    }

    #[test]
    fn assemble_single() {
        let assembled: char = Hangul::new('ㅂ', 'ㅏ', Some('ㅌ')).unwrap().into();
        assert_eq!('밭', assembled);
    }

    #[test]
    fn assemble_single_no_jongsung() {
        let assembled: char = Hangul::new('ㅊ', 'ㅟ', None).unwrap().into();
        assert_eq!('취', assembled);
    }

    #[test]
    fn assemble_nonhangul_error() {
        assert!(Hangul::new('a', 'b', None).is_none());
    }

    proptest! {
        #[test]
        fn no_panic(korean in "[가-힣ㄱ-ㅎㅏ-ㅣ]+") {
            korean.disassemble().assemble();
        }

        #[test]
        fn no_panic_single(letter in proptest::arbitrary::any::<char>()) {
            use std::convert::*;
            let _: Option<char> = letter.try_into().ok().and_then(|hangul: Hangul| hangul.try_into().ok());
        }

        #[test]
        fn hangul_convert_redundant(letter in proptest::char::range('가', '\u{D7A4}')) {
            use std::convert::*;
            let disassembled: Hangul = letter.try_into().unwrap();
            let processed = disassembled.into();
            assert_eq!(letter, processed);
        }
    }
}
