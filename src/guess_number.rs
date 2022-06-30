use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub(crate) fn guess_number() {
    println!();
    println!("猜数游戏");
    println!();
    // generate a random number between 1 and 100
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("猜数游戏开始！");
    println!();
    loop {
        println!("请选择游戏模式 => 1.手动猜测 2.自动使用折半查找进行猜测");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        println!();
        let choice: i32 = choice.trim().parse().expect("Please type a number");
        match choice {
            1 => {
                manual_guess_number(secret_number);
                break;
            }
            2 => {
                auto_guess_number(secret_number);
                break;
            }
            _ => {
                println!("您的输入无效，请重新输入");
            }
        }
    }
    println!();
    println!("游戏结束！");
    println!();
}

fn manual_guess_number(secret_number: i32) {
    loop {
        let mut input = String::new();
        // give value to var_input
        loop {
            println!();
            println!("请输入你的猜测：");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    println!("读取输入失败！请重新输入！");
                    continue;
                }
            }
        }
        // convert input to number
        let var_input: i32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("你没有输入一个纯数字！请重新输入！");
                continue;
            }
        };
        // match var_input with secret_number
        match var_input.cmp(&secret_number) {
            // if var_input is equal to secret_number
            Ordering::Equal => {
                println!("你猜对了！");
                break;
            }
            // if var_input is less than secret_number
            Ordering::Less => {
                println!("你猜的数字小了！请重新猜测！");
            }
            // if var_input is greater than secret_number
            Ordering::Greater => {
                println!("你猜的数字大了！请重新猜测！");
            }
        }
    }
}

fn auto_guess_number(secret_number: i32) {
    let accessibility = check_if_auto_search_valid();
    if !accessibility {
        println!();
        println!("折半查找方法有问题！自动退出程序！");
        println!();
    } else {
        println!();
        println!("折半查找方法无误！继续执行！");
        println!();
    }
    // 使用折半查找，在1到100之间查找 secret_number
    let mut low = 1;
    let mut high = 100;
    loop {
        let mid = (low + high) / 2;
        match mid.cmp(&secret_number) {
            Ordering::Equal => {
                println!("已经完成了自动猜测！数字是：{}", mid);
                break;
            }
            Ordering::Less => {
                low = mid + 1;
            }
            Ordering::Greater => {
                high = mid - 1;
            }
        }
    }
}

fn check_if_auto_search_valid() -> bool {
    for test_num in 1..101 {
        let mut _if_ever_matched: bool = false;
        let mut low = 1;
        let mut high = 100;
        loop {
            let mid = (low + high) / 2;
            match mid.cmp(&test_num) {
                Ordering::Equal => {
                    _if_ever_matched = true;
                    break;
                }
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid - 1;
                }
            }
        }
        if !_if_ever_matched {
            return false;
        }
    }
    return true;
}
