use std::io;

pub(crate) fn senior_struct() {
    println!();
    println!("1 => tuple_struct");
    println!("2 => struct_without_var");
    println!("3 => debug_print_struct");
    println!("4 => pretty_debug_print_struct");
    println!("5 => methods");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number");
    match choice {
        1 => tuple_struct(),
        2 => struct_without_var(),
        3 => debug_print_struct(),
        4 => pretty_debug_print_struct(),
        5 => methods(),
        _ => {
            println!("Invalid choice")
        }
    }
}

fn tuple_struct() {
    println!();
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    impl Color {
        fn new(r: i32, g: i32, b: i32) -> Color {
            Color(r, g, b)
        }
        fn show(&self) {
            println!("R:{}, G:{}, B:{}", self.0, self.1, self.2);
        }
    }
    impl Point {
        fn new(x: i32, y: i32, z: i32) -> Point {
            Point(x, y, z)
        }
        fn show(&self) {
            println!("X:{}, Y:{}, Z:{}", self.0, self.1, self.2);
        }
    }
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let _black = Color::new(0, 0, 0);
    let _origin = Point::new(0, 0, 0);
    // show _black and _origin
    _black.show();
    _origin.show();
    // end
    println!();
}

fn struct_without_var() {
    println!();
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
    println!("input >> ");
    println!("\t let _subject = AlwaysEqual;");
}

fn debug_print_struct() {
    println!();
    #[derive(Debug)]
    struct Foo {
        _var1: i32,
        _var2: i32,
    }
    let foo = Foo {
        _var1: 10,
        _var2: 20,
    };
    println!("Constructed => {:?}", foo);
}

fn pretty_debug_print_struct() {
    println!();
    #[derive(Debug)]
    struct Foo {
        _var1: i32,
        _var2: i32,
    }
    let foo = Foo {
        _var1: 10,
        _var2: 20,
    };
    println!("Constructed => {:#?}", foo);
}

fn methods() {
    println!();
    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }
    impl Rectangle {
        // new() => this is not the constructor!
        // constructor could be only named as Class_name(), and don't need to return anything
        // actually, it's something forces you to build a struct in `factory method` liked pattern
        fn new(width: f64, height: f64) -> Self {
            Self { width, height }
            // param_name = var_name, then could write in this way
            // actually, it should be mentioned, rust allow Struct-Bundle-Pattern
            //      Self { width, height }
            //      ^^^^----------------------> Struct
            //=> that's why we call it as Struct-Bundle-Pattern
            // Struct-Bundle-Pattern <=> Struct-Bundle-Initialization
        }
        fn area(&self) -> f64 {
            self.width * self.height
        }
        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        // &self <=> self: &Self
        // &Self => contained in impl => equal to `*this` in Cpp
    }
    let _rect1 = Rectangle::new(3.0, 4.0);
    let _rect2 = Rectangle::new(5.0, 6.0);
    println!("_rect1 => {:?}", _rect1);
    println!("_rect2 => {:?}", _rect2);
    println!("Area of _rect1 => {:?}", _rect1.area());
    println!("Perimeter of _rect1 => {:?}", _rect1.perimeter());
    println!("Can _rect1 hold _rect2 => {:?}", _rect1.can_hold(&_rect2));
    println!("Can _rect2 hold _rect1 => {:?}", _rect2.can_hold(&_rect1));
    println!();
}
