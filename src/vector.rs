use cast::transmute;
use vec::raw::buf_as_slice;
use ptr::to_unsafe_ptr;
use cmp::{Eq, Ord};
use num::from_int;
use std::cmp::FuzzyEq;

use math::*;

//
//  N-dimensional Vector
//
pub trait Vector<T> {
    static pure fn dim() -> uint;
    
    pure fn add_t(value: T) -> self;
    pure fn sub_t(value: T) -> self;
    pure fn mul_t(value: T) -> self;
    pure fn div_t(value: T) -> self;
    
    pure fn add_v(other: &self) -> self;
    pure fn sub_v(other: &self) -> self;
    
    pure fn dot(other: &self) -> T;
    
    pure fn magnitude2() -> T;
    pure fn magnitude() -> T;
    pure fn normalize() -> self;
    
    pure fn lerp(other: &self, value: T) -> self;
}

pub trait Vector3<T> {
    fn cross(other: &self) -> self;
}




//
//  Vec2
//
pub struct Vec2<T> { x: T, y: T }


//
//  Constructor
//
#[inline(always)]
pub pure fn Vec2<T:Copy>(x: T, y: T) -> Vec2<T> {
    Vec2 { x: move x, y: move y }
}

pub mod Vec2 {
    #[inline(always)] pub pure fn zero     <T: Num>() -> Vec2<T> { Vec2 { x: from_int(0), y: from_int(0) } }
    #[inline(always)] pub pure fn unit_x   <T: Num>() -> Vec2<T> { Vec2 { x: from_int(1), y: from_int(0) } }
    #[inline(always)] pub pure fn unit_y   <T: Num>() -> Vec2<T> { Vec2 { x: from_int(0), y: from_int(1) } }
    #[inline(always)] pub pure fn identity <T: Num>() -> Vec2<T> { Vec2 { x: from_int(1), y: from_int(1) } }
}

pub impl<T:Copy Num Sqrt> Vec2<T>: Vector<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 2 }
    
    #[inline(always)]
    pure fn add_t(value: T) -> Vec2<T> {
        Vec2(self[0] + value,
             self[1] + value)
    }
    
    #[inline(always)]
    pure fn sub_t(value: T) -> Vec2<T> {
        Vec2(self[0] - value,
             self[1] - value)
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Vec2<T> {
        Vec2(self[0] * value,
             self[1] * value)
    }
    
    #[inline(always)]
    pure fn div_t(value: T) -> Vec2<T> {
        Vec2(self[0] / value,
             self[1] / value)
    }
    
    #[inline(always)]
    pure fn add_v(other: &Vec2<T>) -> Vec2<T> {
        Vec2(self[0] + other[0],
             self[1] + other[1])
    }
    
    #[inline(always)]
    pure fn sub_v(other: &Vec2<T>) -> Vec2<T> {
        Vec2(self[0] - other[0],
             self[1] - other[1])
    }
    
    #[inline(always)]
    pure fn dot(other: &Vec2<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1]
    }
    
    #[inline(always)]
    pure fn magnitude2() -> T {
        self[0] * self[0] +
        self[1] * self[1]
    }
    
    #[inline(always)]
    pure fn magnitude() -> T {
        self.magnitude2().sqrt()
    }
    
    #[inline(always)]
    pure fn normalize() -> Vec2<T> {
        let mut n: T = from_int(1); 
        n /= self.magnitude();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(other: &Vec2<T>, value: T) -> Vec2<T> {
        self.add_v(&other.sub_v(&self).mul_t(value))
    }
}

pub impl<T:Copy> Vec2<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec2<T>, *T>(
                to_unsafe_ptr(&self)), 2) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy MinMax> Vec2<T>: MinMax {
    #[inline(always)]
    pure fn min(other: &Vec2<T>) -> Vec2<T> {
        Vec2(min(&self[0], &other[0]),
             min(&self[1], &other[1]))
    }
    
    #[inline(always)]
    pure fn max(other: &Vec2<T>) -> Vec2<T> {
        Vec2(max(&self[0], &other[0]),
             max(&self[1], &other[1]))
    }
}

pub impl<T:Copy Abs> Vec2<T>: Abs {
    #[inline(always)]
    pure fn abs() -> Vec2<T> {
        Vec2(abs(&self[0]),
             abs(&self[1]))
    }
}

pub impl<T:Copy Neg<T>> Vec2<T>: Neg<Vec2<T>> {
    #[inline(always)]
    pure fn neg() -> Vec2<T> {
        Vec2(-self[0], -self[1])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Vec2<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Vec2<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Vec2<T>) -> bool {
        !(self == *other)
    }
}

impl<T:Copy Eq> Vec2<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Vec2<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1]
    }
}

pub impl<T:Copy FuzzyEq> Vec2<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Vec2<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}

pub impl<T:Copy> Vec2<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        to_unsafe_ptr(&self[0])
    }
}






//
//  Vec3
//
pub struct Vec3<T> { x: T, y: T, z: T }

