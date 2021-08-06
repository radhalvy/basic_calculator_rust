mod functions;

fn main() {
    match functions::get_operation() {
        // Addition
        1 => {
            println!("{}", functions::get_first_num() + functions::get_second_num());
        }
        // Subtraction
        2 => {
            println!("{}", functions::get_first_num() - functions::get_second_num());
        }
        // Multiplication
        3 => {
            println!("{}", functions::get_first_num() * functions::get_second_num());
        }
        // Division
        4 => {
            println!("{}", functions::get_first_num() / functions::get_second_num());
        }
        _ => {
            println!("Invalid operation.")
        }
    }
    
        
}
