use std::io;

pub fn get_first_num() -> i32 {
    let mut num_1 = String::new();
    println!("Please, enter the first number: ");
    io::stdin()
        .read_line(&mut num_1)
        .expect("Failed to read line.");

    let num_1: i32 = num_1
        .trim()
        .parse()
        .expect("Input entered was not a number.");

    num_1
}

pub fn get_second_num() -> i32 {
    let mut num_2 = String::new();

    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut num_2)
        .expect("Failed to read line.");

    let num_2: i32 = num_2
        .trim()
        .parse()
        .expect("Input entered was not a number.");
    num_2
}

pub fn get_operation() -> i32 {
    let mut operation = String::new();
    println!("Do you want to do addition(1), subtraction(2), multiplication(3) or division(4)?");
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let operation: i32 = operation
        .trim()
        .parse()
        .expect("Input entered was not valid.");

    operation
}
