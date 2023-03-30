// https://app.codility.com/programmers/lessons/2-arrays/cyclic_rotation/

fn main() {
    let mut v:Vec<i32> = vec![3, 8, 9, 7, 6];
    rotate(&mut v, 3);
    assert_eq!(v, [9,7,6,3,8]);
    v = vec![1,2,3,4];
    rotate(&mut v, 4);
    assert_eq!(v, [1,2,3,4]);
    v = vec![1,2,3,4];
    rotate(&mut v, 0);
    assert_eq!(v, [1,2,3,4]);
    v = vec![1,2,3,4];
    rotate(&mut v, -1);
    assert_eq!(v, [1,2,3,4]);
    v = vec![0,0,0];
    rotate(&mut v, 1);
    assert_eq!(v, [0,0,0]);
    v = vec![1];
    rotate(&mut v, 1);
    assert_eq!(v, [1]);
    v = vec![1];
    rotate(&mut v, 0);
    assert_eq!(v, [1]);
    v = vec![1];
    rotate(&mut v, 3);
    assert_eq!(v, [1]);
    v = vec![];
    rotate(&mut v, 3);
    assert_eq!(v, []);
}

fn rotate(v:  &mut Vec<i32>, count: i32) { 
    if v.len() > 0 && count > 0 {
        for _i in 0..count {
            let r = v[v.len() - 1];
            let mut i = v.len();
            while i > 1 {
                i -= 1;
                v[i] = v[i - 1];
            }
            v[0] = r;
        }
    }
}
