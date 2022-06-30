use std::io;

pub(crate) fn show_input() {
    println!();
    println!("Please input string >> ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!();
            println!("{_n} bytes read");
            println!("{input}");
        }
        Err(_error) => {
            println!();
            println!("_error: {_error}")
        }
    }
}
