pub struct FieldElement<T>(T, T);

impl<T> FieldElement<T> 
where T: Default + std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug + std::ops::Add + std::ops::Sub {
    pub fn new(num:T, prime:T) -> FieldElement<T> {
        if num >= prime || num < T::default() {
            panic!("Num {:?} not in field range 0 to {:?}", num, prime);
        }

        FieldElement(num, prime)
    }
}

impl<T> From<T> for FieldElement<T> 
where T: Default + Clone + std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug + std::ops::Add + std::ops::Sub {
    fn from(value: T) -> Self {
        FieldElement::new(value.clone(), value)
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

impl<T> std::ops::Add for FieldElement<T> 
where T: Default + std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Debug + std::ops::Add<Output = T> + std::ops::Sub<Output = T> {
    type Output = FieldElement<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let prime = match self.1 >= rhs.1 {
            true => self.1,
            false => rhs.1,
        };

        let num = self.0 + rhs.0;

        FieldElement::new(num, prime)
    }
}

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
        let b = FieldElement::new(10, 10);

        let result = a + b;
        assert_eq!(result, 20.into());
    }
}