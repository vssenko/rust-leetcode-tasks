use std::fmt;
use std::ops::{Add, Deref};

#[derive(Debug, Clone, Copy)]
struct Sum<T>(T);

impl<T> Sum<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}

// Here we are just fighting orpan rule, it could be much easier if Rust just allowed us...
// Or we could just not use generic approach, but that's lame.
macro_rules! impl_from_sum {
    ($($t:ty),*) => {
        $(
            impl From<Sum<$t>> for $t {
                fn from(value: Sum<$t>) -> $t {
                    value.0
                }
            }
        )*
    };
}
impl_from_sum!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl<T> Deref for Sum<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FnOnce<(T,)> for Sum<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Sum<T>;

    extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        Sum(self.0 + args.0)
    }
}

impl<T> fmt::Display for Sum<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dynamic_carrying_1() {
        let sum = Sum::new(5 as usize)(6)(7)(8);

        // dynamic conversion to string
        assert_eq!(format!("{}", sum), "26");

        // deref way conversion, why not
        assert_eq!(*sum, 26);

        // convert to result in Rust-conventional .into way
        let result: usize = sum.into();
        assert_eq!(result, 26);
    }

    #[test]
    fn dynamic_carrying_2() {
        let sum = Sum::new(1);
        assert_eq!(*sum, 1);
    }

    #[test]
    fn dynamic_carrying_3() {
        let sum = Sum::new(1.5)(2.2);
        assert_eq!(*sum, 3.7);
    }
}
