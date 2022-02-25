# Chinese Numerals

Convert numbers to Chinese numerals.

## Examples
```rust
extern cate chinese_numerals;

use chinese_numerals::ShortScaleInt;

let num = ShortScaleInt::from(1_0203_0405);
// lowercase
assert_eq!("一垓零二兆零三万零四百零五", format!("{}", num));
// uppercase
assert_eq!("壹垓零贰兆零叁万零肆佰零伍", format!("{:#}", num));
```

## Bigint
Turn on `bigint` feature to convert numbers with absolute values larger than `u128::MAX`.

```toml
[dependencies]
num-bigint = "0.4" 
chinese-numerals = "0.1"
```
