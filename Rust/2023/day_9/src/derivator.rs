pub fn get_derived_function(pattern: Vec<i32>) -> Box<dyn Fn(usize) -> i32> {
    if pattern.iter().all(|x| x == &0) {
        return Box::new(|_| 0);
    }

    let new_pattern = pattern
        .windows(2)
        .map(|y| y[1] - y[0])
        .collect::<Vec<i32>>();

    Box::new(move |x| {
        let derived = get_derived_function(new_pattern.clone());
        let pattern = pattern.clone();
        let last_index = pattern.len();
        let last = *pattern.last().unwrap();
        if x < last_index {
            return pattern[x];
        }

        let mut new_value = last;
        for i in last_index..=x {
            new_value += derived(i - 1);
        }

        new_value
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_derive() {
        let v = vec![0_i32, 0_i32, 0_i32];
        let func = get_derived_function(v);
        let result = func(1);
        assert_eq!(result, 0);
        let result = func(5);
        assert_eq!(result, 0);
        let v = vec![0_i32, 1_i32, 2_i32, 3_i32];
        let func = get_derived_function(v);
        let result = func(1);
        assert_eq!(result, 1);
        let result = (0..10).map(func).collect::<Vec<i32>>();
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let v = vec![1_i32, 2_i32, 4_i32, 7_i32];
        let func = get_derived_function(v);
        let result = (0..10).map(func).collect::<Vec<i32>>();
        assert_eq!(result, vec![1, 2, 4, 7, 11, 16, 22, 29, 37, 46]);
    }
}
