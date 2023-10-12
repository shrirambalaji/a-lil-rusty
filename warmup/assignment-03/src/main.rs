// Problem: Take two arrays, merge them and find median
// both arrays can be of variable length

// eg. arr1 = [1, 2, 3] and arr2 = [4, 5, 6, 7]
// We have to find median of - [1, 2, 3, 4, 5, 6, 7] => 4

fn find_median(array_one: &[usize], array_two: &[usize]) -> f64 {
    let mut merged = [array_one, array_two].concat();
    merged.sort();
    let len = merged.len();
    let middle = len / 2;
    let mut median = merged[middle] as f64;
    if len % 2 == 0 {
        median = (merged[middle - 1] + merged[middle]) as f64 / 2.0;
    }
    median
}

fn main() {
    let median = find_median(&[1, 2, 3], &[4, 5, 6, 7]);
    assert_eq!(median, 4.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd() {
        let array_one = [1, 3, 5];
        let array_two = [2, 4, 6];
        let result = find_median(&array_one, &array_two);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_even() {
        let array_one = [1, 3, 5, 7];
        let array_two = [2, 4, 6, 8];
        let result = find_median(&array_one, &array_two);
        assert_eq!(result, 4.5);
    }

    #[test]
    fn test_empty() {
        let array_one = [];
        let array_two = [2, 4, 6];
        let result = find_median(&array_one, &array_two);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_single() {
        let array_one = [1];
        let array_two = [2];
        let result = find_median(&array_one, &array_two);
        assert_eq!(result, 1.5);
    }
}
