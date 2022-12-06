use std::mem::size_of;

fn main() {
    let size_of_type = size_of::<char>();

    println!("{}", size_of_type)
}

// -2^(n-1) to 2^(n-1) - 1
// 0 to 2^(2n) - 1
// -2^(32-1) to 2^(32-1) - 1
/*
i8      ->  u8      -> 8 bits   -> 1 byte   -> -128 to 127
i16     ->  u16     -> 16 bits  -> 2 bytes
i32     ->  u32     -> 32 bits  -> 4 bytes
i64     ->  u64     -> 64 bits  -> 8 bytes
i128    ->  u128    -> 128 bits -> 16 bytes 

f32
f64

1 byte = 8 bits
2 bytes = 16 bits of memory
*/

