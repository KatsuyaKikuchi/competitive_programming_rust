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

        pub fn pow<T: ToInternalNum>(&self, e: T) -> Self {
            let mut res = 1i64;
            let mut p = self.value;
            let mut e = e.to_internal_num();
            let modulo = ModInt::MOD;
            while e > 0 {
                if (e & 1) == 1 {
                    res = (res * p) % modulo;
                }
                e >>= 1;
                p = (p * p) % modulo;
            }
            Self::new(res)
        }
    }

    impl Clone for ModInt {
        fn clone(&self) -> Self {
            Self::new(self.value)
        }
    }

    impl Copy for ModInt {}

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
    impl_primitive!(u32);
    impl_primitive!(i64);
    impl_primitive!(u64);

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
        fn add(self, rhs: T) -> Self {
            let mut res = self;
            res += rhs;
            res
        }
    }

    impl<T: ToInternalNum> std::ops::SubAssign<T> for ModInt {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let modulo = ModInt::MOD;
            if rhs >= modulo {
                rhs %= modulo;
            }
            if rhs > 0 {
                self.value += modulo - rhs;
            }
            if self.value >= modulo {
                self.value -= modulo;
            }
        }
    }

    impl<T: ToInternalNum> std::ops::Sub<T> for ModInt {
        type Output = Self;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }

    impl<T: ToInternalNum> std::ops::MulAssign<T> for ModInt {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let modulo = ModInt::MOD;
            if rhs >= modulo {
                rhs %= modulo;
            }
            self.value *= rhs;
            if self.value >= modulo {
                self.value %= modulo;
            }
        }
    }

    impl<T: ToInternalNum> std::ops::Mul<T> for ModInt {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<T: ToInternalNum> std::ops::DivAssign<T> for ModInt {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let modulo = ModInt::MOD;
            if rhs >= modulo {
                rhs %= modulo;
            }
            let inv = Self::new(rhs).pow(modulo - 2);
            self.value *= inv.value;
            if self.value >= modulo {
                self.value %= modulo;
            }
        }
    }

    impl<T: ToInternalNum> std::ops::Div<T> for ModInt {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::mod_int::mod_int::ModInt;
    use super::*;

    #[test]
    fn mod_int_works() {
        let mut a = mod_int::ModInt::new(10);
        assert_eq!(a.value, 10);
        let b = mod_int::ModInt::new(-1);
        assert_eq!(b.value, mod_int::ModInt::MOD - 1);
        assert_eq!((a + b).value, 9);
        a += 30;
        assert_eq!(a.value, 40);
        a += b;
        assert_eq!(a.value, 39);
        a -= b;
        assert_eq!(a.value, 40);
        a -= 30;
        assert_eq!(a.value, 10);
        a *= 10;
        assert_eq!(a.value, 100);
        a *= b;
        assert_eq!(a.value, (100 * (ModInt::MOD - 1)) % ModInt::MOD);
        a /= b;
        assert_eq!(a.value, 100);
        a /= 10;
        assert_eq!(a.value, 10);

        let c = a.pow(2);
        assert_eq!(c.value, 100);
        let c = a.pow(12);
        assert_eq!(c.value, 1000000000000i64 % ModInt::MOD);
    }
}