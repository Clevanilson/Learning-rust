fn main() {
    let boolean: bool = true; // boolean.
    let i8_integer: i8 = -100; // signed integer.
    let u8_integer: u8 = 100; // unsigned integer.
    /*
        Integer ranges
        8-bit   | i8   - u8;
        16-bit  | i16  - u16;
        32-bit  | i32  - u32;
        64-bit  | i64  - u64;
        128-bit | i128 - u128;
        16-bit  | i16  - u16;
        size    | based on system  usize - isize.
    */
    let character: char = 'a'; // Single quotes, allow 1 character and unicode signs.
    let float: f32 = 30.5; // Floats ranges f32 and f64.
    let tuple: (f32, i32, bool) = (45.3, 45, false);
    let array: [u32; 3] = [1, 2 , 3]; // Arrays are fix in size and type.
    println!("Boolean: bool               | {}", boolean);
    println!("Signed integer 8 bits: i8   | {}", i8_integer);
    println!("Unsigned integer 8 bits: i8 | {}", u8_integer);
    println!("Character: char             | {}", character);
    println!("Float: f32                  | {}", float);
    println!("tuple: (f32, i32, bool)     | {}, {}, {}",  tuple.0, tuple.1, tuple.2);
    println!("array: [u32; 3]             | {}, {}, {}", array[0], array[1], array[2]);

}
