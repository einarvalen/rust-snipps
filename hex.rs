#![allow(unused)]

fn main() {
use std::io;

pub fn u32_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}


    let myu32 = 0xf1f2f3f4;
    let b = u32_bytes(&myu32);
    println!("{:?}", b);
    println!("{:#01x},{:#01x},{:#01x},{:#01x}", b[0], b[1], b[2], b[3]);
    println!("{:#01x}", (myu32 >> 8 * 0x0) as u8);
    println!("{:#01x}", (myu32 >> 8 * 0x1) as u8);
    println!("{:#01x}", (myu32 >> 8 * 0x2) as u8);
    println!("{:#01x}", (myu32 >> 8 * 0x3) as u8);

}
