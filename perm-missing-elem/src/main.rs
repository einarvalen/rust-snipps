fn main() {
    assert_eq!(4, find_missing_element(&[3,2,1,5]));
}

#[test]
fn testit() {
    assert_eq!(4, find_missing_element(&[3,2,1,5]));
}

fn find_missing_element(ary: &[i64]) -> i64 {
    let mut sum_ary:i64 = 0;
    let mut sum:i64 = 0;
    let mut i = 0;
    let len = ary.len();
    loop {
        let idx = i as i64;
        if i < len {
            sum_ary += ary[i];
            sum += idx;
        } else {
            sum += idx + idx + 1;
            break;
        }
        i += 1;
    }
    return sum - sum_ary;
}
