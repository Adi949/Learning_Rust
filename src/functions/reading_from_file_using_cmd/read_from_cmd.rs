use std::env;

#[derive(Debug)]
pub struct Config {
    pub file: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return Err("invalid params");
        }
        let file = &args[1];
        Ok(Config {
            file: file.to_string(),
        })
    }
}
