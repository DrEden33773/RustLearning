#[allow(non_snake_case)]
pub(crate) fn _else_interface() {}

#[allow(non_snake_case)]
fn _Vec_and_Built_in_Array_Type() {
    fn largest<T>(list: &[T]) -> T
    where
        T: std::cmp::PartialOrd,
        T: std::fmt::Debug,
        T: Copy + Clone,
    {
        let mut largest = list[0].clone();

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn interface() {
        let number_list = vec![34, 50, 25, 100, 65];

        // show the actual memsize of any var => std::mem::size_of_val(&input_var)

        println!(
            "number_list memsize: {}",
            std::mem::size_of_val(&number_list)
        );

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
}
