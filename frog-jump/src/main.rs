const MAX_DISTANCE: u32 = 1_000_000_000;

fn main() {
    assert_eq!(3, frog_jumps(10, 85, 30));
}

fn frog_jumps(position_x: u32, position_y: u32, distance_per_jump: u32) -> u32 {
        if !(position_x <= position_y && position_y <= MAX_DISTANCE && distance_per_jump <= MAX_DISTANCE && distance_per_jump > 0) {
        panic!("Parameters out of bounds");
        }
    let distance_to_jump = position_y - position_x;
    return match distance_to_jump % distance_per_jump == 0 {
        true => distance_to_jump / distance_per_jump,
        false => distance_to_jump / distance_per_jump + 1,
    }
}

#[test]
fn testit() {
    assert_eq!(3, frog_jumps(10, 85, 30));
    assert_eq!(0, frog_jumps(10, 10, 1));
}

#[test]
#[should_panic]
fn test_panic() {
    assert_eq!(0, frog_jumps(0, 0, 0));
}
