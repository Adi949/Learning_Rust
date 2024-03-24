use crate::functions::helper::readline::read_line;

use super::new_vector;

pub fn create_vector() {
    let input = read_line();

    match input.trim().parse::<i32>() {
        Ok(_) => {
            new_vector::iterate(input.trim().parse().unwrap());

            // Do something with the parsed integer if needed
        }
        Err(_) => {
            println!("You entered a string: {}", input.trim());
            // Do nothing for strings
        }
    }
}
