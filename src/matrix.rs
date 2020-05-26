// Math
// by Desmond Germans

use std::{cmp,fmt,ops};
use crate::*;

#[derive(Copy,Clone)]
pub struct Mat2x2<T> {
    pub x: XY<T>,
    pub y: XY<T>,
}

macro_rules! impl_mat2x2 (
    ($t:ty) => (
        impl Mat2x2<$t> {
            pub fn new(x: &XY<$t>,y: &XY<$t>) -> Mat2x2<$t> {
                Mat2x2 {
                    x: *x,
                    y: *y,
                }
            }
        }

        impl cmp::PartialEq for Mat2x2<$t> {
            fn eq(&self,other: &Mat2x2<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
            }
        }
        
        impl fmt::Display for Mat2x2<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({}; {};)",self.x,self.y)
            }
        }
        
        impl fmt::Debug for Mat2x2<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({}; {};)",self.x,self.y)
            }
        }
        
        impl Zero for Mat2x2<$t> {
            fn zero() -> Mat2x2<$t> {
                Mat2x2 {
                    x: XY::zero(),
                    y: XY::zero(),
                }
            }
        }

        impl One for Mat2x2<$t> {
            fn one() -> Mat2x2<$t> {
                Mat2x2 {
                    x: XY::<$t>::new(1.0,0.0),
                    y: XY::<$t>::new(0.0,1.0),
                }
            }
        }

        impl ops::Neg for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn neg(self) -> Self::Output {
                Mat2x2 {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl ops::Add<Mat2x2<$t>> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn add(self,other: Mat2x2<$t>) -> Self::Output {
                Mat2x2 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl ops::Sub<Mat2x2<$t>> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn sub(self,other: Mat2x2<$t>) -> Self::Output {
                Mat2x2 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        impl ops::AddAssign<Mat2x2<$t>> for Mat2x2<$t> {
            fn add_assign(&mut self,other: Mat2x2<$t>) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        impl ops::SubAssign<Mat2x2<$t>> for Mat2x2<$t> {
            fn sub_assign(&mut self,other: Mat2x2<$t>) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }

        impl ops::Mul<$t> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Mat2x2 {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }

        impl ops::Mul<XY<$t>> for Mat2x2<$t> {
            type Output = XY<$t>;
            fn mul(self,other: XY<$t>) -> Self::Output {
                XY {
                    x: self.x.x * other.x + self.y.x * other.y,
                    y: self.x.y * other.x + self.y.y * other.y,
                }
            }
        }

        impl ops::Mul<Mat2x2<$t>> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn mul(self,other: Mat2x2<$t>) -> Self::Output {
                Mat2x2 {
                    x: XY::<$t>::new(
                        self.x.x * other.x.x + self.y.x * other.x.y,
                        self.x.y * other.x.x + self.y.y * other.x.y
                    ),
                    y: XY::<$t>::new(
                        self.x.x * other.y.x + self.y.x * other.y.y,
                        self.x.y * other.y.x + self.y.y * other.y.y
                    ),
                }
            }
        }

        impl ops::Div<$t> for Mat2x2<$t> {
            type Output = Mat2x2<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Mat2x2 {
                        x: self.x / other,
                        y: self.y / other,
                    }
                }
                else {
                    self
                }
            }
        }

        impl ops::MulAssign<$t> for Mat2x2<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
            }
        }

        impl ops::MulAssign<Mat2x2<$t>> for Mat2x2<$t> {
            fn mul_assign(&mut self,other: Mat2x2<$t>) {
                let nx = XY::<$t>::new(
                    self.x.x * other.x.x + self.y.x * other.x.y,
                    self.x.y * other.x.x + self.y.y * other.x.y
                );
                let ny = XY::<$t>::new(
                    self.x.x * other.y.x + self.y.x * other.y.y,
                    self.x.y * other.y.x + self.y.y * other.y.y
                );
                self.x = nx;
                self.y = ny;
            }
        }

        impl ops::DivAssign<$t> for Mat2x2<$t> {
            fn div_assign(&mut self,other: $t) {
                if other != 0.0 {
                    self.x /= other;
                    self.y /= other;
                }
            }
        }
    );
);

#[allow(non_camel_case_types)]
pub type f32m2x2 = Mat2x2<f32>;

impl_mat2x2!(f32);

#[allow(non_camel_case_types)]
pub type f64m2x2 = Mat2x2<f64>;

