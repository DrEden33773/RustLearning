use std::io;

mod control_stream;
mod enum_example;
mod guess_number;
mod hello_world;
mod junior_struct;
mod readline_output_with_bytes;
mod senior_struct;
mod slice_example;

fn main() {
    selector();
}

fn selector() {
    println!();
    println!("1 => example_of_read_line");
    println!("2 => Hello_world");
    println!("3 => guess_number");
    println!("4 => control_stream");
    println!("5 => slice_example");
    println!("6 => junior_struct");
    println!("7 => senior_struct");
    println!("8 => enum_example");
    println!("Please enter your choice:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number");
    match choice {
        1 => readline_output_with_bytes::show_input(),
        2 => hello_world::hello_world(),
        3 => guess_number::guess_number(),
        4 => control_stream::control_stream(),
        5 => slice_example::slice_example(),
        6 => junior_struct::generate_jr_struct(),
        7 => senior_struct::senior_struct(),
        8 => enum_example::enum_example(),
        _ => {
            println!("Invalid choice")
        }
    }
}

fn test_func() {
    println!("this is a {}", "test_func");
}
