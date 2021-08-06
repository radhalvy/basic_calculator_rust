mod functions;

fn main() {
    match functions::get_operation() {
        // Addition
        1 => {
            println!("{}", functions::addition(functions::get_first_num(), functions::get_second_num()));
        }
        // Subtraction
        2 => {
            println!("{}", functions::subtraction(functions::get_first_num(), functions::get_second_num()));
        }
        // Multiplication
        3 => {
            println!("{}", functions::multiplication(functions::get_first_num(),functions::get_second_num()));
        }
        // Division
        4 => {
            println!("{}", functions::division(functions::get_first_num(), functions::get_second_num()));
        }
        _ => {
            println!("Invalid operation.")
        }
    }
    
        
}
