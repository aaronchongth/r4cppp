fn foo(_x: &'static str) -> &'static str {
    "world"
    // if the last statement does not have a semicolon, it is used as the return
    // return "world";
}

fn main() {
    // let world = "world";
    // let world: &'static str = "world";
    // println!("Hello {}!", world);
    
    println!("Hello {}!", foo("bar"));
}
