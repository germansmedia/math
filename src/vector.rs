// Math
// by Desmond Germans

use std::{cmp,fmt,ops};
use crate::*;

#[derive(Copy,Clone)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

macro_rules! impl_xy (
    ($t:ty) => (
        impl XY<$t> {
            pub fn new(x: $t,y: $t) -> XY<$t> {
                XY {
                    x: x,
                    y: y,
                }
            }
        }

        impl cmp::PartialEq for XY<$t> {
            fn eq(&self,other: &XY<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
            }
        }

        impl fmt::Display for XY<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{})",self.x,self.y)
            }
        }

        impl fmt::Debug for XY<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{})",self.x,self.y)
            }
        }

        impl Zero for XY<$t> {
            fn zero() -> XY<$t> {
                XY {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                }
            }
        }

        impl ops::Add<XY<$t>> for XY<$t> {
            type Output = XY<$t>;
            fn add(self,other: XY<$t>) -> Self::Output {
                XY {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl ops::Sub<XY<$t>> for XY<$t> {
            type Output = XY<$t>;
            fn sub(self,other: XY<$t>) -> Self::Output {
                XY {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        impl ops::AddAssign<XY<$t>> for XY<$t> {
            fn add_assign(&mut self,other: XY<$t>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        impl ops::SubAssign<XY<$t>> for XY<$t> {
            fn sub_assign(&mut self,other: XY<$t>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        impl ops::Mul<XY<$t>> for $t {
            type Output = XY<$t>;
            fn mul(self,other: XY<$t>) -> Self::Output {
                XY {
                    x: self * other.x,
                    y: self * other.y,
                }
            }
        }

        impl ops::Mul<$t> for XY<$t> {
            type Output = XY<$t>;
            fn mul(self,other: $t) -> Self::Output {
                XY {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }

        impl ops::MulAssign<$t> for XY<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
            }
        }

        impl ops::Div<$t> for XY<$t> {
            type Output = XY<$t>;
            fn div(self,other: $t) -> Self::Output {
                XY {
                    x: self.x / other,
                    y: self.y / other,
                }
            }
        }

        impl ops::DivAssign<$t> for XY<$t> {
            fn div_assign(&mut self,other: $t) {
                self.x /= other;
                self.y /= other;
            }
        }
    );
);

macro_rules! impl_xy_neg (
    ($t:ty) => (
        impl ops::Neg for XY<$t> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                XY {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type u8xy = XY<u8>;

impl_xy!(u8);

#[allow(non_camel_case_types)]
pub type i8xy = XY<i8>;

impl_xy!(i8);
impl_xy_neg!(i8);

#[allow(non_camel_case_types)]
pub type u16xy = XY<u16>;

impl_xy!(u16);

#[allow(non_camel_case_types)]
pub type i16xy = XY<i16>;

impl_xy!(i16);
impl_xy_neg!(i16);

#[allow(non_camel_case_types)]
pub type u32xy = XY<u32>;

impl_xy!(u32);

#[allow(non_camel_case_types)]
pub type i32xy = XY<i32>;

impl_xy!(i32);
impl_xy_neg!(i32);

#[allow(non_camel_case_types)]
pub type u64xy = XY<u64>;

impl_xy!(u64);

#[allow(non_camel_case_types)]
pub type i64xy = XY<i64>;

impl_xy!(i64);
impl_xy_neg!(i64);

#[allow(non_camel_case_types)]
pub type usizexy = XY<usize>;

impl_xy!(usize);

#[allow(non_camel_case_types)]
pub type isizexy = XY<isize>;

impl_xy!(isize);
impl_xy_neg!(isize);

#[allow(non_camel_case_types)]
pub type f32xy = XY<f32>;

impl_xy!(f32);
impl_xy_neg!(f32);

#[allow(non_camel_case_types)]
pub type f64xy = XY<f64>;

impl_xy!(f64);
impl_xy_neg!(f64);

#[derive(Copy,Clone)]
pub struct XYZ<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_xyz (
    ($t:ty) => (
        impl XYZ<$t> {
            pub fn new(x: $t,y: $t,z: $t) -> XYZ<$t> {
                XYZ {
                    x: x,
                    y: y,
                    z: z,
                }
            }

            pub fn cross(a: XYZ<$t>,b: XYZ<$t>) -> XYZ<$t> {
                XYZ {
                    x: a.y * b.z - a.z * b.y,
                    y: a.z * b.x - a.x * b.z,
                    z: a.x * b.y - a.y * b.x,
                }
            }

            pub fn dot(a: XYZ<$t>,b: XYZ<$t>) -> $t {
                a.x * b.x + a.y * b.y + a.z * b.z
            }

            pub fn abs(&self) -> $t {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
            }

            pub fn norm(self) -> XYZ<$t> {
                let d = self.abs();
                if d != 0.0 {
                    self / d
                }
                else {
                    self
                }
            }
        }

        impl cmp::PartialEq for XYZ<$t> {
            fn eq(&self,other: &XYZ<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
            }
        }

        impl fmt::Display for XYZ<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }

        impl fmt::Debug for XYZ<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{})",self.x,self.y,self.z)
            }
        }

        impl Zero for XYZ<$t> {
            fn zero() -> XYZ<$t> {
                XYZ {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for XYZ<$t> {
            type Output = XYZ<$t>;
            fn neg(self) -> Self::Output {
                XYZ {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }

        impl ops::Add<XYZ<$t>> for XYZ<$t> {
            type Output = XYZ<$t>;
            fn add(self,other: XYZ<$t>) -> Self::Output {
                XYZ {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        impl ops::Sub<XYZ<$t>> for XYZ<$t> {
            type Output = XYZ<$t>;
            fn sub(self,other: XYZ<$t>) -> XYZ<$t> {
                XYZ {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }

        impl ops::AddAssign<XYZ<$t>> for XYZ<$t> {
            fn add_assign(&mut self,other: XYZ<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl ops::SubAssign<XYZ<$t>> for XYZ<$t> {
            fn sub_assign(&mut self,other: XYZ<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        impl ops::Mul<$t> for XYZ<$t> {
            type Output = XYZ<$t>;
            fn mul(self,other: $t) -> Self::Output {
                XYZ {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                }
            }
        }
                    

        impl ops::Mul<XYZ<$t>> for $t {
            type Output = XYZ<$t>;
            fn mul(self,other: XYZ<$t>) -> Self::Output {
                XYZ {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                }
            }
        }

        impl ops::MulAssign<$t> for XYZ<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }

        impl ops::Div<$t> for XYZ<$t> {
            type Output = XYZ<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    XYZ {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl ops::DivAssign<$t> for XYZ<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32xyz = XYZ<f32>;

impl_xyz!(f32);

#[allow(non_camel_case_types)]
pub type f64xyz = XYZ<f64>;

impl_xyz!(f64);

#[derive(Copy,Clone)]
pub struct XYZW<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

macro_rules! impl_xyzw (
    ($t:ty) => (
        impl XYZW<$t> {
            pub fn new(x: $t,y: $t,z: $t,w: $t) -> XYZW<$t> {
                XYZW {
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                }
            }
        }

        impl cmp::PartialEq for XYZW<$t> {
            fn eq(&self,other: &XYZW<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
                && (self.w == other.w)
            }
        }

        impl fmt::Display for XYZW<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
            }
        }

        impl fmt::Debug for XYZW<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({},{},{},{})",self.x,self.y,self.z,self.w)
            }
        }

        impl Zero for XYZW<$t> {
            fn zero() -> XYZW<$t> {
                XYZW {
                    x: <$t>::zero(),
                    y: <$t>::zero(),
                    z: <$t>::zero(),
                    w: <$t>::zero(),
                }
            }
        }

        impl ops::Neg for XYZW<$t> {
            type Output = XYZW<$t>;
            fn neg(self) -> Self::Output {
                XYZW {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                }
            }
        }

        impl ops::Add<XYZW<$t>> for XYZW<$t> {
            type Output = XYZW<$t>;
            fn add(self,other: XYZW<$t>) -> Self::Output {
                XYZW {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: self.w + other.w,
                }
            }
        }

        impl ops::Sub<XYZW<$t>> for XYZW<$t> {
            type Output = XYZW<$t>;
            fn sub(self,other: XYZW<$t>) -> XYZW<$t> {
                XYZW {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: self.w - other.w,
                }
            }
        }

        impl ops::AddAssign<XYZW<$t>> for XYZW<$t> {
            fn add_assign(&mut self,other: XYZW<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
                self.w += other.w;
            }
        }

        impl ops::SubAssign<XYZW<$t>> for XYZW<$t> {
            fn sub_assign(&mut self,other: XYZW<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
                self.w -= other.w;
            }
        }

        impl ops::Mul<$t> for XYZW<$t> {
            type Output = XYZW<$t>;
            fn mul(self,other: $t) -> Self::Output {
                XYZW {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    w: self.w * other,
                }
            }
        }

        impl ops::Mul<XYZW<$t>> for $t {
            type Output = XYZW<$t>;
            fn mul(self,other: XYZW<$t>) -> Self::Output {
                XYZW {
                    x: self * other.x,
                    y: self * other.y,
                    z: self * other.z,
                    w: self * other.w,
                }
            }
        }

        impl ops::MulAssign<$t> for XYZW<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
                self.w *= other;
            }
        }

        impl ops::Div<$t> for XYZW<$t> {
            type Output = XYZW<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    XYZW {
                        x: self.x / other,
                        y: self.y / other,
                        z: self.z / other,
                        w: self.w / other,
                    }    
                }
                else {
                    self
                }
            }
        }

        impl ops::DivAssign<$t> for XYZW<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                    self.z /= other;
                    self.w /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32xyzw = XYZW<f32>;

impl_xyzw!(f32);

#[allow(non_camel_case_types)]
pub type f64xyzw = XYZW<f64>;

impl_xyzw!(f64);
