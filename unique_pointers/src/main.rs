fn foo() {
    let x = Box::new(75);
    // This is the same is new in cpp, allocates memory on heap.
    // We can also write
    // let x: Box<isize> = Box::new(75);
    // which is
    // int* x = new int(75);
    
    // No need to call free or delete
    // deleted when out of scope

    // dereferencing
    println!("`x` points to {}", *x);

    // Owning pointers are immutable by default, both the value and the pointer
    let x = Box::new(75);
    let y = Box::new(42);
    // x = y; // Not allowed, x is immutable
    // *x = 43; // Not allowed, *x is immutable

    let mut x = Box::new(75);
    x = y; // OK, x is mutable
    *x = 43; // OK, *x is mutable
}

// Owning pointers can be returned from a function and continue to live on.
// if returned, their memory will not be freed
// only deleted once it goes out of scope.
fn foo2() -> Box<i32> {
    let x = Box::new(75);
    x
}

fn bar() {
    let y = foo();
    // ... use y ...
    // at the end of bar, y goes out of scope and the memory is freed
}

// Owning pointers are unique (linear), there can only be one.
// It uses move semantics, the previous pointer can no longer be accessed.
fn foo3() {
    let x = Box::new(75);
    let y = x;
    // x can no longer be acessed
    // let z = *x; // Error
}

// Similarly for passing it to another function or stored in a field.
fn bar(y: Box<isize>) {
}

fn foo4() {
    let x = Box::new(75);
    bar(x);
    // x can no longer be acessed
    // let z = *x; // Error
}

// All these errors will occur at compile time, woohoo!

// Owning pointers must be dereferenced to use their values.
// However, method calls automatically dereference, so there is no need for ->
// or * for method calls.
// Rust pointers are a bit similar to both pointers and references in C++
fn bar2(x: Box<Foo>, y: Box<Box<Box<Box<Foo>>>>) {
    x.foo();
    y.foo();
    // assuming that the type Foo has a method foo()
}

// Calling Box::new() with an existing value does not take a reference to that
// value, it copies that value.
fn foo5() {
    let x = 3;
    let mut y = Box::new(x);
    *y = 45;
    println!("x is still {}", x);
}

// Rust has move rather than copy semantics.
// Primitive types have copy semantics.
// in the example above, 3 is copied
// but for more complex values, it will be moved

fn main() {
    println!("Hello, world!");
}
