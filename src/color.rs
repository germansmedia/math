// Math
// by Desmond Germans

use std::{cmp,fmt,ops};
use crate::*;

#[derive(Copy,Clone)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

macro_rules! impl_rgb (
    ($t:ty) => (
        impl RGB<$t> {
            pub fn new(r: $t,g: $t,b: $t) -> RGB<$t> {
                RGB {
                    r: r,
                    g: g,
                    b: b,
                }
            }
        }

        impl cmp::PartialEq for RGB<$t> {
            fn eq(&self,other: &RGB<$t>) -> bool {
                (self.r == other.r)
                && (self.g == other.g)
                && (self.b == other.b)
            }
        }

        impl fmt::Display for RGB<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.r,self.g,self.b)
            }
        }

        impl fmt::Debug for RGB<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.r,self.g,self.b)
            }
        }

        impl Zero for RGB<$t> {
            fn zero() -> RGB<$t> {
                RGB {
                    r: <$t>::zero(),
                    g: <$t>::zero(),
                    b: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for RGB<$t> {
            type Output = RGB<$t>;
            fn neg(self) -> Self::Output {
                RGB {
                    r: -self.r,
                    g: -self.g,
                    b: -self.b,
                }
            }
        }

        impl ops::Add<RGB<$t>> for RGB<$t> {
            type Output = RGB<$t>;
            fn add(self,other: RGB<$t>) -> Self::Output {
                RGB {
                    r: self.r + other.r,
                    g: self.g + other.g,
                    b: self.b + other.b,
                }
            }
        }

        impl ops::Sub<RGB<$t>> for RGB<$t> {
            type Output = RGB<$t>;
            fn sub(self,other: RGB<$t>) -> RGB<$t> {
                RGB {
                    r: self.r - other.r,
                    g: self.g - other.g,
                    b: self.b - other.b,
                }
            }
        }

        impl ops::AddAssign<RGB<$t>> for RGB<$t> {
            fn add_assign(&mut self,other: RGB<$t>) {
                self.r += other.r;
                self.g += other.g;
                self.b += other.b;
            }
        }

        impl ops::SubAssign<RGB<$t>> for RGB<$t> {
            fn sub_assign(&mut self,other: RGB<$t>) {
                self.r -= other.r;
                self.g -= other.g;
                self.b -= other.b;
            }
        }

        impl ops::Mul<$t> for RGB<$t> {
            type Output = RGB<$t>;
            fn mul(self,other: $t) -> Self::Output {
                RGB {
                    r: self.r * other,
                    g: self.g * other,
                    b: self.b * other,
                }
            }
        }
                    

        impl ops::Mul<RGB<$t>> for $t {
            type Output = RGB<$t>;
            fn mul(self,other: RGB<$t>) -> Self::Output {
                RGB {
                    r: self * other.r,
                    g: self * other.g,
                    b: self * other.b,
                }
            }
        }

        impl ops::Mul<RGB<$t>> for RGB<$t> {
            type Output = RGB<$t>;
            fn mul(self,other: RGB<$t>) -> Self::Output {
                RGB {
                    r: self.r * other.r,
                    g: self.g * other.g,
                    b: self.b * other.b,
                }
            }
        }

        impl ops::MulAssign<$t> for RGB<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.r *= other;
                self.g *= other;
                self.b *= other;
            }
        }

        impl ops::MulAssign<RGB<$t>> for RGB<$t> {
            fn mul_assign(&mut self,other: RGB<$t>) {
                self.r *= other.r;
                self.g *= other.g;
                self.b *= other.b;
            }
        }

        impl ops::Div<$t> for RGB<$t> {
            type Output = RGB<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    RGB {
                        r: self.r / other,
                        g: self.g / other,
                        b: self.b / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl ops::DivAssign<$t> for RGB<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.r /= other;
                    self.g /= other;
                    self.b /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32rgb = RGB<f32>;

impl_rgb!(f32);

#[allow(non_camel_case_types)]
pub type f64rgb = RGB<f64>;

impl_rgb!(f64);

#[derive(Copy,Clone)]
pub struct RGBA<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

macro_rules! impl_rgba (
    ($t:ty) => (
        impl RGBA<$t> {
            pub fn new(r: $t,g: $t,b: $t,a: $t) -> RGBA<$t> {
                RGBA {
                    r: r,
                    g: g,
                    b: b,
                    a: a,
                }
            }
        }

        impl cmp::PartialEq for RGBA<$t> {
            fn eq(&self,other: &RGBA<$t>) -> bool {
                (self.r == other.r)
                && (self.g == other.g)
                && (self.b == other.b)
                && (self.a == other.a)
            }
        }

        impl fmt::Display for RGBA<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{},{})",self.r,self.g,self.b,self.a)
            }
        }

        impl fmt::Debug for RGBA<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{},{})",self.r,self.g,self.b,self.a)
            }
        }

        impl Zero for RGBA<$t> {
            fn zero() -> RGBA<$t> {
                RGBA {
                    r: <$t>::zero(),
                    g: <$t>::zero(),
                    b: <$t>::zero(),
                    a: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for RGBA<$t> {
            type Output = RGBA<$t>;
            fn neg(self) -> Self::Output {
                RGBA {
                    r: -self.r,
                    g: -self.g,
                    b: -self.b,
                    a: -self.a,
                }
            }
        }

        impl ops::Add<RGBA<$t>> for RGBA<$t> {
            type Output = RGBA<$t>;
            fn add(self,other: RGBA<$t>) -> Self::Output {
                RGBA {
                    r: self.r + other.r,
                    g: self.g + other.g,
                    b: self.b + other.b,
                    a: self.a + other.a,
                }
            }
        }

        impl ops::Sub<RGBA<$t>> for RGBA<$t> {
            type Output = RGBA<$t>;
            fn sub(self,other: RGBA<$t>) -> RGBA<$t> {
                RGBA {
                    r: self.r - other.r,
                    g: self.g - other.g,
                    b: self.b - other.b,
                    a: self.a - other.a,
                }
            }
        }

        impl ops::AddAssign<RGBA<$t>> for RGBA<$t> {
            fn add_assign(&mut self,other: RGBA<$t>) {
                self.r += other.r;
                self.g += other.g;
                self.b += other.b;
                self.a += other.a;
            }
        }

        impl ops::SubAssign<RGBA<$t>> for RGBA<$t> {
            fn sub_assign(&mut self,other: RGBA<$t>) {
                self.r -= other.r;
                self.g -= other.g;
                self.b -= other.b;
                self.a -= other.a;
            }
        }

        impl ops::Mul<$t> for RGBA<$t> {
            type Output = RGBA<$t>;
            fn mul(self,other: $t) -> Self::Output {
                RGBA {
                    r: self.r * other,
                    g: self.g * other,
                    b: self.b * other,
                    a: self.a * other,
                }
            }
        }
                    

        impl ops::Mul<RGBA<$t>> for $t {
            type Output = RGBA<$t>;
            fn mul(self,other: RGBA<$t>) -> Self::Output {
                RGBA {
                    r: self * other.r,
                    g: self * other.g,
                    b: self * other.b,
                    a: self * other.a,
                }
            }
        }

        impl ops::Mul<RGBA<$t>> for RGBA<$t> {
            type Output = RGBA<$t>;
            fn mul(self,other: RGBA<$t>) -> Self::Output {
                RGBA {
                    r: self.r * other.r,
                    g: self.g * other.g,
                    b: self.b * other.b,
                    a: self.a * other.a,
                }
            }
        }

        impl ops::MulAssign<$t> for RGBA<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.r *= other;
                self.g *= other;
                self.b *= other;
                self.a *= other;
            }
        }

        impl ops::MulAssign<RGBA<$t>> for RGBA<$t> {
            fn mul_assign(&mut self,other: RGBA<$t>) {
                self.r *= other.r;
                self.g *= other.g;
                self.b *= other.b;
                self.a *= other.a;
            }
        }

        impl ops::Div<$t> for RGBA<$t> {
            type Output = RGBA<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    RGBA {
                        r: self.r / other,
                        g: self.g / other,
                        b: self.b / other,
                        a: self.a / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl ops::DivAssign<$t> for RGBA<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.r /= other;
                    self.g /= other;
                    self.b /= other;
                    self.a /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32rgba = RGBA<f32>;

impl_rgba!(f32);

#[allow(non_camel_case_types)]
pub type f64rgba = RGBA<f64>;

impl_rgba!(f64);
