// Math
// by Desmond Germans

use std::{cmp,fmt,ops};
use crate::*;

pub const TAU: f32 = 6.28318531;

#[derive(Copy,Clone)]
pub struct YPB<T> {
    pub y: T,
    pub p: T,
    pub b: T,
}

macro_rules! impl_ypb (
    ($t:ty) => (
        impl YPB<$t> {
            pub fn new(y: $t,p: $t,b: $t) -> YPB<$t> {
                YPB {
                    y: y,
                    p: p,
                    b: b,
                }
            }
        }

        impl cmp::PartialEq for YPB<$t> {
            fn eq(&self,other: &YPB<$t>) -> bool {
                (self.y == other.y)
                && (self.p == other.p)
                && (self.b == other.b)
            }
        }

        impl fmt::Display for YPB<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.y,self.p,self.b)
            }
        }

        impl fmt::Debug for YPB<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.y,self.p,self.b)
            }
        }

        impl Zero for YPB<$t> {
            fn zero() -> YPB<$t> {
                YPB {
                    y: <$t>::zero(),
                    p: <$t>::zero(),
                    b: <$t>::zero(),
                }
            }
        }

        impl ops::Add<YPB<$t>> for YPB<$t> {
            type Output = YPB<$t>;
            fn add(self,other: YPB<$t>) -> Self::Output {
                YPB {
                    y: self.y + other.y,
                    p: self.p + other.p,
                    b: self.b + other.b,
                }
            }
        }

        impl ops::Sub<YPB<$t>> for YPB<$t> {
            type Output = YPB<$t>;
            fn sub(self,other: YPB<$t>) -> Self::Output {
                YPB {
                    y: self.y - other.y,
                    p: self.p - other.p,
                    b: self.b - other.b,
                }
            }
        }

        impl ops::AddAssign<YPB<$t>> for YPB<$t> {
            fn add_assign(&mut self,other: YPB<$t>) {
                self.y += other.y;
                self.p += other.p;
                self.b += other.b;
            }
        }

        impl ops::SubAssign<YPB<$t>> for YPB<$t> {
            fn sub_assign(&mut self,other: YPB<$t>) {
                self.y -= other.y;
                self.p -= other.p;
                self.b -= other.b;
            }
        }

        impl ops::Mul<YPB<$t>> for $t {
            type Output = YPB<$t>;
            fn mul(self,other: YPB<$t>) -> Self::Output {
                YPB {
                    y: self * other.y,
                    p: self * other.p,
                    b: self * other.b,
                }
            }
        }

        impl ops::Mul<$t> for YPB<$t> {
            type Output = YPB<$t>;
            fn mul(self,other: $t) -> Self::Output {
                YPB {
                    y: self.y * other,
                    p: self.p * other,
                    b: self.b * other,
                }
            }
        }

        impl ops::MulAssign<$t> for YPB<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.y *= other;
                self.p *= other;
                self.b *= other;
            }
        }

        impl ops::Div<$t> for YPB<$t> {
            type Output = YPB<$t>;
            fn div(self,other: $t) -> Self::Output {
                YPB {
                    y: self.y / other,
                    p: self.p / other,
                    b: self.b / other,
                }
            }
        }

        impl ops::DivAssign<$t> for YPB<$t> {
            fn div_assign(&mut self,other: $t) {
                self.y /= other;
                self.p /= other;
                self.b /= other;
            }
        }

        impl ops::Neg for YPB<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                YPB {
                    y: -self.y,
                    p: -self.p,
                    b: -self.b,
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32ypb = YPB<f32>;

impl_ypb!(f32);

#[allow(non_camel_case_types)]
pub type f64ypb = YPB<f64>;

impl_ypb!(f64);
