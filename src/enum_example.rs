use std::io;

pub(crate) fn enum_example() {
    println!();
    println!("1 => enum_bounded_with_struct");
    println!("2 => evolution_null");
    // receive input from user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    match input {
        1 => enum_bounded_with_struct(),
        2 => evolution_null(),
        _ => println!("Invalid input"),
    }
}

fn enum_bounded_with_struct() {
    // 定义 直角坐标系 和 极坐标系
    #[derive(Debug)]
    struct CartesianStruct {
        x: f64,
        y: f64,
    }
    #[derive(Debug)]
    struct PolarStruct {
        radius: f64,
        theta: f64,
    }
    #[derive(Debug)]
    enum Coordinate {
        Cartesian(CartesianStruct),
        Polar(PolarStruct),
    }
    impl CartesianStruct {
        fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
        fn show(&self) {
            println!("X:{}, Y:{}", self.x, self.y);
        }
    }
    impl PolarStruct {
        fn new(radius: f64, theta: f64) -> Self {
            Self { radius, theta }
        }
        fn show(&self) {
            println!("R:{}, θ:{}", self.radius, self.theta);
        }
    }
    impl Coordinate {
        fn transform(&self) -> Self {
            // match is strong enough to get the type of self
            match self {
                Coordinate::Cartesian(cartesian) => Coordinate::Polar(PolarStruct::new(
                    cartesian.x.hypot(cartesian.y),
                    cartesian.x.atan2(cartesian.y),
                )),
                Coordinate::Polar(polar) => Coordinate::Cartesian(CartesianStruct::new(
                    polar.radius * polar.theta.sin(),
                    polar.radius * polar.theta.cos(),
                )),
            }
        }
        fn show(&self) {
            match self {
                Coordinate::Cartesian(cartesian) => cartesian.show(),
                Coordinate::Polar(polar) => polar.show(),
            }
        }
    }
    // examples
    let cartesian = Coordinate::Cartesian(CartesianStruct::new(3.0, 4.0));
    let polar = Coordinate::Polar(PolarStruct::new(3.0, 0.0));
    cartesian.show();
    polar.show();
    let cartesian_to_polar = cartesian.transform();
    cartesian_to_polar.show();
    let polar_to_cartesian = polar.transform();
    polar_to_cartesian.show();
    // end
    println!();
}

fn evolution_null() {
    println!();
    println!("Here is an example of `Option<T>`, this is a nullable type");
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;
    println!("Called => let x: Option<i32> = Some(5);");
    println!("Called => let y: Option<i32> = None;");
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("x.unwrap() is {}", x.unwrap());
    // println!("y.unwrap() is {}", y.unwrap()); //=> panic
    println!("y is None, so it could not be unwraped ... ");
    // end
    println!();
}
