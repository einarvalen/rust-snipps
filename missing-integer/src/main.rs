use std::collections::HashSet;

/// FInd smallest positive integer not in the array
fn find_smallest_positive_integer_missing(ary: &[i32]) -> i32{
    let mut set = HashSet::new();
    for i in ary {
        if *i > 0 {
            set.insert(*i);
        }
    }
    let mut missing: i32 = 1;
    loop {
        if !set.contains(&missing) {
            return missing;
        }
        missing += 1;
    }
}


fn main() {
    assert_eq!(5, find_smallest_positive_integer_missing(&[1, 3, 6, 4, 1, 2]));
    assert_eq!(4, find_smallest_positive_integer_missing(&[1, 2, 3]));
    assert_eq!(1, find_smallest_positive_integer_missing(&[-1,-3]));
}

#[test]
fn testit() {
    assert_eq!(5, find_smallest_positive_integer_missing(&[1, 3, 6, 4, 1, 2]));
    assert_eq!(4, find_smallest_positive_integer_missing(&[1, 2, 3]));
    assert_eq!(1, find_smallest_positive_integer_missing(&[-1,-3]));
}
