crate fn ex05_move_only() {
    let maybe_rusty = String::from("Rust is Good");
    let (maybe_rusty_now, is_rusty) = starts_with_rust(maybe_rusty);
    // let (maybe_rusty_now, len) = how_long(maybe_rusty_now);

    if is_rusty {
        println!("Your string '{}' is rusty", maybe_rusty_now);
        // println!("Your string is '{}' characters long", len);
    }
}

fn starts_with_rust(a_string: String) -> (String, bool) {
    let is_rusty = a_string.starts_with("Rust");
    (a_string, is_rusty)
}

// fn how_long(a_string: String) -> (String, usize) {
//     let len = a_string.len();
//     (a_string, len)
// }
