pub struct ModInt {
    value: i64,
}

impl ModInt {
    // version 1.5でconst genericsに移せる
    // https://qiita.com/namn1125/items/5100cb85021a1d6e8f6c
    const MOD: i64 = 1000_000_007;

    fn new(value: i64) -> Self {
        let v = if value < 0 {
            Self::MOD - ((-value) % Self::MOD)
        } else {
            value
        };

        let v = if v >= Self::MOD {
            v % Self::MOD
        } else {
            v
        };

        ModInt {
            value: v
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_int_works() {
        let a = ModInt::new(10);
        assert_eq!(a.value, 10);
        let b = ModInt::new(-1);
        assert_eq!(b.value, ModInt::MOD - 1);
    }
}