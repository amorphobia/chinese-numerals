//! # Chinese Numerals
//!
//! Converts primitive integers and [big integers](num_bigint) to [Chinese numerals](https://en.wikipedia.org/wiki/Chinese_numerals).
//!
//! According to 《[五经算术](https://zh.wikipedia.org/wiki/%E4%BA%94%E7%B6%93%E7%AE%97%E8%A1%93)》, for representing numbers larger than 1,0000, there have been ten names (亿, 兆, 京, 垓, 秭, 壤, 沟, 涧, 正, and 载) and three systems (下数 short scale, 中数 mid-scale, 上数 long scale). Plus the myriad scale, in which each name represents a number 1,0000 times the previous, this crate can convert integers to four scales.
//!
//! ## Usage
//!
//! Add to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! chinese-numerals = "0.2"
//! ```
//!
//! All structs have implemented [`Display`](std::fmt::Display) trait's normal (with `"{}"`) and alternative (with `"{:#}"`) formats, converting to lowercase and uppercase Chinese numbers. Besides, [`ChineseNumeral`] trait provides following functions:
//!
//! - [`to_lowercase`](crate::ChineseNumeral::to_lowercase)
//! - [`to_lowercase_simp`](crate::ChineseNumeral::to_lowercase_simp)
//! - [`to_lowercase_trad`](crate::ChineseNumeral::to_lowercase_trad)
//! - [`to_uppercase`](crate::ChineseNumeral::to_uppercase)
//! - [`to_uppercase_simp`](crate::ChineseNumeral::to_uppercase_simp)
//! - [`to_uppercase_trad`](crate::ChineseNumeral::to_uppercase_trad)
//!
//! ## Premitive Integers
//!
//! For each scale, a struct has been implemented to perform the convertion.
//!
//! [`ShortScaleInt`] has implemented [`From`] trait for `i8`, `u8`, `i16`, `u16`, `i32`, and `u32`, and [`TryFrom`] trait for `i64`, `u64`, `i128`, `u128`, `isize`, and `usize`.
//!
//! [`MyriadScaleInt`], [`MidScaleInt`], and [`LongScaleInt`] have implemented `From` trait for all premitive integers.
//!
//! ### Examples
//! ```
//! use chinese_numerals::{ChineseNumeral, ShortScaleInt, MidScaleInt};
//!
//! let num = ShortScaleInt::from(1_0203_0405);
//! assert_eq!("一垓零二兆零三万零四百零五", format!("{}", num));
//! assert_eq!("壹垓零贰兆零叁万零肆佰零伍", format!("{:#}", num));
//!
//! let num = MidScaleInt::from(1_0203_0405);
//! assert_eq!("一億零二百零三萬零四百零五", num.to_lowercase_trad());
//! assert_eq!("壹億零貳佰零叄萬零肆佰零伍", num.to_uppercase_trad());
//! ```
//!
//! ## Big Integers
//!
//! For scales except short scale, a struct has been implemented to perform the convertion from [`BigInt`](num_bigint::BigInt) and [`BigUint`](num_bigint::BigUint).
//!
//! [`MyriadScaleBigInt`], [`MidScaleBigInt`], and [`LongScaleBigInt`] have implemented `TryFrom` trait for both `BigInt` and `BigUint`.
//!
//! ### Dependencies
//!
//! To enable `bigint` feature, set dependencies in `Cargo.toml`:
//! ```toml
//! [dependencies]
//! num-bigint = "0.4"
//! chinese-numerals = { version = "0.2", features = ["bigint"] }
//! ```
//!
//! ### Examples
//! ```
//! use chinese_numerals::{ChineseNumeral, LongScaleBigInt};
//! use num_bigint::BigUint;
//!
//! // 130_5480_5271_5637_0597_2964
//! let num = BigUint::new(vec![463665380, 3016835882, 707]);
//! let num = LongScaleBigInt::try_from(num).expect("Out of range");
//!
//! assert_eq!(
//!     "一百三十万五千四百八十兆五千二百七十一万\
//!     五千六百三十七亿零五百九十七万二千九百六十四",
//!     num.to_lowercase_simp()
//! );
//! ```

mod characters;
mod longscale;
mod macros;
mod midscale;
mod myriadscale;
mod shortscale;

use characters::NumChar;
pub use longscale::LongScaleInt;
pub use midscale::MidScaleInt;
pub use myriadscale::MyriadScaleInt;
pub use shortscale::ShortScaleInt;

#[cfg(feature = "bigint")]
use num_bigint::BigUint;

