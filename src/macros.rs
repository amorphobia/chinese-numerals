use crate::{
    ChineseNumeral, Error, LongScaleInt, MidScaleInt, MyriadScaleInt, ShortScaleInt, Sign,
};

#[cfg(feature = "bigint")]
use crate::{LongScaleBigInt, MidScaleBigInt, MyriadScaleBigInt};
#[cfg(feature = "bigint")]
use num_bigint::{BigInt, BigUint};
#[cfg(feature = "bigint")]
use num_traits::{Signed, Zero};

macro_rules! impl_signed_int {
    ($($int:ident, $data:ty),+ $(,)?) => {
        $(impl crate::sealed::SignedInteger for $int {
            type Data = $data;

            fn sign(&self) -> Sign {
                self.sign
            }

            fn data(&self) -> &Self::Data {
                &self.data
            }
        })+
    };
}

impl_signed_int! {ShortScaleInt, u64, MyriadScaleInt, u128, MidScaleInt, u128, LongScaleInt, u128}

#[cfg(feature = "bigint")]
impl_signed_int! {MyriadScaleBigInt, BigUint, MidScaleBigInt, BigUint, LongScaleBigInt, BigUint}

macro_rules! impl_disp {
    ($($int:ident),+ $(,)?) => {
        $(impl std::fmt::Display for $int {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    write!(f, "{}", self.to_uppercase_simp())
                } else {
                    write!(f, "{}", self.to_lowercase_simp())
                }
            }
        })+
    };
}

impl_disp! {ShortScaleInt, MyriadScaleInt, MidScaleInt, LongScaleInt}

#[cfg(feature = "bigint")]
impl_disp! {MyriadScaleBigInt, MidScaleBigInt, LongScaleBigInt}

macro_rules! impl_try_from_uint {
    ($($u:ty),+ $(,)?) => {
        $(impl TryFrom<$u> for ShortScaleInt {
            type Error = Error;

            /// Performs the conversion. Returns [`Error`] if the absolute value is out of range.
            fn try_from(value: $u) -> Result<Self, Self::Error> {
                if value == 0 {
                    Ok(Self::default())
                } else if value <= Self::MAX_ABS as $u {
                    Ok(Self {
                        sign: Sign::Pos,
                        data: value as u64,
                    })
                } else {
                    Err(Error::ShortScaleOutOfRange(value as u128))
                }
            }
        })+
    };
}

impl_try_from_uint! {u64, u128, usize}

macro_rules! impl_try_from_int {
    ($($i:ty),+ $(,)?) => {
        $(impl TryFrom<$i> for ShortScaleInt {
            type Error = Error;

            /// Performs the conversion. Returns [`Error`] if the absolute value is out of range.
            fn try_from(value: $i) -> Result<Self, Self::Error> {
                if value < -(Self::MAX_ABS as $i) || value > Self::MAX_ABS as $i {
                    let abs = if value as i128 == i128::MIN {
                        (i128::MAX as u128) + 1
                    } else {
                        value.abs() as u128
                    };
                    Err(Error::ShortScaleOutOfRange(abs))
                } else if value.is_negative() {
                    Ok(Self {
                        sign: Sign::Neg,
                        data: value.abs() as u64,
                    })
                } else if value.is_positive() {
                    Ok(Self {
                        sign: Sign::Pos,
                        data: value as u64,
                    })
                } else {
                    Ok(Self::default())
                }
            }
        })+
    };
}

impl_try_from_int! {i64, i128, isize}

macro_rules! impl_from_int {
    ($num:ident, $data:ty, $($pre:ty),+ $(,)?) => {
        $(impl From<$pre> for $num {
            fn from(value: $pre) -> Self {
                if value == 0 {
                    Self::default()
                } else if value < 0 {
                    let abs = if value == <$pre>::MIN {
                        (<$pre>::MAX as $data) + 1
                    } else {
                        value.abs() as $data
                    };
                    Self {
                        sign: Sign::Neg,
                        data: abs,
                    }
                } else {
                    Self {
                        sign: Sign::Pos,
                        data: value as $data,
                    }
                }
            }
        })+
    };
}

