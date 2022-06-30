use std::io;
use std::process::exit;
use std::vec;

pub(crate) fn control_stream() {
    selector();
}

fn selector() {
    println!();
    println!("1 => official");
    println!("2 => fibonacci");
    println!("3 => Yanghui_triangle");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number");
    match choice {
        1 => {
            official();
        }
        2 => {
            fibonacci();
        }
        3 => {
            yanghui_triangle();
        }
        _ => {
            println!("Invalid choice")
        }
    }
}

fn official() {
    let y = {
        let x = 3;
        x + 1
    };
    println!();
    println!("The value of y is: {}", y);
    println!();
}

fn fibonacci() {
    println!();
    println!("Give a number for N (N > 0) => ");
    let mut given_num = String::new();
    io::stdin()
        .read_line(&mut given_num)
        .expect("Failed to read line");
    let final_num: i32 = given_num.trim().parse().expect("Failed to parse");
    if final_num <= 0 {
        println!("invalid input, exit ... ");
        exit(-1);
    }
    println!();
    println!("{} => faster method without vec", better_fib(final_num));
    println!("{} => faster method with vec", faster_fib(final_num));
    if final_num <= 30 {
        println!("{} => original method", fib(final_num))
    } else {
        println!(
            "N:{} >= 30, so original method will not be called",
            final_num
        );
    }
    println!();
}

fn fib(input: i32) -> i32 {
    if input == 1 || input == 2 {
        return 1;
    }
    return fib(input - 1) + fib(input - 2);
}

fn faster_fib(input: i32) -> i32 {
    if input == 1 || input == 2 {
        return 1;
    }
    let mut _2 = vec![1, 1];
    let mut _1 = vec![2, 1];
    let mut _0 = vec![3, 2];
    loop {
        if _0[0] == input {
            break;
        }
        let current = vec![_0[0], _0[1]];
        _2 = _1;
        _1 = _0;
        _0 = current;
        _0[0] += 1;
        _0[1] = _1[1] + _2[1];
    }
    return _0[1];
}

fn better_fib(input: i32) -> i32 {
    if input == 1 || input == 2 {
        return 1;
    }
    let mut _2 = 1;
    let mut _1 = 1;
    let mut _0 = 2;
    let mut current_num = 3;
    // ATTENTION !!! >=> Could not replace `loop` with `for`
    // for current_num++ operated at first instead of in the end
    loop {
        if current_num == input {
            break;
        }
        current_num += 1;
        _2 = _1;
        _1 = _0;
        _0 = _1 + _2;
    }
    return _0;
}

fn yanghui_triangle() {
    println!();
    println!("Give a number for N (N >= 0) => ");
    let mut given_num = String::new();
    io::stdin()
        .read_line(&mut given_num)
        .expect("Failed to read line");
    let final_num: i32 = given_num.trim().parse().expect("Failed to parse");
    if final_num < 0 {
        println!("invalid input, exit ... ");
        exit(-1);
    }
    println!();
    yanghui_tri(final_num + 1);
}

fn yanghui_tri(input_num: i32) {
    let mut _reference_vec = vec![1];
    let mut current_vec = vec![1];
    for current_num in 1..=input_num {
        // output current vector
        for element in &current_vec {
            print!("{} ", element);
        }
        println!();
        if input_num == 1 {
            break;
        }
        // pre_operation for the next time loop

        // in this case, `current_num` should += 1 while used as indexer
        //=> we could create a explicit indexed
        let pre_operation_num = current_num + 1;

        // move current_vec to reference_vec
        _reference_vec = current_vec;
        current_vec = Vec::new();
        // build new current_vec
        current_vec.push(1);
        for index in 1..pre_operation_num - 1 {
            current_vec.push(_reference_vec[index as usize] + _reference_vec[index as usize - 1]);
        }
        current_vec.push(1);
        // continue the loop
    }
}