//
//  Constructor
//
#[inline(always)]
pub pure fn Vec3<T:Copy>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x: move x, y: move y, z: move z }
}

pub mod Vec3 {
    #[inline(always)] pub pure fn zero     <T: Num>() -> Vec3<T> { Vec3 { x: from_int(0), y: from_int(0), z: from_int(0) } }
    #[inline(always)] pub pure fn unit_x   <T: Num>() -> Vec3<T> { Vec3 { x: from_int(1), y: from_int(0), z: from_int(0) } }
    #[inline(always)] pub pure fn unit_y   <T: Num>() -> Vec3<T> { Vec3 { x: from_int(0), y: from_int(1), z: from_int(0) } }
    #[inline(always)] pub pure fn unit_z   <T: Num>() -> Vec3<T> { Vec3 { x: from_int(0), y: from_int(0), z: from_int(1) } }
    #[inline(always)] pub pure fn identity <T: Num>() -> Vec3<T> { Vec3 { x: from_int(1), y: from_int(1), z: from_int(1) } }
}

pub impl<T:Copy Num> Vec3<T>: Vector3<T> {
    #[inline(always)]
    fn cross(other: &Vec3<T>) -> Vec3<T> {
        Vec3((self[1] * other[2]) - (self[2] * other[1]),
             (self[2] * other[0]) - (self[0] * other[2]),
             (self[0] * other[1]) - (self[1] * other[0]))
    }
}

pub impl<T:Copy Num Sqrt> Vec3<T>: Vector<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 3 }
    
    #[inline(always)]
    pure fn add_t(value: T) -> Vec3<T> {
        Vec3(self[0] + value,
             self[1] + value,
             self[2] + value)
    }
    
    #[inline(always)]
    pure fn sub_t(value: T) -> Vec3<T> {
        Vec3(self[0] - value,
             self[1] - value,
             self[2] - value)
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Vec3<T> {
        Vec3(self[0] * value,
             self[1] * value,
             self[2] * value)
    }
    
    #[inline(always)]
    pure fn div_t(value: T) -> Vec3<T> {
        Vec3(self[0] / value,
             self[1] / value,
             self[2] / value)
    }
    
    #[inline(always)]
    pure fn add_v(other: &Vec3<T>) -> Vec3<T>{
        Vec3(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2])
    }
    
    #[inline(always)]
    pure fn sub_v(other: &Vec3<T>) -> Vec3<T>{
        Vec3(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2])
    }
    
    #[inline(always)]
    pure fn dot(other: &Vec3<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2]
    }
    
    #[inline(always)]
    pure fn magnitude2() -> T {
        self[0] * self[0] +
        self[1] * self[1] +
        self[2] * self[2]
    }
    
    #[inline(always)]
    pure fn magnitude() -> T {
        self.magnitude2().sqrt()
    }
    
    #[inline(always)]
    pure fn normalize() -> Vec3<T> {
        let mut n: T = from_int(1);
        n /= self.magnitude();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(other: &Vec3<T>, value: T) -> Vec3<T> {
        self.add_v(&other.sub_v(&self).mul_t(value))
    }
}

pub impl<T:Copy> Vec3<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe { do buf_as_slice(
            transmute::<*Vec3<T>, *T>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy MinMax> Vec3<T>: MinMax {
    #[inline(always)]
    pure fn min(other: &Vec3<T>) -> Vec3<T> {
        Vec3(min(&self[0], &other[0]),
             min(&self[1], &other[1]),
             min(&self[2], &other[2]))
    }
    
    #[inline(always)]
    pure fn max(other: &Vec3<T>) -> Vec3<T> {
        Vec3(max(&self[0], &other[0]),
             max(&self[1], &other[1]),
             max(&self[2], &other[2]))
    }
}

pub impl<T:Copy Abs> Vec3<T>: Abs {
    #[inline(always)]
    pure fn abs() -> Vec3<T> {
        Vec3(abs(&self[0]),
             abs(&self[1]),
             abs(&self[2]))
    }
}

pub impl<T:Copy Neg<T>> Vec3<T>: Neg<Vec3<T>> {
    #[inline(always)]
    pure fn neg() -> Vec3<T> {
        Vec3(-self[0], -self[1], -self[2])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Vec3<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Vec3<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Vec3<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy Eq> Vec3<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Vec3<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2]
    }
}

pub impl<T:Copy FuzzyEq> Vec3<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Vec3<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}

pub impl<T:Copy> Vec3<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        to_unsafe_ptr(&self[0])
    }
}






//
//  Vec4
//
pub struct Vec4<T> { x: T, y: T, z: T, w: T }

