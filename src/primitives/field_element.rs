use num::One;

/// Supertrait that numbers for FieldElement need to implement.
pub trait Number<T>:
    Default
    + Clone
    + std::cmp::PartialEq
    + std::cmp::PartialOrd
    + std::fmt::Debug
    + std::ops::Add<Output = T>
    + std::ops::Sub<Output = T>
    + One
{
}

impl<T> Number<T> for T where
    T: Default
        + Clone
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + std::fmt::Debug
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + One
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
        if num >= prime || num < T::default() {
            panic!("{:?} is not in field range 0 to {:?}", num, prime);
        }

        FieldElement(num, prime)
    }
}

// Implement the From trait for easy casting from numbers into FieldElement.
impl<T> From<T> for FieldElement<T>
where
    T: Number<T>,
{
    fn from(value: T) -> Self {
        let prime = value.clone() + One::one();
        FieldElement::new(value, prime)
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
    type Output = FieldElement<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let highest_prime = match self.1 >= rhs.1 {
            true => self.1,
            false => rhs.1,
        };

        let num = self.0.clone() + rhs.0.clone();

        let prime = match num > highest_prime {
            true => num.clone() + One::one(),
            false => highest_prime,
        };

        FieldElement::new(num, prime)
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
    fn add_works() {
        let a = FieldElement::new(10, 25);
        let b = FieldElement::new(12, 13);

        assert!(a != b);

        let result = a + b;
        assert_eq!(result, 22.into());
    }
}
