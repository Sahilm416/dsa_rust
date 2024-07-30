//These are scalar data types in rust

fn main() {
    /*
    Integers in rust
    */
    // 1. Signed integers (i8 - i64 - i128)
    let num_1: i32 = 77; //i32 has signed range from -2^31 to +2^31 - 1 (for sign bit)
    println!("Signed Integer is {}", num_1);

    // 2. Unsigned integers (u8 - u64 - u128)
    let num_2: u32 = 66;
    println!("Unsigned Integer is {}", num_2); //u32 has range up to 2^32 - 1

    /*
    Floating points in rust
    */
    // 1. f32
    let num_3: f32 = 22.90; // default float is f64
    println!("Float Integer is {}", num_3);

    /*
    Booleans in rust
    */
    let is_true: bool = true;
    println!("Boolean is {}", is_true);

    /*
    Character in rust
    */
    let char_1: char = 'C';
    println!("char is {}", char_1);
}
