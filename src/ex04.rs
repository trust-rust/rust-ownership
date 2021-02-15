crate fn ex04_fn_return() {
    let mine_now = give_ownership(); // The return value of give_ownership is moved into mine_now
    let given_back = take_and_give_back(mine_now); // mine_now is moved into take_and_give_back
                                                   // the return value of take_and_give_back is moved into given_back

    // Uncomment line 24 to see the following error
    //     error[E0382]: borrow of moved value: `mine_now`
    //   --> src/ex04.rs:24:34
    //    |
    // 2  |     let mine_now = give_ownership(); // The return value of give_ownership is moved into mine_now
    //    |         -------- move occurs because `mine_now` has type `String`, which does not implement the `Copy` trait
    // 3  |     let given_back = take_and_give_back(mine_now); // mine_now is moved into take_and_give_back
    //    |                                         -------- value moved here
    // ...
    // 24 |     println!("mine_now is '{}'", mine_now);
    //    |                                  ^^^^^^^^ value borrowed here after move

    // error: aborting due to previous error

    // For more information about this error, try `rustc --explain E0382`.
    // error: could not compile `tr`

    // To learn more, run the command again with --verbose.
    // println!("mine_now is '{}'", mine_now);
    println!("given_back is '{}'", given_back);
}

fn give_ownership() -> String {
    let string_to_give = String::from("move everywhere"); // string_to_give takes ownership of the String
    string_to_give // The value is retuned here, giving ownership to the caller
}

fn take_and_give_back(taken: String) -> String {
    // taken comes into scope
    taken // The value is retuned here, giving ownership to the caller
}
