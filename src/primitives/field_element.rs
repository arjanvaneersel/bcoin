use std::fmt::Display;

use mod_exp::mod_exp;
use num::{traits::Euclid, One, Zero};

/// Supertrait that numbers for FieldElement need to implement.
pub trait Number:
    Default
    + Clone
    + Copy
    + One
    + Zero
    + PartialEq
    + PartialOrd
    + Euclid
    + num::Bounded
    + num::Num
    + std::fmt::Debug
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::Rem<Self, Output = Self>
    + std::ops::Shr<Self, Output = Self>
{
}

#[cfg(feature = "signed_field_elements")]
impl<T> Number for T where
    T: Default
        + Clone
        + Copy
        + One
        + Zero
        + PartialEq
        + PartialOrd
        + Euclid
        + num::Bounded
        + num::Num
        + std::fmt::Debug
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::ops::Rem<T, Output = T>
        + std::ops::Shr<T, Output = T>
{
}

// Implement Number for unsigned primitive types
macro_rules! impl_number {
    ($T:ty) => {
        #[cfg(not(feature = "signed_field_elements"))]
        impl Number for $T {}
    };
}

// By default only unsigned numbers are enabled.
impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(u128);
impl_number!(usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Error<T> {
    NotInRange(T, T),
}

impl<T: std::fmt::Debug> Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NotInRange(v, p) => format!("{:?} is not in field range 0 to {:?}", v, p),
        };
        write!(f, "{}", s)
    }
}

impl<T: std::fmt::Debug> std::error::Error for Error<T> {}

/// A finite field
///
/// The mathematical definition of a finite field is:
/// 1. If a and b are in the set, then a + b and a * b are in the set. (Closed property)
/// 2. 0 exists and has the property a + 0 = a. (Additive identity)
/// 3. 1 exists and has the property a * 1 = a. (Multiplicative identity)
/// 4. If a is in the set then -a is in the set, which is defined as the value that makes a + (-a) = 0. (Additive inverse)
/// 5. If a is in the set and is not 0, a^-1 is in the set where a*a^-1 = 1. (Multiplicative inverse)
///
/// The mathematical notation is Fp = {0,1,2,...p-1}.
pub struct FieldElement<T>(T, T);

// Constructor with trait bounds.
impl<T> FieldElement<T>
where
    T: Number,
{
    pub fn new(num: T, prime: T) -> Result<FieldElement<T>, Error<T>> {
        let fe = FieldElement(num, prime);
        if !fe.has_valid_range(false) {
            return Err(Error::NotInRange(num, prime));
        }
        Ok(fe)
    }

    fn has_valid_range(&self, panic: bool) -> bool {
        if self.0 >= self.1 || self.0 < T::default() {
            if panic {
                panic!("{:?} is not in field range 0 to {:?}", self.0, self.1);
            }
            return false;
        }
        true
    }

    fn __ensure_valid_range(&self) {
        self.has_valid_range(true);
    }

    #[allow(dead_code)]
    // TODO: Remove #[allow(dead_code)] once it's being used.
    pub fn pow(&self, exp: T) -> FieldElement<T> {
        self.__ensure_valid_range();

        // If the exponent is zero, the value 1 should be returned.
        if exp == T::zero() {
            return FieldElement(T::one(), self.1);
        }

        // Ensure a positive exponent.
        let n = exp.rem_euclid(&(self.1 - T::one()));

        // Calculate while capping to prime.
        let num = mod_exp(self.0, n, self.1);

        FieldElement(num, self.1)
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for FieldElement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for FieldElement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// Implement the add operator.
impl<T> std::ops::Add for FieldElement<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.__ensure_valid_range();
        rhs.__ensure_valid_range();

        // Ensure that the primes are equal.
        if self.1 != rhs.1 {
            panic!("Sides are of different fields")
        }

        // Add the numbers while capping to the prime value.
        let num = (self.0 + rhs.0) % self.1;
        FieldElement::new(num, self.1).unwrap()
    }
}

// Implement the sub operator.
impl<T> std::ops::Sub for FieldElement<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.__ensure_valid_range();
        rhs.__ensure_valid_range();

        // Ensure that the primes are equal.
        if self.1 != rhs.1 {
            panic!("Sides are of different fields")
        }

        let num = (self.0 - rhs.0) % self.1;
        FieldElement::new(num, self.1).unwrap()
    }
}

// Implement the mul operator.
impl<T> std::ops::Mul for FieldElement<T>
where
    T: Number,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.__ensure_valid_range();
        rhs.__ensure_valid_range();

        // Ensure that the primes are equal.
        if self.1 != rhs.1 {
            panic!("Sides are of different fields")
        }

        // Multiply the numbers while capping to the prime value.
        let num = (self.0 * rhs.0) % self.1;
        FieldElement::new(num, self.1).unwrap()
    }
}

