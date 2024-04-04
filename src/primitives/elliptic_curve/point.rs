use num::{pow::Pow, One, Zero};
use std::fmt::Display;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Error<T> {
    InvalidPoint,
    NotOnCurve(T, T),
}

impl<T: Number> Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::InvalidPoint => "Invalid point".into(),
            Self::NotOnCurve(x, y) => format!("({:?}, {:?}) is not on the curve", x, y),
        };
        write!(f, "{}", s)
    }
}

impl<T: Number> std::error::Error for Error<T> {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Point<T> {
    Point { a: T, b: T, x: T, y: T },
    Infinite { a: T, b: T },
}

impl<T: Number> Eq for Point<T> {}

impl<T: Number> Point<T> {
    #[allow(dead_code)]
    // TODO: Remove #[allow(dead_code)] once it's being used.
    pub fn new(x: Option<T>, y: Option<T>, a: T, b: T) -> Result<Point<T>, Error<T>> {
        match (x, y) {
            (None, None) => Ok(Point::Infinite { a, b }),
            (Some(x), Some(y)) => {
                let point = Point::Point { a, b, x, y };
                if !point.is_valid(false) {
                    return Err(Error::NotOnCurve(x, y));
                }
                Ok(point)
            }
            _ => return Err(Error::InvalidPoint),
        }
    }

    // Checks if a non-infinite point is on the curve, thus valid
    //
    // When panic is set to true it will raise a panic if the point is
    // invalid.
    fn is_valid(&self, panic: bool) -> bool {
        match self {
            Self::Point { a, b, x, y } => {
                if y.pow(2_u8) != x.pow(3_u8) + *a * *x + *b {
                    if panic {
                        panic!("({:?}, {:?}) is not on the curve", x, y)
                    }
                    return false;
                }
            }
            _ => {}
        }
        true
    }

    // Should be called first by any operation.
    // This is to ensure that no operations are done with invalid points.
    //
    // Point should never be invalid if made via the new constructor, but
    // self constructed points could theoretically be invalid.
    fn __ensure_valid(&self) {
        if !self.is_valid(true) {}
    }
}

impl<T: Number> std::ops::Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.__ensure_valid();

        match (self, rhs) {
            (Self::Infinite { a, b }, Self::Infinite { a: _, b: _ }) => Self::Infinite { a, b },
            (
                Self::Infinite { a: _, b: _ },
                Self::Point {
                    a: _,
                    b: _,
                    x: _,
                    y: _,
                },
            ) => rhs,
            (
                Self::Point {
                    a: _,
                    b: _,
                    x: _,
                    y: _,
                },
                Self::Infinite { a: _, b: _ },
            ) => self,
            (
                Self::Point { a, b, x, y },
                Self::Point {
                    a: _,
                    b: _,
                    x: rhs_x,
                    y: rhs_y,
                },
            ) if x == rhs_x && y != rhs_y => Self::Infinite { a, b },
            (
                Self::Point { a, b, x, y },
                Self::Point {
                    a: _,
                    b: _,
                    x: rhs_x,
                    y: rhs_y,
                },
            ) if x == rhs_x && (y == T::zero() || rhs_y == T::zero()) => Self::Infinite { a, b },
            (
                Self::Point { a, b, x, y },
                Self::Point {
                    a: _,
                    b: _,
                    x: rhs_x,
                    y: rhs_y,
                },
            ) if x == rhs_x && y == rhs_y => {
                let one = T::one();
                let two = one + one;
                let three = two + one;

                let s = (three * x.pow(2_u8) + a) / (two * y);
                let new_x = s.pow(2_u8) * x - rhs_x;
                let new_y = s * (x - new_x) - y;
                Point::Point {
                    a,
                    b,
                    x: new_x,
                    y: new_y,
                }
            }
            (
                Self::Point { a, b, x, y },
                Self::Point {
                    a: _,
                    b: _,
                    x: rhs_x,
                    y: rhs_y,
                },
            ) if x != rhs_x => {
                let s = (rhs_y - y) / (rhs_x - x);
                let new_x = s.pow(2) - x - rhs_x;
                let new_y = s * (x - new_x) - y;
                Point::Point {
                    a,
                    b,
                    x: new_x,
                    y: new_y,
                }
            }
            _ => panic!("Should never reach this point"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_point_on_curve() {
        Point::new(Some(-1), Some(-1), 5, 7).unwrap();
    }

    #[test]
    fn eq_and_neq_works() {
        let a = Point::new(Some(-1), Some(-1), 5, 7).unwrap();
        let b = Point::new(Some(-1), Some(-1), 5, 7).unwrap();
        let c = Point::new(None, None, 0, 0).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn can_not_create_point_not_on_curve() {
        assert_eq!(
            Point::new(Some(-1), Some(-2), 5, 7),
            Err(Error::NotOnCurve(-1, -2))
        );
    }
}
