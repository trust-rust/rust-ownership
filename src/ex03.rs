crate fn ex03_move_fn() {
    let x = String::from("  trust-rust  "); // x establishes ownership of the string here
    let y = modify_string(x); // x is moved into the function 'modify_string'

    // Uncomment line 23 to see the following error
    // error[E0382]: borrow of moved value: `x`
    //   --> src/ex03.rs:23:27
    //    |
    // 2  |     let x = String::from("  trust-rust  "); // x establishes ownership of the string here
    //    |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
    // 3  |     let y = modify_string(x); // x is moved into the function 'modify_string'
    //    |                           - value moved here
    // ...
    // 23 |     println!("x is '{}'", x);
    //    |                           ^ value borrowed here after move

    // error: aborting due to previous error

    // For more information about this error, try `rustc --explain E0382`.
    // error: could not compile `tr`

    // To learn more, run the command again with --verbose.
    // println!("x is '{}'", x);
    println!("y is '{}'", y);

    let mine_now = give_ownership(); // The return value of give_ownership is moved into mine_now
    let given_back = take_and_give_back(mine_now); // mine_now is moved into take_and_give_back
                                                   // the return value of take_and_give_back is moved into given_back
                                                   // println!("mine_now is '{}'", mine_now);
    println!("given_back is '{}'", given_back);
}

fn modify_string(x: String) -> String {
    // x comes into scope
    println!("Doing something with x in modify_string: '{}'", x);
    x.trim().to_string() // Using x while in scope
} // Here x goes out of scope and is dropped.

fn give_ownership() -> String {
    let string_to_give = String::from("move everywhere"); // string_to_give takes ownership of the String
    string_to_give // The value is retuned here, giving ownership to the caller
}

fn take_and_give_back(taken: String) -> String {
    // taken comes into scope
    taken // The value is retuned here, giving ownership to the caller
}