// Implement the div operator.
impl<T> std::ops::Div for FieldElement<T>
where
    T: Number,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.__ensure_valid_range();
        rhs.__ensure_valid_range();

        // Ensure that the primes are equal.
        if self.1 != rhs.1 {
            panic!("Sides are of different fields")
        }

        let two = T::one() + T::one();

        // Use Fermat's little theorem:
        // self.num**(p-1) % p == 1
        // this means:
        // 1/n == pow(n, p-2, p)
        let num = self.0 * mod_exp(rhs.0, self.1 - two, self.1) % self.1;
        FieldElement::new(num, self.1).unwrap()
    }
}

// Implement the eq operator.
impl<T: std::cmp::PartialEq> std::cmp::PartialEq for FieldElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: PartialEq> Eq for FieldElement<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_not_create_out_of_range_field_element() {
        assert_eq!(
            FieldElement::new(13_u8, 7_u8),
            Err(Error::NotInRange(13_u8, 7_u8))
        );
    }

    #[test]
    fn add_works() {
        let a = FieldElement(7_u8, 13_u8);
        let b = FieldElement(12_u8, 13_u8);
        let c = FieldElement(6_u8, 13_u8);
        assert_eq!(a + b, c);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn add_panics_on_lhs_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let b = FieldElement(3_u8, 10_u8);
        let _ = a + b;
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn add_panics_on_rhs_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let b = FieldElement(3_u8, 10_u8);
        let _ = b + a;
    }

    #[test]
    #[should_panic(expected = "Sides are of different fields")]
    fn add_panics_on_different_fields() {
        let a = FieldElement(7_u8, 13_u8);
        let b = FieldElement(12_u8, 100_u8);
        let _ = a + b;
    }

    #[test]
    fn sub_works() {
        let a = FieldElement(35_u8, 40_u8);
        let b = FieldElement(7_u8, 40_u8);
        let c = FieldElement(28_u8, 40_u8);
        assert_eq!(a - b, c);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn sub_panics_on_lhs_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let b = FieldElement(7_u8, 10_u8);
        let _ = a - b;
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn sub_panics_on_rhs_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let b = FieldElement(7_u8, 10_u8);
        let _ = b - a;
    }

    #[test]
    #[should_panic(expected = "Sides are of different fields")]
    fn sub_panics_on_different_fields() {
        let a = FieldElement(12_u8, 13_u8);
        let b = FieldElement(7_u8, 100_u8);
        let _ = a - b;
    }

    #[test]
    fn mul_works() {
        let a = FieldElement(3_u8, 13_u8);
        let b = FieldElement(12_u8, 13_u8);
        let c = FieldElement(10_u8, 13_u8);
        assert_eq!(a * b, c);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn mul_panics_on_lhs_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let b = FieldElement(7_u8, 10_u8);
        let _ = a * b;
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn mul_panics_on_rhs_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let b = FieldElement(7_u8, 10_u8);
        let _ = b * a;
    }

    #[test]
    #[should_panic(expected = "Sides are of different fields")]
    fn mul_panics_on_different_fields() {
        let a = FieldElement(7_u8, 13_u8);
        let b = FieldElement(12_u8, 100_u8);
        let _ = a * b;
    }

    #[test]
    fn pow_works() {
        let a = FieldElement(3_u8, 13_u8);
        let b = FieldElement(1_u8, 13_u8);
        assert_eq!(a.pow(3_u8), b);

        let a = FieldElement(7_u8, 13_u8);
        let b = FieldElement(1_u8, 13_u8);
        assert_eq!(a.pow(0_u8), b);
    }

    #[cfg(feature = "signed_field_elements")]
    #[test]
    fn pow_works_with_negative_exponential() {
        let a = FieldElement(7, 13);
        let b = FieldElement(8, 13);
        assert_eq!(a.pow(-3), b);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn pow_panics_on_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let _ = a.pow(3_u8);
    }

    #[test]
    fn div_works() {
        let a = FieldElement(10_u8, 13_u8);
        let b = FieldElement(2_u8, 13_u8);
        let c = FieldElement(5_u8, 13_u8);
        assert_eq!(a / b, c);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn div_panics_on_lhs_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let b = FieldElement(7_u8, 10_u8);
        let _ = a / b;
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn div_panics_on_rhs_out_of_range() {
        let a = FieldElement(12_u8, 10_u8);
        let b = FieldElement(7_u8, 10_u8);
        let _ = b / a;
    }

    #[test]
    #[should_panic(expected = "Sides are of different fields")]
    fn div_panics_on_different_fields() {
        let a = FieldElement(12_u8, 13_u8);
        let b = FieldElement(7_u8, 100_u8);
        let _ = a / b;
    }

    #[test]
    fn debug_and_display_impl_works() {
        let a = FieldElement(10_u8, 13_u8);
        assert_eq!(format!("{:?}", a), "10".to_string());
        assert_eq!(format!("{}", a), "10".to_string());
    }
}
