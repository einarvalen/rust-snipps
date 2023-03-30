use std::collections::HashSet;

fn main() {
    assert_eq!(-1,find_shortest_time_across_river(&[1 ,3 ,1 ,4 ,2 ,3 ,2 ,4], 5));
    assert_eq!(6,find_shortest_time_across_river(&[1 ,3 ,1 ,4 ,2 ,3 ,5 ,4], 5));
}

fn find_shortest_time_across_river(slice: &[i32], a: i32) -> i32 {
    let mut set = HashSet::new();
    for i in 1..=a {
        set.insert(i);
    }
    for (i, x) in slice.iter().enumerate() {
        set.remove(x);
        if set.len() == 0 {
            return i as i32;
        }
    } 
    -1
}
#[test]
fn testit() {
    assert_eq!(-1,find_shortest_time_across_river(&[1 ,3 ,1 ,4 ,2 ,3 ,2 ,4], 5));
    assert_eq!(6,find_shortest_time_across_river(&[1 ,3 ,1 ,4 ,2 ,3 ,5 ,4], 5));
}