pub mod Vec4 {
    #[inline(always)] pub pure fn zero     <T: Num>() -> Vec4<T> { Vec4 { x: from_int(0), y: from_int(0), z: from_int(0), w: from_int(0) } }
    #[inline(always)] pub pure fn unit_x   <T: Num>() -> Vec4<T> { Vec4 { x: from_int(1), y: from_int(0), z: from_int(0), w: from_int(0) } }
    #[inline(always)] pub pure fn unit_y   <T: Num>() -> Vec4<T> { Vec4 { x: from_int(0), y: from_int(1), z: from_int(0), w: from_int(0) } }
    #[inline(always)] pub pure fn unit_z   <T: Num>() -> Vec4<T> { Vec4 { x: from_int(0), y: from_int(0), z: from_int(1), w: from_int(0) } }
    #[inline(always)] pub pure fn unit_w   <T: Num>() -> Vec4<T> { Vec4 { x: from_int(0), y: from_int(0), z: from_int(0), w: from_int(1) } }
    #[inline(always)] pub pure fn identity <T: Num>() -> Vec4<T> { Vec4 { x: from_int(1), y: from_int(1), z: from_int(1), w: from_int(1) } }
}

//
//  Constructor
//
#[inline(always)]
pub pure fn Vec4<T:Copy>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x: move x, y: move y, z: move z, w: move w }
}

pub impl<T:Copy Num Sqrt> Vec4<T>: Vector<T> {
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    pure fn add_t(value: T) -> Vec4<T> {
        Vec4(self[0] + value,
             self[1] + value,
             self[2] + value,
             self[3] + value)
    }
    
    #[inline(always)]
    pure fn sub_t(value: T) -> Vec4<T> {
        Vec4(self[0] - value,
             self[1] - value,
             self[2] - value,
             self[3] - value)
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Vec4<T> {
        Vec4(self[0] * value,
             self[1] * value,
             self[2] * value,
             self[3] * value)
    }
    
    #[inline(always)]
    pure fn div_t(value: T) -> Vec4<T> {
        Vec4(self[0] / value,
             self[1] / value,
             self[2] / value,
             self[3] / value)
    }
    
    #[inline(always)]
    pure fn add_v(other: &Vec4<T>) -> Vec4<T> {
        Vec4(self[0] + other[0],
             self[1] + other[1],
             self[2] + other[2],
             self[3] + other[3])
    }
    
    #[inline(always)]
    pure fn sub_v(other: &Vec4<T>) -> Vec4<T> {
        Vec4(self[0] - other[0],
             self[1] - other[1],
             self[2] - other[2],
             self[3] - other[3])
    }
    
    #[inline(always)]
    pure fn dot(other: &Vec4<T>) -> T {
        self[0] * other[0] +
        self[1] * other[1] +
        self[2] * other[2] +
        self[3] * other[3]
    }
    
    #[inline(always)]
    pure fn magnitude2() -> T {
        self[0] * self[0] +
        self[1] * self[1] +
        self[2] * self[2] +
        self[3] * self[3]
    }
    
    #[inline(always)]
    pure fn magnitude() -> T {
        self.magnitude2().sqrt()
    }
    
    #[inline(always)]
    pure fn normalize() -> Vec4<T> {
        let mut n: T = from_int(1);
        n /= self.magnitude();
        return self.mul_t(n);
    }
    
    #[inline(always)]
    pure fn lerp(other: &Vec4<T>, value: T) -> Vec4<T> {
        self.add_v(&other.sub_v(&self).mul_t(value))
    }
}

pub impl<T:Copy> Vec4<T>: Index<uint, T> {
    #[inline(always)]
    pure fn index(i: uint) -> T {
        unsafe {
            do buf_as_slice(
                transmute::<*Vec4<T>, *T>(
                    to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy MinMax> Vec4<T>: MinMax {
    #[inline(always)]
    pure fn min(other: &Vec4<T>) -> Vec4<T> {
        Vec4(min(&self[0], &other[0]),
             min(&self[1], &other[1]),
             min(&self[2], &other[2]),
             min(&self[3], &other[3]))
    }
    
    #[inline(always)]
    pure fn max(other: &Vec4<T>) -> Vec4<T> {
        Vec4(max(&self[0], &other[0]),
             max(&self[1], &other[1]),
             max(&self[2], &other[2]),
             max(&self[3], &other[3]))
    }
}

pub impl<T:Copy Abs> Vec4<T>: Abs {
    #[inline(always)]
    pure fn abs() -> Vec4<T> {
        Vec4(abs(&self[0]),
             abs(&self[1]),
             abs(&self[2]),
             abs(&self[3]))
    }
}

pub impl<T:Copy Neg<T>> Vec4<T>: Neg<Vec4<T>> {
    #[inline(always)]
    pure fn neg() -> Vec4<T> {
        Vec4(-self[0], -self[1], -self[2], -self[3])
    }
}

pub impl<T:Copy FuzzyEq> Vec4<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Vec4<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Vec4<T>) -> bool {
        !(self == *other)
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy Eq> Vec4<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Vec4<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
}

pub impl<T:Copy FuzzyEq> Vec4<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Vec4<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}

pub impl<T:Copy> Vec4<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        to_unsafe_ptr(&self[0])
    }
}