impl_mat2x2!(f64);

#[derive(Copy,Clone)]
pub struct Mat3x3<T> {
    pub x: XYZ<T>,
    pub y: XYZ<T>,
    pub z: XYZ<T>,
}

macro_rules! impl_mat3x3 (
    ($t:ty) => (
        impl Mat3x3<$t> {
            pub fn new(x: XYZ<$t>,y: XYZ<$t>,z: XYZ<$t>) -> Mat3x3<$t> {
                Mat3x3 {
                    x: x,
                    y: y,
                    z: z,
                }
            }

            pub fn scale(s: XYZ<$t>) -> Mat3x3<$t> {
                Mat3x3 {
                    x: XYZ::<$t>::new(s.x,0.0,0.0),
                    y: XYZ::<$t>::new(0.0,s.y,0.0),
                    z: XYZ::<$t>::new(0.0,0.0,s.z),
                }
            }

            pub fn rotate(r: $t,i: $t,j: $t,k: $t) -> Mat3x3<$t> {
                let mut rr = r * r;
                let mut ii = i * i;
                let mut jj = j * j;
                let kk = k * k;
                let n = rr + ii + jj + kk;
                let s = if n != 0.0 {
                    2.0 / n
                }
                else {
                    0.0
                };
                let kr = s * k * r;
                rr *= s;
                ii *= s;
                let ki = s * k * i;
                let ri = s * r * i;
                let ij = s * i * j;
                let kj = s * k * j;
                let rj = s * r * j;
                jj *= s;
                Mat3x3 {
                    x: XYZ::<$t>::new(1.0 - (ii + jj),ri - kj,rj + ki),
                    y: XYZ::<$t>::new(ri + kj,1.0 - (rr + jj),ij - kr),
                    z: XYZ::<$t>::new(rj - ki,ij + kr,1.0 - (rr + ii)),
                }
            }

            pub fn pitch(a: $t) -> Mat3x3<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3 {
                    x: XYZ::<$t>::new(1.0,0.0,0.0),
                    y: XYZ::<$t>::new(0.0,ca,sa),
                    z: XYZ::<$t>::new(0.0,-sa,ca),
                }
            }

            pub fn yaw(a: $t) -> Mat3x3<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3 {
                    x: XYZ::<$t>::new(ca,0.0,-sa),
                    y: XYZ::<$t>::new(0.0,1.0,0.0),
                    z: XYZ::<$t>::new(sa,0.0,ca),
                }
            }

            pub fn roll(a: $t) -> Mat3x3<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat3x3 {
                    x: XYZ::<$t>::new(ca,sa,0.0),
                    y: XYZ::<$t>::new(-sa,ca,0.0),
                    z: XYZ::<$t>::new(0.0,0.0,1.0),
                }
            }

            pub fn normal_from(m: Mat4x4<$t>) -> Mat3x3<$t> {
                Mat3x3 {
                    x: XYZ::<$t>::new(m.x.x,m.x.y,m.x.z),
                    y: XYZ::<$t>::new(m.y.x,m.y.y,m.y.z),
                    z: XYZ::<$t>::new(m.z.x,m.z.y,m.z.z),
                }.inverse().transpose()
            }

            pub fn det(&self) -> $t {
                let a = self.x.x;
                let b = self.y.x;
                let c = self.z.x;
                let d = self.x.y;
                let e = self.y.y;
                let f = self.z.y;
                let g = self.x.z;
                let h = self.y.z;
                let i = self.z.z;
                let cofa = e * i - f * h;
                let cofb = f * g - d * i;
                let cofc = d * h - e * g;
                a * cofa - b * cofb + c * cofc
            }

            pub fn inverse(&self) -> Mat3x3<$t> {
                let a = self.x.x;
                let b = self.y.x;
                let c = self.z.x;
                let d = self.x.y;
                let e = self.y.y;
                let f = self.z.y;
                let g = self.x.z;
                let h = self.y.z;
                let i = self.z.z;
                let ma = e * i - f * h;
                let md = f * g - d * i;
                let mg = d * h - e * g;
                let nd = a * ma + b * md + c * mg;
                if nd != 0.0 {
                    let mb = c * h - b * i;
                    let mc = b * f - c * e;
                    let me = a * i - c * g;
                    let mf = c * d - a * f;
                    let mh = b * g - a * h;
                    let mi = a * e - b * d;
                    Mat3x3 {
                        x: XYZ::<$t>::new(ma,md,mg),
                        y: XYZ::<$t>::new(mb,me,mh),
                        z: XYZ::<$t>::new(mc,mf,mi),
                    } / nd
                }
                else {
                    Mat3x3::one()
                }
            }

            pub fn transpose(&self) -> Mat3x3<$t> {
                Mat3x3 {
                    x: XYZ::<$t>::new(self.x.x,self.y.x,self.z.x),
                    y: XYZ::<$t>::new(self.x.y,self.y.y,self.z.y),
                    z: XYZ::<$t>::new(self.x.z,self.y.z,self.z.z),
                }
            }
        }

        impl cmp::PartialEq for Mat3x3<$t> {
            fn eq(&self,other: &Mat3x3<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
            }
        }
        
        impl fmt::Display for Mat3x3<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({}; {}; {};)",self.x,self.y,self.z)
            }
        }
        
        impl fmt::Debug for Mat3x3<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({}; {}; {};)",self.x,self.y,self.z)
            }
        }
        
        impl Zero for Mat3x3<$t> {
            fn zero() -> Mat3x3<$t> {
                Mat3x3 {
                    x: XYZ::zero(),
                    y: XYZ::zero(),
                    z: XYZ::zero(),
                }
            }
        }

        impl One for Mat3x3<$t> {
            fn one() -> Mat3x3<$t> {
                Mat3x3 {
                    x: XYZ::<$t>::new(1.0,0.0,0.0),
                    y: XYZ::<$t>::new(0.0,1.0,0.0),
                    z: XYZ::<$t>::new(0.0,0.0,1.0),
                }
            }
        }

        impl ops::Neg for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn neg(self) -> Self::Output {
                Mat3x3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }

        impl ops::Add<Mat3x3<$t>> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn add(self,other: Mat3x3<$t>) -> Self::Output {
                Mat3x3 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        impl ops::Sub<Mat3x3<$t>> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn sub(self,other: Mat3x3<$t>) -> Self::Output {
                Mat3x3 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }

        impl ops::AddAssign<Mat3x3<$t>> for Mat3x3<$t> {
            fn add_assign(&mut self,other: Mat3x3<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl ops::SubAssign<Mat3x3<$t>> for Mat3x3<$t> {
            fn sub_assign(&mut self,other: Mat3x3<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        impl ops::Mul<$t> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Mat3x3 {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                }
            }
        }

        impl ops::Mul<XYZ<$t>> for Mat3x3<$t> {
            type Output = XYZ<$t>;
            fn mul(self,other: XYZ<$t>) -> Self::Output {
                XYZ {
                    x: self.x.x * other.x + self.y.x * other.y + self.z.x * other.z,
                    y: self.x.y * other.x + self.y.y * other.y + self.z.y * other.z,
                    z: self.x.z * other.x + self.y.z * other.y + self.z.z * other.z,
                }
            }
        }

        impl ops::Mul<Mat3x3<$t>> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn mul(self,other: Mat3x3<$t>) -> Self::Output {
                Mat3x3 {
                    x: XYZ::<$t>::new(
                        self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z,
                        self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z,
                        self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z,
                    ),
                    y: XYZ::<$t>::new(
                        self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z,
                        self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z,
                        self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z,
                    ),
                    z: XYZ::<$t>::new(
                        self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z,
                        self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z,
                        self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z,
                    ),
                }
            }
        }

        impl ops::Div<$t> for Mat3x3<$t> {
            type Output = Mat3x3<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Mat3x3 {
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

        impl ops::MulAssign<$t> for Mat3x3<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }

        impl ops::MulAssign<Mat3x3<$t>> for Mat3x3<$t> {
            fn mul_assign(&mut self,other: Mat3x3<$t>) {
                let nx = XYZ::<$t>::new(
                    self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z,
                    self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z,
                    self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z,
                );
                let ny = XYZ::<$t>::new(
                    self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z,
                    self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z,
                    self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z,
                );
                let nz = XYZ::<$t>::new(
                    self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z,
                    self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z,
                    self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z,
                );
                self.x = nx;
                self.y = ny;
                self.z = nz;
            }
        }

        impl ops::DivAssign<$t> for Mat3x3<$t> {
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
pub type f32m3x3 = Mat3x3<f32>;

impl_mat3x3!(f32);

#[allow(non_camel_case_types)]
pub type f64m3x3 = Mat3x3<f64>;

impl_mat3x3!(f64);

#[derive(Copy,Clone)]
pub struct Mat4x4<T> {
    pub x: XYZW<T>,
    pub y: XYZW<T>,
    pub z: XYZW<T>,
    pub w: XYZW<T>,
}

macro_rules! impl_mat4x4 (
    ($t:ty) => (
        impl Mat4x4<$t> {
            pub fn new(x: XYZW<$t>,y: XYZW<$t>,z: XYZW<$t>,w: XYZW<$t>) -> Mat4x4<$t> {
                Mat4x4 {
                    x: x,
                    y: y,
                    z: z,
                    w: w,
                }
            }
        
            pub fn det(&self) -> $t {
                let a = self.x.x;
                let b = self.y.x;
                let c = self.z.x;
                let d = self.w.x;
                let e = self.x.y;
                let f = self.y.y;
                let g = self.z.y;
                let h = self.w.y;
                let i = self.x.z;
                let j = self.y.z;
                let k = self.z.z;
                let l = self.w.z;
                let m = self.x.w;
                let n = self.y.w;
                let o = self.z.w;
                let p = self.w.w;
                let kpol = k * p - o * l;
                let jpnl = j * p - n * l;
                let jonk = j * o - n * k;
                let ipml = i * p - m * l;
                let iomk = i * o - m * k;
                let inmj = i * n - m * j;
                let cofa = f * kpol - g * jpnl + h * jonk;
                let cofb = e * kpol - g * ipml + h * iomk;
                let cofc = e * jpnl - f * ipml + h * inmj;
                let cofd = e * jonk - f * iomk + g * inmj;
                a * cofa - b * cofb + c * cofc - d * cofd
            }

            pub fn inverse(&self) -> Mat4x4<$t> {
                let a = self.x.x;
                let b = self.y.x;
                let c = self.z.x;
                let d = self.w.x;
                let e = self.x.y;
                let f = self.y.y;
                let g = self.z.y;
                let h = self.w.y;
                let i = self.x.z;
                let j = self.y.z;
                let k = self.z.z;
                let l = self.w.z;
                let m = self.x.w;
                let n = self.y.w;
                let o = self.z.w;
                let p = self.w.w;
                let kpol = k * p - o * l;
                let jpnl = j * p - n * l;
                let jonk = j * o - n * k;
                let ipml = i * p - m * l;
                let iomk = i * o - m * k;
                let inmj = i * n - m * j;
                let cofa = f * kpol - g * jpnl + h * jonk;
                let cofb = e * kpol - g * ipml + h * iomk;
                let cofc = e * jpnl - f * ipml + h * inmj;
                let cofd = e * jonk - f * iomk + g * inmj;
                let nd = a * cofa - b * cofb + c * cofc - d * cofd;
                if(nd != 0.0)
                {
                    let chgd = c * h - g * d;
                    let bhfd = b * h - f * d;
                    let bgfc = b * g - f * c;
                    let ahed = a * h - e * d;
                    let agec = a * g - e * c;
                    let afeb = a * f - e * b;
                    let cofe = b * kpol - c * jpnl + d * jonk;
                    let coff = a * kpol - c * ipml + d * iomk;
                    let cofg = a * jpnl - b * ipml + d * inmj;
                    let cofh = a * jonk - b * iomk + c * inmj;
                    let cofi = n * chgd - o * bhfd + p * bgfc;
                    let cofj = m * chgd - o * ahed + p * agec;
                    let cofk = m * bhfd - n * ahed + p * afeb;
                    let cofl = m * bgfc - n * agec + o * afeb;
                    let cofm = j * chgd - k * bhfd + l * bgfc;
                    let cofn = i * chgd - k * ahed + l * agec;
                    let cofo = i * bhfd - j * ahed + l * afeb;
                    let cofp = i * bgfc - j * agec + k * afeb;
                    Mat4x4 {
                        x: XYZW::<$t>::new(cofa,-cofb,cofc,-cofd),
                        y: XYZW::<$t>::new(-cofe,coff,-cofg,cofh),
                        z: XYZW::<$t>::new(cofi,-cofj,cofk,-cofl),
                        w: XYZW::<$t>::new(-cofm,cofn,-cofo,cofp),
                    } / nd
                }
                else {
                    Mat4x4::one()
                }
            }

            pub fn transpose(&self) -> Mat4x4<$t> {
                Mat4x4 {
                    x: XYZW::<$t>::new(self.x.x,self.y.x,self.z.x,self.w.x),
                    y: XYZW::<$t>::new(self.x.y,self.y.y,self.z.y,self.w.y),
                    z: XYZW::<$t>::new(self.x.z,self.y.z,self.z.z,self.w.z),
                    w: XYZW::<$t>::new(self.x.w,self.y.w,self.z.w,self.w.w),
                }
            }

            pub fn ortho(l: $t,r: $t,b: $t,t: $t,n: $t,f: $t) -> Mat4x4<$t> {
                let dx = r - l;
                let dy = t - b;
                let dz = f - n;
                let rx = -(r + l) / dx;
                let ry = -(t + b) / dy;
                let rz = -(f + n) / dz;
                Mat4x4 {
                    x: XYZW::<$t>::new(2.0 / dx,0.0,0.0,0.0),
                    y: XYZW::<$t>::new(0.0,2.0 / dy,0.0,0.0),
                    z: XYZW::<$t>::new(0.0,0.0,-2.0 / dz,0.0),
                    w: XYZW::<$t>::new(rx,ry,rz,1.0),
                }
            }

            pub fn perspective(fovy: $t,aspect: $t,n: $t,f: $t) -> Mat4x4<$t> {
                let q = 1.0 / (fovy.to_radians() / 2.0).tan();
                Mat4x4 {
                    x: XYZW::<$t>::new(q / aspect,0.0,0.0,0.0),
                    y: XYZW::<$t>::new(0.0,q,0.0,0.0),
                    z: XYZW::<$t>::new(0.0,0.0,(f + n) / (n - f),-1.0),
                    w: XYZW::<$t>::new(0.0,0.0,2.0 * f * n / (n - f),0.0),
                }
            }

            pub fn translate(t: XYZ<$t>) -> Mat4x4<$t> {
                Mat4x4 {
                    x: XYZW::<$t>::new(1.0,0.0,0.0,0.0),
                    y: XYZW::<$t>::new(0.0,1.0,0.0,0.0),
                    z: XYZW::<$t>::new(0.0,0.0,1.0,0.0),
                    w: XYZW::<$t>::new(t.x,t.y,t.z,1.0),
                }
            }

            pub fn scale(s: XYZ<$t>) -> Mat4x4<$t> {
                Mat4x4 {
                    x: XYZW::<$t>::new(s.x,0.0,0.0,0.0),
                    y: XYZW::<$t>::new(0.0,s.y,0.0,0.0),
                    z: XYZW::<$t>::new(0.0,0.0,s.z,0.0),
                    w: XYZW::<$t>::new(0.0,0.0,0.0,1.0),
                }
            }

            pub fn rotate(r: $t,i: $t,j: $t,k: $t) -> Mat4x4<$t> {
                let mut rr = r * r;
                let mut ii = i * i;
                let mut jj = j * j;
                let kk = k * k;
                let n = rr + ii + jj + kk;
                let s = if n != 0.0 {
                    2.0 / n
                }
                else {
                    0.0
                };
                let kr = s * k * r;
                rr *= s;
                ii *= s;
                let ki = s * k * i;
                let ri = s * r * i;
                let ij = s * i * j;
                let kj = s * k * j;
                let rj = s * r * j;
                jj *= s;
                Mat4x4 {
                    x: XYZW::<$t>::new(1.0 - (ii + jj),ri - kj,rj + ki,0.0),
                    y: XYZW::<$t>::new(ri + kj,1.0 - (rr + jj),ij - kr,0.0),
                    z: XYZW::<$t>::new(rj - ki,ij + kr,1.0 - (rr + ii),0.0),
                    w: XYZW::<$t>::new(0.0,0.0,0.0,1.0),
                }
            }

            pub fn pitch(a: $t) -> Mat4x4<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat4x4 {
                    x: XYZW::<$t>::new(1.0,0.0,0.0,0.0),
                    y: XYZW::<$t>::new(0.0,ca,sa,0.0),
                    z: XYZW::<$t>::new(0.0,-sa,ca,0.0),
                    w: XYZW::<$t>::new(0.0,0.0,0.0,1.0),
                }
            }

            pub fn yaw(a: $t) -> Mat4x4<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat4x4 {
                    x: XYZW::<$t>::new(ca,0.0,-sa,0.0),
                    y: XYZW::<$t>::new(0.0,1.0,0.0,0.0),
                    z: XYZW::<$t>::new(sa,0.0,ca,0.0),
                    w: XYZW::<$t>::new(0.0,0.0,0.0,1.0),
                }
            }

            pub fn roll(a: $t) -> Mat4x4<$t> {
                let sa = a.sin();
                let ca = a.cos();
                Mat4x4 {
                    x: XYZW::<$t>::new(ca,sa,0.0,0.0),
                    y: XYZW::<$t>::new(-sa,ca,0.0,0.0),
                    z: XYZW::<$t>::new(0.0,0.0,1.0,0.0),
                    w: XYZW::<$t>::new(0.0,0.0,0.0,1.0),
                }
            }
        }

        impl cmp::PartialEq for Mat4x4<$t> {
            fn eq(&self,other: &Mat4x4<$t>) -> bool {
                (self.x == other.x)
                && (self.y == other.y)
                && (self.z == other.z)
                && (self.w == other.w)
            }
        }
        
        impl fmt::Display for Mat4x4<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({}; {}; {}; {};)",self.x,self.y,self.z,self.w)
            }
        }
        
        impl fmt::Debug for Mat4x4<$t> {
            fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,"({}; {}; {}; {};)",self.x,self.y,self.z,self.w)
            }
        }
        
        impl Zero for Mat4x4<$t> {
            fn zero() -> Mat4x4<$t> {
                Mat4x4 {
                    x: XYZW::zero(),
                    y: XYZW::zero(),
                    z: XYZW::zero(),
                    w: XYZW::zero(),
                }
            }
        }

        impl One for Mat4x4<$t> {
            fn one() -> Mat4x4<$t> {
                Mat4x4 {
                    x: XYZW::<$t>::new(1.0,0.0,0.0,0.0),
                    y: XYZW::<$t>::new(0.0,1.0,0.0,0.0),
                    z: XYZW::<$t>::new(0.0,0.0,1.0,0.0),
                    w: XYZW::<$t>::new(0.0,0.0,0.0,1.0),
                }
            }
        }

        impl ops::Neg for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn neg(self) -> Self::Output {
                Mat4x4 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                }
            }
        }

        impl ops::Add<Mat4x4<$t>> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn add(self,other: Mat4x4<$t>) -> Self::Output {
                Mat4x4 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                    w: self.w + other.w,
                }
            }
        }

        impl ops::Sub<Mat4x4<$t>> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn sub(self,other: Mat4x4<$t>) -> Self::Output {
                Mat4x4 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                    w: self.w - other.w,
                }
            }
        }

        impl ops::AddAssign<Mat4x4<$t>> for Mat4x4<$t> {
            fn add_assign(&mut self,other: Mat4x4<$t>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
                self.w += other.w;
            }
        }

        impl ops::SubAssign<Mat4x4<$t>> for Mat4x4<$t> {
            fn sub_assign(&mut self,other: Mat4x4<$t>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
                self.w -= other.w;
            }
        }

        impl ops::Mul<$t> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn mul(self,other: $t) -> Self::Output {
                Mat4x4 {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                    w: self.w * other,
                }
            }
        }

        impl ops::Mul<XYZW<$t>> for Mat4x4<$t> {
            type Output = XYZW<$t>;
            fn mul(self,other: XYZW<$t>) -> Self::Output {
                XYZW {
                    x: self.x.x * other.x + self.y.x * other.y + self.z.x * other.z + self.w.x * other.w,
                    y: self.x.y * other.x + self.y.y * other.y + self.z.y * other.z + self.w.y * other.w,
                    z: self.x.z * other.x + self.y.z * other.y + self.z.z * other.z + self.w.z * other.w,
                    w: self.x.w * other.x + self.y.w * other.y + self.z.w * other.z + self.w.w * other.w,
                }
            }
        }

        impl ops::Mul<Mat4x4<$t>> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn mul(self,other: Mat4x4<$t>) -> Self::Output {
                Mat4x4 {
                    x: XYZW::<$t>::new(
                        self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z + self.w.x * other.x.w,
                        self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z + self.w.y * other.x.w,
                        self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z + self.w.z * other.x.w,
                        self.x.w * other.x.x + self.y.w * other.x.y + self.z.w * other.x.z + self.w.w * other.x.w,
                    ),
                    y: XYZW::<$t>::new(
                        self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z + self.w.x * other.y.w,
                        self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z + self.w.y * other.y.w,
                        self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z + self.w.z * other.y.w,
                        self.x.w * other.y.x + self.y.w * other.y.y + self.z.w * other.y.z + self.w.w * other.y.w,
                    ),
                    z: XYZW::<$t>::new(
                        self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z + self.w.x * other.z.w,
                        self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z + self.w.y * other.z.w,
                        self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z + self.w.z * other.z.w,
                        self.x.w * other.z.x + self.y.w * other.z.y + self.z.w * other.z.z + self.w.w * other.z.w,
                    ),
                    w: XYZW::<$t>::new(
                        self.x.x * other.w.x + self.y.x * other.w.y + self.z.x * other.w.z + self.w.x * other.w.w,
                        self.x.y * other.w.x + self.y.y * other.w.y + self.z.y * other.w.z + self.w.y * other.w.w,
                        self.x.z * other.w.x + self.y.z * other.w.y + self.z.z * other.w.z + self.w.z * other.w.w,
                        self.x.w * other.w.x + self.y.w * other.w.y + self.z.w * other.w.z + self.w.w * other.w.w,
                    ),
                }
            }
        }

        impl ops::Div<$t> for Mat4x4<$t> {
            type Output = Mat4x4<$t>;
            fn div(self,other: $t) -> Self::Output {
                if other != 0.0 {
                    Mat4x4 {
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

        impl ops::MulAssign<$t> for Mat4x4<$t> {
            fn mul_assign(&mut self,other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
                self.w *= other;
            }
        }

        impl ops::MulAssign<Mat4x4<$t>> for Mat4x4<$t> {
            fn mul_assign(&mut self,other: Mat4x4<$t>) {
                let nx = XYZW::<$t>::new(
                    self.x.x * other.x.x + self.y.x * other.x.y + self.z.x * other.x.z + self.w.x * other.x.w,
                    self.x.y * other.x.x + self.y.y * other.x.y + self.z.y * other.x.z + self.w.y * other.x.w,
                    self.x.z * other.x.x + self.y.z * other.x.y + self.z.z * other.x.z + self.w.z * other.x.w,
                    self.x.w * other.x.x + self.y.w * other.x.y + self.z.w * other.x.z + self.w.w * other.x.w,
                );
                let ny = XYZW::<$t>::new(
                    self.x.x * other.y.x + self.y.x * other.y.y + self.z.x * other.y.z + self.w.x * other.y.w,
                    self.x.y * other.y.x + self.y.y * other.y.y + self.z.y * other.y.z + self.w.y * other.y.w,
                    self.x.z * other.y.x + self.y.z * other.y.y + self.z.z * other.y.z + self.w.z * other.y.w,
                    self.x.w * other.y.x + self.y.w * other.y.y + self.z.w * other.y.z + self.w.w * other.y.w,
                );
                let nz = XYZW::<$t>::new(
                    self.x.x * other.z.x + self.y.x * other.z.y + self.z.x * other.z.z + self.w.x * other.z.w,
                    self.x.y * other.z.x + self.y.y * other.z.y + self.z.y * other.z.z + self.w.y * other.z.w,
                    self.x.z * other.z.x + self.y.z * other.z.y + self.z.z * other.z.z + self.w.z * other.z.w,
                    self.x.w * other.z.x + self.y.w * other.z.y + self.z.w * other.z.z + self.w.w * other.z.w,
                );
                let nw = XYZW::<$t>::new(
                    self.x.x * other.w.x + self.y.x * other.w.y + self.z.x * other.w.z + self.w.x * other.w.w,
                    self.x.y * other.w.x + self.y.y * other.w.y + self.z.y * other.w.z + self.w.y * other.w.w,
                    self.x.z * other.w.x + self.y.z * other.w.y + self.z.z * other.w.z + self.w.z * other.w.w,
                    self.x.w * other.w.x + self.y.w * other.w.y + self.z.w * other.w.z + self.w.w * other.w.w,
                );
                self.x = nx;
                self.y = ny;
                self.z = nz;
                self.w = nw;
            }
        }

        impl ops::DivAssign<$t> for Mat4x4<$t> {
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
pub type f32m4x4 = Mat4x4<f32>;

impl_mat4x4!(f32);

#[allow(non_camel_case_types)]
pub type f64m4x4 = Mat4x4<f64>;

impl_mat4x4!(f64);
