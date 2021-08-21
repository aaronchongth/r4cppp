// These 2 functions are essentially doing the same thing
fn foo(x: i32) -> &'static str {
    let mut result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

// But this function is more Rust style.
fn bar(x: i32) -> &'static str {
    if x < 10 {
        "less than 10"
    } else {
        "10 or more"
    }
}

fn print_all(all: Vec<i32>) {
    for a in all.iter() {
        println!("{}", a);
    }

    // if we want to loop over the indices
    for i in 0..all.len() {
        println!("{}: {}", i, all[i]);
    }

    // Use of an enumerating iterator
    for (i, a) in all.iter().enumerate() {
        println!("{}: {}", i, a);
    }
}

// This function uses mutable iterators, and the vector was passed in
// as a mutable reference too.
// The dereference is similar to c++, from the iterator.
fn double_all(all: &mut Vec<i32>) {
    for a in all.iter_mut() {
        *a += *a;
    }
}

// Switch/match
fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        y => println!("x is something else {}", y), // last comma is optional
    }

    // the inner x is declared and masks the argument
    match x {
        x => println!("x is something else {}", x)
    }

    // We can use _ for an unnamed variable, which is like a wildcard match
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        _ => {}
    }

    // Unlike switch statements, there is no fall through from one arm to the
    // next, so it is more like if ... else
    match x {
        0 | 1 | 10 => println!("x is one of zero, one or ten"),
        y if y < 20 => println!("x is less than 20, but not zero, one, or ten"),
        y if y == 200 => println!("x is 200 (but this is not very stylish)"),
        _ => {}
    }

    // Just like 'if' expressions, 'match' statements are actually expressions
    // too, so we can re-write the last example as
    let msg = match x {
        0 | 1 | 10 => "one of zero, one or ten",
        y if y < 20 => "less than 20, but not zero, one, or ten",
        y if y == 200 => "200 (but this is not very stylish)",
        _ => "something else"
    }; // this has an ending semicolon because let msg =...; is a statement.
    println!("x is {}", msg);
}

fn main() {
    foo(20);
    bar(21);

    let mut x = 10;
    while x > 0 {
        println!("Current value: {}", x);
        x -= 1;
    }

    // loop statement that loops forever
    // loop {
    //     println!("Just looping");
    // }
    
    // Rust has break and continue too

    print_some(200);
}
