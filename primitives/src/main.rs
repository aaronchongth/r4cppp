fn main() {
    let x: bool = true;
    let x = 34; // type i32, this was inferred
    // let x = 2147483648; // error: literal out of range for `i32`
    let x = 34isize;
    let x = 34usize;
    let x = 34u8;
    let x = 34i64;
    let x = 34f32;

    // Rust lets us redefine variables, so the the above code is legal
    // Each let call creates a new x and hides the previous one
   
    let x = 12;
    let x = 0b1100; // bit array
    let x = 0o14; // octal
    let x = 0xe; // hexa
    let y = 0b_1100_0011_1011_0001; // bit array
    // underscores are ignored anywhere in numeric literals
    
    let x = 34usize as isize; // cast usize to isize
    let x = 10 as f32; // isize to float
    let x = 10.45f64 as i8; // float to i8 (loses precision)
    let x = 4u8 as u64; // gains precision
    let x = 400u16 as u8; // 144, loses precision (and thus changes the value)
    println!("`400u16 as u8` gives {}", x);
    let x = -3i8 as u8; // 253, signed to unsigned (changes sign)
    println!("`-3i8 as u8` gives {}", x);
    // let x = 45 as bool; // FAILS! (use 45 != 0 instead)
    let x = true as usize; // cast bool to usize (gives a 1)

    // Using as for casting, note that you can't cast from numeric types to
    // boolean types, but the reverse is ok
    
    // Rust operators are similar to C++
    // there are no increment or decrement ++, --
    // there are compound assignment operators, += -=
    // ! negates a boolean and inverts every bit on an integer, it is ~ on C++
    // bitwise operators can only be applied to integers
    // logical operators can only be applied to booleans.
}
