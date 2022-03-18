# Chinese Numerals

Converts primitive integers and [big integers](https://docs.rs/num-bigint) to [Chinese numerals](https://en.wikipedia.org/wiki/Chinese_numerals).

According to 《[五经算术](https://zh.wikipedia.org/wiki/%E4%BA%94%E7%B6%93%E7%AE%97%E8%A1%93)》, for representing numbers larger than 1,0000, there have been ten names (亿, 兆, 京, 垓, 秭, 壤, 沟, 涧, 正, and 载) and three systems (下数 short scale, 中数 mid-scale, 上数 long scale). Plus the myriad scale, in which each name represents a number 1,0000 times the previous, this crate can convert integers to four scales.

## Usage

Add to `Cargo.toml`:
```toml
[dependencies]
chinese-numerals = "0.2"
```

All structs have implemented [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait's normal (with `"{}"`) and alternative (with `"{:#}"`) formats, converting to lowercase and uppercase Chinese numbers. Besides, [`ChineseNumeral`](https://docs.rs/chinese-numerals/latest/chinese_numerals/trait.ChineseNumeral.html) trait provides following functions:

- [`to_lowercase`](https://docs.rs/chinese-numerals/latest/chinese_numerals/trait.ChineseNumeral.html#method.to_lowercase)
- [`to_lowercase_simp`](https://docs.rs/chinese-numerals/latest/chinese_numerals/trait.ChineseNumeral.html#method.to_lowercase#method.to_lowercase_simp)
- [`to_lowercase_trad`](https://docs.rs/chinese-numerals/latest/chinese_numerals/trait.ChineseNumeral.html#method.to_lowercase#method.to_lowercase_trad)
- [`to_uppercase`](https://docs.rs/chinese-numerals/latest/chinese_numerals/trait.ChineseNumeral.html#method.to_lowercase#method.to_uppercase)
- [`to_uppercase_simp`](https://docs.rs/chinese-numerals/latest/chinese_numerals/trait.ChineseNumeral.html#method.to_lowercase#method.to_uppercase_simp)
- [`to_uppercase_trad`](https://docs.rs/chinese-numerals/latest/chinese_numerals/trait.ChineseNumeral.html#method.to_lowercase#method.to_uppercase_trad)

## Premitive Integers

For each scale, a struct has been implemented to perform the convertion.

[`ShortScaleInt`](https://docs.rs/chinese-numerals/latest/chinese_numerals/struct.ShortScaleInt.html) has implemented [`From`](https://doc.rust-lang.org/core/convert/trait.From.html) trait for `i8`, `u8`, `i16`, `u16`, `i32`, and `u32`, and [`TryFrom`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html) trait for `i64`, `u64`, `i128`, `u128`, `isize`, and `usize`.

[`MyriadScaleInt`](https://docs.rs/chinese-numerals/latest/chinese_numerals/struct.MyriadScaleInt.html), [`MidScaleInt`](https://docs.rs/chinese-numerals/latest/chinese_numerals/struct.MidScaleInt.html), and [`LongScaleInt`](https://docs.rs/chinese-numerals/latest/chinese_numerals/struct.LongScaleInt.html) have implemented `From` trait for all premitive integers.

### Examples
```
use chinese_numerals::{ChineseNumeral, ShortScaleInt, MidScaleInt};

let num = ShortScaleInt::from(1_0203_0405);
assert_eq!("一垓零二兆零三万零四百零五", format!("{}", num));
assert_eq!("壹垓零贰兆零叁万零肆佰零伍", format!("{:#}", num));

let num = MidScaleInt::from(1_0203_0405);
assert_eq!("一億零二百零三萬零四百零五", num.to_lowercase_trad());
assert_eq!("壹億零貳佰零叄萬零肆佰零伍", num.to_uppercase_trad());
```

## Big Integers

For scales except short scale, a struct has been implemented to perform the convertion from [`BigInt`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html) and [`BigUint`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigUint.html).

[`MyriadScaleBigInt`](https://docs.rs/chinese-numerals/latest/chinese_numerals/struct.MyriadScaleBigInt.html), [`MidScaleBigInt`](https://docs.rs/chinese-numerals/latest/chinese_numerals/struct.MidScaleBigInt.html), and [`LongScaleBigInt`](https://docs.rs/chinese-numerals/latest/chinese_numerals/struct.LongScaleBigInt.html) have implemented `TryFrom` trait for both `BigInt` and `BigUint`.

### Dependencies

To enable `bigint` feature, set dependencies in `Cargo.toml`:
```toml
[dependencies]
num-bigint = "0.4"
chinese-numerals = { version = "0.2", features = ["bigint"] }
```

### Examples
```
use chinese_numerals::{ChineseNumeral, LongScaleBigInt};
use num_bigint::BigUint;

// 130_5480_5271_5637_0597_2964
let num = BigUint::new(vec![463665380, 3016835882, 707]);
let num = LongScaleBigInt::try_from(num).expect("Out of range");

assert_eq!(
    "一百三十万五千四百八十兆五千二百七十一万\
    五千六百三十七亿零五百九十七万二千九百六十四",
    num.to_lowercase_simp()
);
```
