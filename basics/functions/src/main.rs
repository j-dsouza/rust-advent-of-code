fn main() {
    println!("Hello, world!");

    let x = another_function(5, 6);

    println!("The value of x is: {}", x);

    let n = plus_one(10);

    println!("The value of n is: {}", n)
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    x
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
