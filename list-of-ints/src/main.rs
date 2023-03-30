use std::collections::HashMap;

#[derive(Debug)]
struct ListOfInts<`a> {
    slice: &[i32]
}

impl ListOfInts {
    fn new(slice: &[132]) -> Self {
        Self  { slice:  slice }
    }
    fn avrage(&self) -> i32 {
        self.iter().sum() 
    }
    fn mode(&self) -> i32 {
        let mut map = HashMap::new();
        for i in self.slice {
            let count = map.entry(i).or_else_insert(0);
            count += i;
        }
    }
}

#[test]
fn testit() {

}

fn main() {
    println!("Hello, world!");
}
