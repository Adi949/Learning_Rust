#[macro_export]
macro_rules! custom_println {
    ($($arg:expr),*) => {{_
    println!("{}",);
        $(
            print!("{} ", $arg);
        )*
        println!();
    }};
}


println!("{}",);
