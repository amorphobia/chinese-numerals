use crate::{characters::*, ChineseNumeralBase, Sign, Signed};

/// Short scale integers (下数).
///
/// 「下数者，十十变之。若言十万曰亿，十亿曰兆，十兆曰京也。」
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
pub struct ShortScaleInt {
    pub(super) sign: Sign,
    pub(super) data: u64,
}

impl ShortScaleInt {
    pub(super) const MAX_ABS: u64 = 999_9999_9999_9999;

    /// The maximum integer can be expressed in short scale.
    pub const MAX: Self = Self {
        sign: Sign::Pos,
        data: Self::MAX_ABS,
    };

    /// The minimum integer can be expressed in short scale
    pub const MIN: Self = Self {
        sign: Sign::Neg,
        data: Self::MAX_ABS,
    };
}

impl ChineseNumeralBase for ShortScaleInt {
    fn to_chars(&self) -> Vec<NumChar> {
        let mut chars = Vec::new();
        let mut num = *self.data();
        let mut prev_rem = 1;

        for exp in 9..=23 {
            let rem = num % 10;
            num /= 10;

            if rem > 0 {
                if !chars.is_empty() && prev_rem < 1 {
                    chars.push(NUM_CHARS[0]);
                }
                if exp > 9 {
                    chars.push(NUM_CHARS[exp]);
                }
                chars.push(NUM_CHARS[rem as usize]);
            }
            prev_rem = rem;
        }
        chars
    }

    fn to_chars_trimmed(&self) -> Vec<NumChar> {
        let mut chars = self.to_chars();
        if self.data() >= &10 && self.data() <= &19 {
            let one = chars.pop();
            debug_assert_eq!(one, Some(NumChar::One));
        }
        chars
    }
}
