pub(crate) fn enum_example() {
    println!();
    enum_bounded_with_struct();
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
}