#[cfg(feature = "bigint")]
pub use longscale::LongScaleBigInt;
#[cfg(feature = "bigint")]
pub use midscale::MidScaleBigInt;
#[cfg(feature = "bigint")]
pub use myriadscale::MyriadScaleBigInt;

/// The sign of the number.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Sign {
    /// Negative.
    Neg,
    /// No sign, zero.
    Nil,
    /// Positive.
    Pos,
}

/// Chinese variants.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Variant {
    /// Simplified Chinese. Used in China, Singapore, and Malaysia.
    Simplified,
    /// Traditional Chinese. Used in Taiwan (Province of China), Hong Kong, and Macau.
    Traditional,
}

impl Default for Sign {
    fn default() -> Self {
        Self::Nil
    }
}

/// Out of range errors.
#[cfg(feature = "bigint")]
#[derive(Debug)]
pub enum Error {
    ShortScaleOutOfRange(u128),
    MyriadScaleOutOfRange(BigUint),
    MidScaleOutOfRange(BigUint),
    LongScaleOutOfRange(BigUint),
}

#[cfg(feature = "bigint")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ShortScaleOutOfRange(value) => write!(
                f,
                "Absolute value {value} out of range for a short scale number"
            ),
            Error::MyriadScaleOutOfRange(value) => write!(
                f,
                "Absolute value {value} out of range for a myriad scale number"
            ),
            Error::MidScaleOutOfRange(value) => write!(
                f,
                "Absolute value {value} out of range for a mid-scale number"
            ),
            Error::LongScaleOutOfRange(value) => write!(
                f,
                "Absolute value {value} out of range for a long scale number"
            ),
        }
    }
}

#[cfg(not(feature = "bigint"))]
#[derive(Debug)]
pub enum Error {
    ShortScaleOutOfRange(u128),
}

#[cfg(not(feature = "bigint"))]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ShortScaleOutOfRange(value) => write!(
                f,
                "Absolute value {value} out of range for a short scale number"
            ),
        }
    }
}

impl std::error::Error for Error {}

pub(crate) mod sealed {
    pub trait SignedInteger {
        type Data;

        fn sign(&self) -> crate::Sign;
        fn data(&self) -> &Self::Data;
    }

    pub trait ChineseNumeralBase: SignedInteger {
        fn to_chars(&self) -> Vec<crate::characters::NumChar>;
        fn to_chars_trimmed(&self) -> Vec<crate::characters::NumChar>;
    }
}

/// Provides methods to generate Chinease numeral expression for a number.
pub trait ChineseNumeral: sealed::ChineseNumeralBase {
    /// Converts the number to lowercase (小写数字, used for normal contexts).
    fn to_lowercase(&self, variant: Variant) -> String {
        let method = match variant {
            Variant::Simplified => NumChar::to_lowercase_simp,
            Variant::Traditional => NumChar::to_lowercase_trad,
        };
        let mut chars = self.to_chars_trimmed();
        match self.sign() {
            Sign::Neg => chars.push(NumChar::Neg),
            Sign::Nil => chars.push(NumChar::Zero),
            _ => {}
        }
        chars.into_iter().rev().map(method).collect()
    }

    /// Converts the number to lowercase (小写数字, used for normal contexts) in simplified Chinese.
    fn to_lowercase_simp(&self) -> String {
        self.to_lowercase(Variant::Simplified)
    }

    /// Converts the number to lowercase (小写数字, used for normal contexts) in traditional Chinese.
    fn to_lowercase_trad(&self) -> String {
        self.to_lowercase(Variant::Traditional)
    }

    /// Converts the number to uppercase (大写数字, used for financial contexts).
    fn to_uppercase(&self, variant: Variant) -> String {
        let method = match variant {
            Variant::Simplified => NumChar::to_uppercase_simp,
            Variant::Traditional => NumChar::to_uppercase_trad,
        };
        let mut chars = self.to_chars();
        match self.sign() {
            Sign::Neg => chars.push(NumChar::Neg),
            Sign::Nil => chars.push(NumChar::Zero),
            _ => {}
        }
        chars.into_iter().rev().map(method).collect()
    }

    /// Converts the number to uppercase (大写数字, used for financial contexts) in simplified Chinese.
    fn to_uppercase_simp(&self) -> String {
        self.to_uppercase(Variant::Simplified)
    }

    /// Converts the number to uppercase (大写数字, used for financial contexts) in traditional Chinese.
    fn to_uppercase_trad(&self) -> String {
        self.to_uppercase(Variant::Traditional)
    }
}

impl<T> ChineseNumeral for T where T: sealed::ChineseNumeralBase {}
