use std::io::{self, stdin};

pub fn read_line() -> String {
    let mut x: String = String::new();
    match io::stdin().read_line(&mut x) {
        Ok(_) => x.trim().to_string(),
        Err(error) => {
        
            format!("error in reading the value{}", error)
        
        }
    }
}
