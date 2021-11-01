use std::ops;

pub mod mod_int {
    type ModInternalType = i64;

    pub struct ModInt {
        pub value: ModInternalType,
    }

    impl ModInt {
        // version 1.5でconst genericsに移せる
        // https://qiita.com/namn1125/items/5100cb85021a1d6e8f6c
        pub const MOD: ModInternalType = 1000_000_007;

        pub fn new<T: ToInternalNum>(value: T) -> Self {
            let value = value.to_internal_num();
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

    // 整数型をModIntとの演算に使用できるようにするためのトレイト
    pub trait ToInternalNum {
        fn to_internal_num(&self) -> ModInternalType;
    }

    impl ToInternalNum for ModInt {
        fn to_internal_num(&self) -> ModInternalType {
            self.value
        }
    }

    macro_rules! impl_primitive {
        ($primitive:ident)=>{
            impl ToInternalNum for $primitive{
                fn to_internal_num(&self) ->ModInternalType{
                    *self as ModInternalType
                }
            }
        }
    }

    impl_primitive!(i32);

    // 四則演算
    impl<T: ToInternalNum> std::ops::AddAssign<T> for ModInt {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let modulo = ModInt::MOD;
            if rhs >= modulo {
                rhs %= modulo;
            }
            self.value += rhs;
            if self.value >= modulo {
                self.value -= modulo;
            }
        }
    }

    impl<T: ToInternalNum> std::ops::Add<T> for ModInt {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let mut res = self;
            res += other;
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_int_works() {
        let mut a = mod_int::ModInt::new(10);
        assert_eq!(a.value, 10);
        let mut b = mod_int::ModInt::new(-1);
        assert_eq!(b.value, mod_int::ModInt::MOD - 1);
        assert_eq!((a + b).value, 9)
    }
}