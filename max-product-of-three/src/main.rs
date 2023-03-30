#[derive(Debug)]
struct Store {
    negative_count: i32,
    big: i32, bigger: i32, biggest: i32,
    smaller: i32, smallest: i32,
}

impl Store {
    fn new() -> Self {
        Self {
            negative_count: 0,
            big: i32::min_value(), 
            bigger: i32::min_value(), 
            biggest: i32::min_value(),
            smaller: 0,
            smallest: 0,
        }
    }

    pub fn max_product_of_three(ary: &[i32]) -> Result<i32, &'static str> {
        match ary.len() {
            1 | 2 => Err("Argument ary must have 3 or more elements"),
            3 => Ok(ary[0] * ary[1] * ary[2]),
            _ => {
                let mut store = Store::new();
                for i in ary {
                    store.find_three_largest(*i);
                }
                println!("Store: {:?}", store);
                Ok(store.calc())
            }
        }
    }

    fn find_three_largest(&mut self, i: i32) {
        self.find_two_smallest_negatives(i);
        if i > self.biggest {
            self.big = self.bigger;
            self.bigger = self.biggest;
            self.biggest = i;
        } else if i > self.bigger {
            self.big = self.bigger;
            self.bigger = i;
        } else if i > self.big {
            self.big = i;
        }
    }

    fn find_two_smallest_negatives(&mut self, i: i32) {
        if i < 0 {
            self.negative_count += 1;
            if i < self.smallest{
                self.smaller = self.smallest;
                self.smallest = i;
            } else if i < self.smaller {
                self.smaller = i;
            }
        }
    }

    fn largest_of(a: i32, b: i32) -> i32 {
        if a > b {a} else {b}
    }

    fn calc(self) -> i32 {
        if self.negative_count >= 2 {
            Store::largest_of(self.big * self.bigger * self.biggest, self.smaller * self.smallest * self.biggest)
        } else {
            self.big * self.bigger * self.biggest
        }
    }
}

fn main() {
    println!("{}", Store::max_product_of_three(&[-3,-1,-2,-2,-5,-6]).unwrap()); // Expect -3 *- 2 * -1 = -6
}

#[test]
fn testit() {
    assert_eq!(60, Store::max_product_of_three(&[-3,1,2,-2,5,6]).unwrap());
    assert_eq!(90, Store::max_product_of_three(&[3,-1,-2,-2,-5,-6]).unwrap());
    assert_eq!(90, Store::max_product_of_three(&[3,1,2,2,-5,-6]).unwrap());
    assert_eq!(-4, Store::max_product_of_three(&[-3,-1,-2,-2,-5,-6]).unwrap());
    assert_eq!(0, Store::max_product_of_three(&[-3,-1,-2,-2,-5,0]).unwrap());
    assert_eq!(-6, Store::max_product_of_three(&[-3,-1,-2]).unwrap());
    assert_eq!(6, Store::max_product_of_three(&[3,-1,-2]).unwrap());
    assert_eq!(6, Store::max_product_of_three(&[3,1,2]).unwrap());
}
