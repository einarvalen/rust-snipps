fn main() {
    assert_eq!(1, min_equilibrium(&[3, 1, 2, 4, 3]));
}

fn min_equilibrium(slice: &[i32]) -> i32 {
    // sum all elements into S
    let mut sum = 0;
    for i in slice {
        sum += i;
    }
    // loop from start and add elements into A
    let mut a = 0;
    let mut prev_calc = sum;
    for i in slice {
        // finish looping when current calulation of S - A - A > previous same calculation
        a += i;
        let calc = i32::abs(sum - a - a);
        println!(
            "sum={}, i={}, a={}, prev_calc={}, calc={}",
            sum, i, a, prev_calc, calc
        );
        if calc > prev_calc {
            // return smallest calculation of S - A - A (previous)
            return prev_calc;
        }
        prev_calc = calc;
    }
    prev_calc
}

#[test]
fn testit() {
    assert_eq!(1, min_equilibrium(&[3, 1, 2, 4, 3]));
}
