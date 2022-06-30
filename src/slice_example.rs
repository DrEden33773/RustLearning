pub(crate) fn slice_example() {
    println!("");
    println!("This is an example of SliceReference ... ");
    println!("");

    let my_string = String::from("Hello world"); // => 字符串

    // `first_word` 接受 `String` 的切片，无论是部分还是全部
    let _word = return_slice(&my_string[0..6]);
    let _word = return_slice(&my_string[..]);
    // `first_word` 也接受 `String` 的引用，
    // 这等同于 `String` 的全部切片
    let word = return_slice(&my_string);

    println!("<{}> is the first word of [<| String |>]", word);

    let my_string_literal = "hello world"; // => 字符串字面值(literal)

    // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    let _word = return_slice(&my_string_literal[0..6]);
    let _word = return_slice(&my_string_literal[..]);
    // 因为字符串字面值**就是**字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = return_slice(my_string_literal);

    println!("<{}> is the first word of [<| String_literal |>]", word);

    // end
    println!();
}

fn return_slice(input: &str) -> &str {
    // &str <=> Slice_type
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if !((item >= b'a' && item <= b'z') || (item >= b'A' && item <= b'Z')) {
            return &input[0..i];
        }
    }
    return &input[..];
}

// let my_str = String::from("a");
//                          ^^^^^---------> type: String
// let my_str_literal = "a";
//                      ^^^---------> type: &'static str (literal)
// literal <=> a slice of the whole string(var)
