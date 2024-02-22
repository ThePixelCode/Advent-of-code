#[derive(Debug, Clone)]
pub struct Derived {
    derived: Option<Box<Derived>>,
    pattern: Vec<i32>,
}
impl Derived {
    pub fn new(pattern: Vec<i32>) -> Self {
        if pattern.iter().all(|x| x == &0) {
            return Self {
                derived: None,
                pattern,
            };
        }

        let derived_pattern = pattern
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect::<Vec<i32>>();

        Self {
            derived: Some(Box::new(Self::new(derived_pattern))),
            pattern,
        }
    }

    pub fn forward_one(&self) -> i32 {
        match &self.derived {
            None => 0,
            Some(s) => *self.pattern.last().unwrap() + s.forward_one(),
        }
    }

    pub fn backward_one(&self) -> i32 {
        match &self.derived {
            None => 0,
            Some(s) => *self.pattern.first().unwrap() - s.backward_one(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_derive() {
        let v = vec![0_i32, 0_i32, 0_i32];
        let func = Derived::new(v);
        let result = func.forward_one();
        assert_eq!(result, 0);
        let v = vec![0_i32, 1_i32, 2_i32, 3_i32];
        let func = Derived::new(v);
        let result = func.forward_one();
        assert_eq!(result, 4);
        let v = vec![1_i32, 2_i32, 4_i32, 7_i32];
        let func = Derived::new(v);
        let result = func.forward_one();
        assert_eq!(result, 11);
    }
}
