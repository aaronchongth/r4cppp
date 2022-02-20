// Borrowed pointers

fn foo() {
    // let x = &3; // type: &i32 // creates a borrowed reference to i32
    // let y = *x; // 3, type: i32 // dereferences it to type i32
    // bar(x, *x);

    // The & operators does not allocate memory, if a borrow reference goes out
    // of scope, no memory gets deleted.
    // borrowed references are not unique, you can have multiple borrowed
    // references pointing to the same value.
    let x = 5; // i32
    let y = &x; // &i32
    let z = y; // &i32
    let w = y; // &i32
    println!("These should all be 5: {} {} {} {}", x, *w, *y, *z);
}

fn bar(z: &i32, i: i32) {
    println!("those are {}, {}", z, i);
}

// like values, borrowed references are immutable by default. you can also use
// &mut to take a mutable referenec, or to denote mutable reference types.
//
// Mutable borrowed references are unique (you can only take a single mutable
// reference to a value, and you can only have a mutable reference if there are
// no immutable references).
//
// You can use a mutable reference where an immutable one is wanted, but not
// vice versa.
//
// Putting all that together in an example:

fn bar2(x: &i32) {
    println!("bar2 called: {}", *x);
}

fn bar_mut(x: &mut i32) {
    println!("bar_mut called: {}", *x);
}

fn foo2() {
    let x = 5;

    // Error - can't make a mutable reference to an immutable variable
    // let xr = &mut x;
    
    // Ok - creates an immutable ref
    let xr = &x;
    bar2(xr);
    // bar_mut(xr); // Error - expects a mutable ref

    let mut x = 5;
    let xr = &x; // Ok - creates an immutable ref
    // *xr = 4; // Error - mutating immutable ref

    // Error - there is already an immutable ref, so we can't make a mutable one
    let xrr = &mut x;
    *xrr = 6;
    // let new_mut = &mut x;
    // *new_mut = 6;

    let mut x = 5;
    let xr = &mut x; // Ok - creates a mutable ref
    *xr = 4; // Ok
    // let xr2 = &x; // Error - there is already a mutable ref, so we can't make
    // an immutable one

    // // let xr = &mut x; // Error - can only have one mutable ref at a time
    // bar2(xrr);
    bar2(xr); // ok
    bar_mut(xr); // ok
}

fn test_immutability() {
    let x = 5;
    let immut_x = &x;
    // let mut_x = &mut x; // this is an error, makes sense
    println!("run test_immutability");

    let mut y = 6;
    let immut_y = &y;
    // *immut_y = 7; // this is an error, cannot dereference and write
    println!("printing dereference immut_y: {}", *immut_y);

    let mut_y = &mut y; // this should not be allowed since immut_y has already
    // borrowed
    *mut_y = 7;
    // println!("printing dereference mut and immut: {}, {}", *mut_y, *immut_y);
    // using mut_y in any way makes it call the error that we are borrowing
    // mutable y while an immutable borrow of y has already occurred.
}

fn test_borrowing_mut() {
    let mut x = 5;
    {
        let y = &x; // borrowed
        // x = 4; // Error, x has been borrowed
        // println!("y is {}", y); // the line above is compiled only when it is
        // used.
        println!("{}", x);
    }
    x = 6;
}

fn test_borrow_go_out_of_scope() {
    let mut x = 5;
    {
        let y = &mut x;
        println!("y is {}", *y);
        x = 4;
        println!("{}", x);
    }
    x = 4;
    println!("{}", x);
}

fn main() {
    println!("Hello, world!");
    foo();
    foo2();
    test_immutability();
    test_borrowing_mut();
    test_borrow_go_out_of_scope();
}
