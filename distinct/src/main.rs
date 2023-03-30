use std::collections::HashSet;

fn main() {
    println!("{}", distinct(&[2,1,1,2,3,1]));
}

fn distinct(ary : &[i32]) -> usize {
    let mut set = HashSet::new();
    for i in ary {
        set.insert(*i);
    }
    set.len()
}

#[test]
fn testit() {
    assert_eq!(3, distinct(&[2,1,1,2,3,1]));
    assert_eq!(1, distinct(&[2]));
    assert_eq!(0, distinct(&[]));
}
