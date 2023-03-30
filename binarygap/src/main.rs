fn main() {
    assert_eq!(2, solution(9));
    assert_eq!(5, solution(1041));
    assert_eq!(0, solution(32));
    assert_eq!(1, solution(20));
    assert_eq!(0, solution(15));
    assert_eq!(4, solution(529));
}
 fn solution(n: i32) -> i32 {
		let mut num:u32 = n as u32;
    let base = 2u32;
		let mut gap = 0;
		let mut count = 0;
		let mut start = false;
    while num != 0 {
        let i = num % base;
				if start && i == 1 {
					start = false;
					if count > gap {
						gap = count;
					}
					count = 0;
				}
				if !start && i == 1 {
					start = true;
					count = 0;
				}
				if start && i == 0 {
					count += 1;
				}

        num /= base;
    }
		gap
 }
