use std::io;

struct JrStruct {
    name: String,
    // attention: this type shouldn't be &str
    // because we want the whole struct to hold the ownership of it's data
    // and this could avoid the problem caused by incomplete lifetime
    age: u8,
}

// attention:
// struct in rust-lang,
// could not be given value during declaration

impl JrStruct {
    /// Creates a new [`jr_struct`].
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
    /// Returns the name of the [`jr_struct`].
    fn get_name(&self) -> &str {
        &self.name
    }
    /// Returns the age of the [`jr_struct`].
    fn get_age(&self) -> u8 {
        self.age
    }
}

pub(crate) fn generate_jr_struct() {
    println!();
    // input name and age
    let mut name = String::new();
    let mut age = String::new();
    println!("Please input your name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Please input your age:");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    let name: String = name.trim().to_string(); // must be done, or the name will contain '\n'
    let age: u8 = age.trim().parse().expect("Please type a number");
    // create a new jr_struct
    let jr_struct = JrStruct::new(name, age); /* JrStruct::new() => static method */
    // print the jr_struct
    println!();
    println!(
        "{} is {} years old.",
        jr_struct.get_name(),
        jr_struct.get_age()
    );
    println!();
    // make a copy of the struct
    let jr_struct_copy = jr_struct; // moved! now you could not use jr_struct anymore
    println!("struct has been copied, now to show the information of the copied one ... ");
    println!(
        "{} is {} years old.",
        jr_struct_copy.get_name(),
        jr_struct_copy.get_age()
    );
    println!();
    // make a copy with change of the original struct
    let jr_struct_copy_t = JrStruct {
        age: jr_struct_copy.get_age() + 1,
        ..jr_struct_copy // ..jr_struct_copy must be placed in the end
                         // but, it's unnecessary to value variables in the original order
                         // origianl order => order occured during declaration
    };
    println!("struct has been copied again, now to show the information of the copied one ... ");
    println!(
        "{} is {} years old.",
        jr_struct_copy_t.get_name(),
        jr_struct_copy_t.get_age()
    );
    println!();
}
