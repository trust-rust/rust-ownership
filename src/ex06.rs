crate fn ex06_borrow() {
    let maybe_rusty = String::from("Rust is Good");
    // let len = how_long(&maybe_rusty);

    if starts_with_rust(&maybe_rusty) {
        println!("Your string '{}' is rusty", maybe_rusty);
        // println!("Your string is '{}' characters long", len);
    }
}

fn starts_with_rust(a_string: &String) -> bool {
    a_string.starts_with("Rust")
}

// fn how_long(a_string: &String) -> usize {
//     a_string.len()
// }
