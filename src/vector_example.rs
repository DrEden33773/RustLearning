use std::io;

pub(crate) fn vector_example() {
    println!();
    println!("This is a new vector example ... ");
    println!();
    println!("1 => print_each_element");
    println!("2 => store_enum_in_vec");
    println!("3 => compare_operator_and_get_method");
    println!("4 => reference_borrow_rule_of_vec");
    println!("Please input your choice >> ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number");
    println!();
    match choice {
        1 => print_each_element(),
        2 => store_enum_in_vec(),
        3 => compare_operator_and_get_method(),
        4 => reference_borrow_rule_of_vec(),
        _ => {
            println!("Invalid choice")
        }
    }
}

fn print_each_element() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    for i in &v {
        print!("{} ", i);
    }
    println!();
    for i in &mut v {
        let res = *i + 1; // i: &mut i32 => must use * to dereference!
        print!("{} ", res);
    }
    println!();
}

fn store_enum_in_vec() {
    #[derive(Debug)]
    enum SelfDefinedEnum {
        A(String),
        B(isize),
        C(f64),
    }
    println!("vec_without_macro >>");
    let mut vec_without_macro: Vec<SelfDefinedEnum> = Vec::new();
    vec_without_macro.push(SelfDefinedEnum::A(String::from("a")));
    vec_without_macro.push(SelfDefinedEnum::B(1));
    vec_without_macro.push(SelfDefinedEnum::C(2.0));
    for element in &vec_without_macro {
        println!("{:?} ", element);
    }
    println!();
    println!("vec_with_macro >>");
    let vec_without_macro = vec![
        SelfDefinedEnum::A(String::from("a")),
        SelfDefinedEnum::B(1),
        SelfDefinedEnum::C(2.0),
    ];
    for element in &vec_without_macro {
        println!("{:?} ", element);
    }
    println!();
}

fn compare_operator_and_get_method() {
    let vec = vec![1, 2, 3];
    let get_method_returned = vec.get(3);
    match get_method_returned {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
    let operator_returned = vec[3]; //=> paniced
    println!("{}", operator_returned);
}

fn reference_borrow_rule_of_vec() {
    let mut _v = vec![1, 2, 3, 4, 5];
    // let first = &_v[0];
    // _v.push(6);
    // println!("The first element is: {}", first);
    println!("Avoid Error!");
    println!();
}
