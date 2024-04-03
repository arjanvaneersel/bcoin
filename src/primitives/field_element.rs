use mod_exp::mod_exp;
use num::{traits::Euclid, One, Zero};

/// Supertrait that numbers for FieldElement need to implement.
pub trait Number<T>:
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
    + std::ops::Add<T, Output = T>
    + std::ops::Sub<T, Output = T>
    + std::ops::Mul<T, Output = T>
    + std::ops::Div<T, Output = T>
    + std::ops::Rem<T, Output = T>
    + std::ops::Shr<T, Output = T>
{
}

impl<T> Number<T> for T where
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
    T: Number<T>,
{
    pub fn new(num: T, prime: T) -> FieldElement<T> {
        FieldElement(num, prime).__pass_if_valid_range()
    }

    fn __ensure_valid_range(&self) {
        if self.0 >= self.1 || self.0 < T::default() {
            panic!("{:?} is not in field range 0 to {:?}", self.0, self.1);
        }
    }

    fn __pass_if_valid_range(self) -> FieldElement<T> {
        self.__ensure_valid_range();
        self
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
        dbg!(self.0, n, self.1);
        let num = mod_exp(self.0, n, self.1);
        dbg!(num);

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
    T: Number<T>,
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
        FieldElement::new(num, self.1)
    }
}

// Implement the sub operator.
impl<T> std::ops::Sub for FieldElement<T>
where
    T: Number<T>,
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
        FieldElement::new(num, self.1)
    }
}

// Implement the mul operator.
impl<T> std::ops::Mul for FieldElement<T>
where
    T: Number<T>,
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
        FieldElement::new(num, self.1)
    }
}

// Implement the div operator.
impl<T> std::ops::Div for FieldElement<T>
where
    T: Number<T>,
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
        FieldElement::new(num, self.1)
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
    #[should_panic]
    fn can_not_create_out_of_range_field_element() {
        FieldElement::new(13, 7);
    }

    #[test]
    fn add_works() {
        let a = FieldElement(7, 13);
        let b = FieldElement(12, 13);
        let c = FieldElement(6, 13);
        assert_eq!(a + b, c);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn add_panics_on_lhs_out_of_range() {
        let a = FieldElement(12, 10);
        let b = FieldElement(3, 10);
        let _ = a + b;
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn add_panics_on_rhs_out_of_range() {
        let a = FieldElement(12, 10);
        let b = FieldElement(3, 10);
        let _ = b + a;
    }

    #[test]
    #[should_panic(expected = "Sides are of different fields")]
    fn add_panics_on_different_fields() {
        let a = FieldElement(7, 13);
        let b = FieldElement(12, 100);
        let _ = a + b;
    }

    #[test]
    fn sub_works() {
        let a = FieldElement(35, 40);
        let b = FieldElement(7, 40);
        let c = FieldElement(28, 40);
        assert_eq!(a - b, c);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn sub_panics_on_lhs_out_of_range() {
        let a = FieldElement(12, 10);
        let b = FieldElement(7, 10);
        let _ = a - b;
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn sub_panics_on_rhs_out_of_range() {
        let a = FieldElement(12, 10);
        let b = FieldElement(7, 10);
        let _ = b - a;
    }

    #[test]
    #[should_panic(expected = "Sides are of different fields")]
    fn sub_panics_on_different_fields() {
        let a = FieldElement(12, 13);
        let b = FieldElement(7, 100);
        let _ = a - b;
    }

    #[test]
    fn mul_works() {
        let a = FieldElement(3, 13);
        let b = FieldElement(12, 13);
        let c = FieldElement(10, 13);
        assert_eq!(a * b, c);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn mul_panics_on_lhs_out_of_range() {
        let a = FieldElement(12, 10);
        let b = FieldElement(7, 10);
        let _ = a * b;
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn mul_panics_on_rhs_out_of_range() {
        let a = FieldElement(12, 10);
        let b = FieldElement(7, 10);
        let _ = b * a;
    }

    #[test]
    #[should_panic(expected = "Sides are of different fields")]
    fn mul_panics_on_different_fields() {
        let a = FieldElement(7, 13);
        let b = FieldElement(12, 100);
        let _ = a * b;
    }

    #[test]
    fn pow_works() {
        let a = FieldElement(3, 13);
        let b = FieldElement(1, 13);
        assert_eq!(a.pow(3), b);

        let a = FieldElement(7, 13);
        let b = FieldElement(8, 13);
        assert_eq!(a.pow(-3), b);

        let a = FieldElement(7, 13);
        let b = FieldElement(1, 13);
        assert_eq!(a.pow(0), b);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn pow_panics_on_out_of_range() {
        let a = FieldElement(12, 10);
        let _ = a.pow(3);
    }

    #[test]
    fn div_works() {
        let a = FieldElement(10, 13);
        let b = FieldElement(2, 13);
        let c = FieldElement(5, 13);
        assert_eq!(a / b, c);
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn div_panics_on_lhs_out_of_range() {
        let a = FieldElement(12, 10);
        let b = FieldElement(7, 10);
        let _ = a / b;
    }

    #[test]
    #[should_panic(expected = "12 is not in field range 0 to 10")]
    fn div_panics_on_rhs_out_of_range() {
        let a = FieldElement(12, 10);
        let b = FieldElement(7, 10);
        let _ = b / a;
    }

    #[test]
    #[should_panic(expected = "Sides are of different fields")]
    fn div_panics_on_different_fields() {
        let a = FieldElement(12, 13);
        let b = FieldElement(7, 100);
        let _ = a / b;
    }

    #[test]
    fn debug_and_display_impl_works() {
        let a = FieldElement(10, 13);
        assert_eq!(format!("{:?}", a), "10".to_string());
        assert_eq!(format!("{}", a), "10".to_string());
    }
}
