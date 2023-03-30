use std::collections::HashSet;

fn main() {
    assert_eq!(7, find_odd_occurrence(&[9,3,9,3,9,9,7]).unwrap());
    assert_eq!(Err("Not a single odd value in array!"), find_odd_occurrence(&[9,3,9,3,9,9]));
}

fn find_odd_occurrence(slice: &[i32]) -> Result<i32, &'static str> {
    let mut set = HashSet::new();
    for i in slice {
        if set.contains(i) {
            set.remove(i);
        } else {
            set.insert(i);
        }
    }
    return match set.len() == 1 {
        true => {
            let mut odd = 0;
            for i in set {
                odd = *i;
            }
            Ok(odd)
        },
        false => Err("Not a single odd value in array!"),
    }
}

#[test]
fn testit() {
    assert_eq!(7, find_odd_occurrence(&[9,3,9,3,9,9,7]).unwrap());
    assert_eq!(9, find_odd_occurrence(&[9,9,9,9,9,9,9]).unwrap());
    assert_eq!(9, find_odd_occurrence(&[9]).unwrap());
    assert_eq!(Err("Not a single odd value in array!"), find_odd_occurrence(&[9,3,9,3,9,9]));
    assert_eq!(Err("Not a single odd value in array!"), find_odd_occurrence(&[]));
}
