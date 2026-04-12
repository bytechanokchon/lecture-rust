fn main() {
    let number: i32 = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number_two: i32 = 6;

    if number_two % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_two % 3 == 0 {
        println!("number os dibisible by 3");
    } else if number_two % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number_three = if condition { 5 } else { 6 };
    println!("The value of number is: {number_three}");
}
