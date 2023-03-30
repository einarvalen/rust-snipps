fn main() {
    println!("{}",min_avg_two_slice(&[4,2,2,5,1,5,8]));
}

fn min_avg_two_slice(ary: &[i32]) -> usize {
    let mut min_avg:f64 = f64::MAX;
    let mut min_avg_idx = 1;
    let mut i = 1;
    while i < ary.len() {
        let avg:f64 = f64::from(ary[i-1] + ary[i]) / 2.0;
        if avg < min_avg {
            min_avg = avg;
            min_avg_idx = i - 1;
        }
        i += 1;
    }
    min_avg_idx
}

#[test]
fn testit() {
    assert_eq!(1, min_avg_two_slice(&[4,2,2,5,1,5,8]));
    assert_eq!(5, min_avg_two_slice(&[4,2,2,5,8,1,1]));
    assert_eq!(3, min_avg_two_slice(&[-4,-2,-2,-5,-8,-1,-1]));
    assert_eq!(4, min_avg_two_slice(&[-4,-2,-2,-5,-1,-5,-8]));
}
