fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Main function.");

    another_function(5);
    print_labeled_measurement(10, 'h');

    let x: i32 = five();
    println!("The value of x is {x}");

    let y: i32 = plus_one(5);
    println!("The value of y is {y}");
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}