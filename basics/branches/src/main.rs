fn main() {
    let number = 90;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number not divisible by 2, 3, or 4");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("Number: {}", number)
}
