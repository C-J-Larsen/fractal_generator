use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

// A trait for things that when the basic operations (+ - * /) are done on them,
// they return something of the same type. Also myst be copiable (aka a stack
// variable).
pub trait BasicOps: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self>
                    + Div<Output=Self> + Sized + Copy {}

impl<T> BasicOps for T where T: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self>
                                + Div<Output=Self> + Sized + Copy {}

// My own implementation of a complex number
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct MyComplex<T: BasicOps> {
    r: T,
    i: T,
}

impl<T: BasicOps> MyComplex<T> {
    // Constructor for MyComplex
    pub fn new(r: T, i: T) -> Self {
        Self { r: r, i: i }
    }
    // Getter functions for MyComplex
    pub fn r(&self) -> T { self.r }
    pub fn i(&self) -> T { self.i }

    // Define the magnitude (squared, for efficiency) operator for a MyComplex
    pub fn mag_sqr(&self) -> T {
        self.r * self.r + self.i * self.i
    }
}

// Override the basic mathematical operators, allong with '+=' for convenience
impl<T: BasicOps> Add for MyComplex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

impl<T: BasicOps> Sub for MyComplex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i
        }
    }
}

impl<T: BasicOps> Mul for MyComplex<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r
        }
    }
}

impl<T: BasicOps> Div for MyComplex<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let divisor: T = rhs.r*rhs.r + rhs.i*rhs.i;
        Self {
            r: (self.r*rhs.r + self.i*rhs.i) / divisor,
            i: (self.i*rhs.r - self.r*rhs.i) / divisor
        }
    }
}

impl<T: BasicOps> AddAssign for MyComplex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r = self.r + rhs.r;
        self.i = self.i + rhs.i;
    }
}

impl<T: BasicOps> SubAssign for MyComplex<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r = self.r - rhs.r;
        self.i = self.i - rhs.i;
    }
}

impl<T: BasicOps> MulAssign for MyComplex<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.r = self.r * rhs.r - self.i * rhs.i;
        self.i = self.r * rhs.i + self.i * rhs.r;
    }
}

impl<T: BasicOps> DivAssign for MyComplex<T> {
    fn div_assign(&mut self, rhs: Self) {
        let divisor: T = rhs.r*rhs.r + rhs.i*rhs.i;
        self.r = (self.r*rhs.r + self.i*rhs.i) / divisor;
        self.i = (self.i*rhs.r - self.r*rhs.i) / divisor;
    }
}
/*===================================================================
UNIT TESTS
===================================================================*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_addition_f32() {
        assert_eq!(MyComplex {r:8.9, i:5.2} + MyComplex {r:-4.0, i:0.1},
            MyComplex {r:4.9, i:5.3});
    }

    #[test]
    fn complex_addition_i16() {
        assert_eq!(MyComplex {r:8_i16, i:5_i16} + MyComplex {r:-4_i16, i:0_i16},
            MyComplex {r:4_i16, i:5_i16});
    }

    #[test]
    fn complex_subtraction() {
        assert_eq!(MyComplex {r:120.4, i:10.0} - MyComplex {r:-110.0, i:4.0},
            MyComplex {r:230.4, i:6.0});
    }

    #[test]
    fn complex_multiplication() {
        assert_eq!(MyComplex {r:120.4, i:10.0} * MyComplex {r:-110.0, i:4.0},
            MyComplex {r:-13_284.0, i:-618.4});
    }

    /*
    #[test]
    fn complex_division() {
        assert_eq!(MyComplex {r:8_i16, i:5_i16} + MyComplex {r:-4_i16, i:0_i16},
            MyComplex {r:4_i16, i:5_i16});
    }

    #[test]
    fn complex_plus_equals() {
        assert_eq!(MyComplex {r:8_i16, i:5_i16} + MyComplex {r:-4_i16, i:0_i16},
            MyComplex {r:4_i16, i:5_i16});
    }
    */
}
