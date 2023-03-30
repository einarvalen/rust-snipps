use std::collections::HashSet;

fn main() {
    assert_eq!(true,  is_permutation(&[4,1,3,2]));
    assert_eq!(false, is_permutation(&[4,1,3]));
}

fn is_permutation(slice: &[i32]) -> bool {
    let mut set = HashSet::new();
    for i in 1..slice.len() {
        set.insert(i as i32);
    }
    for i in slice {
        set.remove(i);
    }
    set.len() == 0
}

#[test]
fn testit() {
    assert_eq!(true,  is_permutation(&[4,1,3,2]));
    assert_eq!(false, is_permutation(&[4,1,3]));
}
