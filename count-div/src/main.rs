fn main() {
    println!("{}", count_div(6, 11, 2));
    println!("{}", count_div(-2, 4, 2));
}


/// Compute number of integers divisible by k in range [a..b]. 
fn count_div(a: i64, b: i64, k: i64) -> i64 {
    let mut count: i64 = 0;
    for i in a..=b {
        if i % k == 0 {
            count += 1;
        }
    }
    count
}

#[test]
fn testit() {
    assert_eq!(3, count_div(6,11,2));
    assert_eq!(4, count_div(-2, 4, 2))
}