impl_from_int! {ShortScaleInt, u64, i8, i16, i32}
impl_from_int! {MyriadScaleInt, u128, i8, i16, i32, i64, i128, isize}
impl_from_int! {MidScaleInt, u128, i8, i16, i32, i64, i128, isize}
impl_from_int! {LongScaleInt, u128, i8, i16, i32, i64, i128, isize}

macro_rules! impl_from_uint {
    ($num:ident, $data:ty, $($pre:ty),+ $(,)?) => {
        $(impl From<$pre> for $num {
            fn from(value: $pre) -> Self {
                if value == 0 {
                    Self::default()
                } else {
                    Self {
                        sign: Sign::Pos,
                        data: value as $data,
                    }
                }
            }
        })+
    };
}

impl_from_uint! {ShortScaleInt, u64, u8, u16, u32}
impl_from_uint! {MyriadScaleInt, u128, u8, u16, u32, u64, u128, usize}
impl_from_uint! {MidScaleInt, u128, u8, u16, u32, u64, u128, usize}
impl_from_uint! {LongScaleInt, u128, u8, u16, u32, u64, u128, usize}

#[cfg(feature = "bigint")]
macro_rules! impl_try_from_big {
    ($($int:ty, $err:ident),+ $(,)?) => {
        $(
            impl TryFrom<&BigUint> for $int {
                type Error = Error;

                /// Performs the conversion. Returns [`Error`] if the absolute value is out of range.
                fn try_from(value: &BigUint) -> Result<Self, Self::Error> {
                    if value == &BigUint::zero() {
                        Ok(Self::default())
                    } else if value <= &BigUint::from_slice(Self::MAX_ABS_ARR) {
                        Ok(Self {
                            sign: Sign::Pos,
                            data: value.to_owned(),
                        })
                    } else {
                        Err(Error::$err(value.to_owned()))
                    }
                }
            }

            impl TryFrom<BigUint> for $int {
                type Error = Error;

                /// Performs the conversion. Returns [`Error`] if the absolute value is out of range.
                fn try_from(value: BigUint) -> Result<Self, Self::Error> {
                    <$int>::try_from(&value)
                }
            }

            impl TryFrom<&BigInt> for $int {
                type Error = Error;

                /// Performs the conversion. Returns [`Error`] if the absolute value is out of range.
                fn try_from(value: &BigInt) -> Result<Self, Self::Error> {
                    if value < &BigInt::from_slice(num_bigint::Sign::Minus, Self::MAX_ABS_ARR) || value > &BigInt::from_slice(num_bigint::Sign::Plus, Self::MAX_ABS_ARR) {
                        let abs = value.abs().to_biguint().unwrap();
                        Err(Error::$err(abs))
                    } else if value == &BigInt::zero() {
                        Ok(Self::default())
                    } else if value < &BigInt::zero() {
                        Ok(Self {
                            sign: Sign::Neg,
                            data: value.abs().to_biguint().unwrap(),
                        })
                    } else {
                        Ok(Self {
                            sign: Sign::Pos,
                            data: value.to_biguint().unwrap(),
                        })
                    }
                }
            }

            impl TryFrom<BigInt> for $int {
                type Error = Error;

                /// Performs the conversion. Returns [`Error`] if the absolute value is out of range.
                fn try_from(value: BigInt) -> Result<Self, Self::Error> {
                    <$int>::try_from(&value)
                }
            }
        )+
    };
}

#[cfg(feature = "bigint")]
impl_try_from_big! {
    MyriadScaleBigInt, MyriadScaleOutOfRange,
    MidScaleBigInt, MidScaleOutOfRange,
    LongScaleBigInt, LongScaleOutOfRange,
}
