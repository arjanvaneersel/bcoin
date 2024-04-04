use num::{pow::Pow, One, Zero};

/// Supertrait that numbers for FieldElement need to implement.
pub trait Number:
    Default
    + Clone
    + Copy
    + PartialEq
    + One
    + Zero
    + std::fmt::Debug
    + Pow<u8, Output = Self>
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::Rem<Self, Output = Self>
    + std::ops::Shr<Self, Output = Self>
{
}

impl<T> Number for T where
    T: Default
        + Clone
        + Copy
        + PartialEq
        + One
        + Zero
        + std::fmt::Debug
        + Pow<u8, Output = Self>
        + std::ops::Add<Self, Output = Self>
        + std::ops::Sub<Self, Output = Self>
        + std::ops::Mul<Self, Output = Self>
        + std::ops::Div<Self, Output = Self>
        + std::ops::Rem<Self, Output = Self>
        + std::ops::Shr<Self, Output = Self>
{
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point<T> {
    a: T,
    b: T,
    x: Option<T>,
    y: Option<T>,
}

impl<T: Number> Eq for Point<T> {}

impl<T: Number> Point<T> {
    #[allow(dead_code)]
    // TODO: Remove #[allow(dead_code)] once it's being used.
    pub fn new(x: Option<T>, y: Option<T>, a: T, b: T) -> Point<T> {
        Point { a, b, x, y }.__pass_if_valid()
    }

    fn __ensure_valid(&self) {
        if let (Some(x), Some(y)) = (self.x, self.y) {
            if y.pow(2_u8) != x.pow(3_u8) + self.a * x + self.b {
                panic!("({:?}, {:?}) is not on the curve", x, y);
            }
        }
    }

    fn __pass_if_valid(self) -> Point<T> {
        self.__ensure_valid();
        self
    }
}

impl<T: Number> std::ops::Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.__ensure_valid();

        if self.a != rhs.a || self.b != rhs.b {
            panic!("{:?}, {:?} aren't on the same curve", self, rhs)
        }

        if self.x.is_none() {
            return rhs;
        }

        if rhs.x.is_none() {
            return self;
        }

        let self_x = self.x.unwrap();
        let self_y = self.y.unwrap();
        let rhs_x = rhs.x.unwrap();
        let rhs_y = rhs.y.unwrap();

        if self_x == rhs_x && self_y != rhs_y {
            return Point::new(None, None, self.a, self.b);
        }

        if self_x != rhs_x {
            let s = (rhs_y - self_y) / (rhs_x - self_x);
            let x = s.pow(2) - self_x - rhs_x;
            let y = s * (self_x - x) - self_y;
            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        if self == rhs && self_y == T::zero() * self_x {
            return Point::new(None, None, self.a, self.b);
        }

        if self == rhs {
            let one = T::one();
            let two = one + one;
            let three = two + one;

            let s = (three * self_x.pow(2_u8) + self.a) / (two * self_y);
            let x = s.pow(2_u8) * self_x;
            let y = s * (self_x - x) - self_y;
            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        panic!("Should never reach this point")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_point_on_curve() {
        Point::new(Some(-1), Some(-1), 5, 7);
    }

    #[test]
    fn eq_and_neq_works() {
        let a = Point::new(Some(-1), Some(-1), 5, 7);
        let b = Point::new(Some(-1), Some(-1), 5, 7);
        let c = Point::new(None, None, 0, 0);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    #[should_panic(expected = "(-1, -2) is not on the curve")]
    fn can_not_create_point_not_on_curve() {
        Point::new(Some(-1), Some(-2), 5, 7);
    }
}
