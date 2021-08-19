fn foo(x: i32) -> &'static str {
    let mut result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

fn bar(x: i32) -> &'statuc str {
    if x < 10 {
        "less than 10"
    } else {
        "10 or more"
    }
}

fn main() {
    let mut x = 10;
    while x > 0 {
        println!("Current value: {}", x);
        x -= 1;
    }

    // there is no `loop` statement
    /
}
