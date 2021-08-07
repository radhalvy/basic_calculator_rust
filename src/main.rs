mod functions;

fn main() {
    match functions::get_operation() {
        // Addition
        1 => {
            println!("The answer is {}", functions::get_first_num() + functions::get_second_num());
        }
        // Subtraction
        2 => {
            println!("The answer is {}", functions::get_first_num() - functions::get_second_num());
        }
        // Multiplication
        3 => {
            println!("The answer is {}", functions::get_first_num() * functions::get_second_num());
        }
        // Division
        4 => {
            println!("The answer is {}", functions::get_first_num() / functions::get_second_num());
        }
        _ => {
            println!("Invalid operation.")
        }
    }
    
        
}
