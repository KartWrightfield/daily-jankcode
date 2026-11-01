use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecToHex {
    value: u32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Correct,
    Incorrect { expected: String },
    Invalid(ParseError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    NotHex
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "Please enter an answer"),
            ParseError::NotHex => write!(f, "Input is not a valid hexadecimal number"),
        }
    }
}

impl DecToHex {
    pub const MAX: u32 = 255;

    pub fn new(value: u32) -> Self { Self { value } }

    pub fn from_seed(seed: u64) -> Self {
        let value = (mix(seed) % (Self::MAX as u64 + 1)) as u32;
        Self::new(value)
    }

    pub fn value(&self) -> u32 { self.value }

    pub fn answer(&self) -> String { format!("{:X}", self.value) }

    pub fn check(&self, input: &str) -> Outcome {
        match parse_hex(input) {
            Ok(n) if n == self.value => Outcome::Correct,
            Ok(_) => Outcome::Incorrect { expected: self.answer() },
            Err(e) => Outcome::Invalid(e),
        }
    }
}

fn parse_hex(input: &str) -> Result<u32, ParseError> {
    let trimmed = input.trim();
    let digits = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    if digits.is_empty() {
        return Err(ParseError::Empty);
    }
    if !digits.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ParseError::NotHex);
    }
    u32::from_str_radix(digits, 16).map_err(|_| ParseError::NotHex)
}

fn mix(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_is_uppercase_hex() {
        assert_eq!(DecToHex::new(255).answer(), "FF");
        assert_eq!(DecToHex::new(0).answer(), "0");
        assert_eq!(DecToHex::new(16).answer(), "10");
    }

    #[test]
    fn accepts_case_prefix_and_whitespace() {
        let q = DecToHex::new(255);
        assert_eq!(q.check("FF"), Outcome::Correct);
        assert_eq!(q.check("ff"), Outcome::Correct);
        assert_eq!(q.check("0xFF"), Outcome::Correct);
        assert_eq!(q.check("  0Xff  "), Outcome::Correct);
    }

    #[test]
    fn wrong_answer_reports_expected() {
        let q = DecToHex::new(10);
        assert_eq!(q.check("B"), Outcome::Incorrect { expected: "A".into() });
    }

    #[test]
    fn invalid_input_is_reported() {
        let q = DecToHex::new(10);
        assert_eq!(q.check(""), Outcome::Invalid(ParseError::Empty));
        assert_eq!(q.check("0x"), Outcome::Invalid(ParseError::Empty));
        assert_eq!(q.check("G1"), Outcome::Invalid(ParseError::NotHex));
        assert_eq!(q.check("+A"), Outcome::Invalid(ParseError::NotHex));
    }

    #[test]
    fn seed_is_deterministic_and_in_range() {
        for seed in 0..1000 {
            let q = DecToHex::from_seed(seed);
            assert_eq!(q, DecToHex::from_seed(seed));
            assert!(q.value() <= DecToHex::MAX);
        }
    }
}