crate fn ex02_move() {
    let x = String::from("trust-rust"); // x establishes ownership of the string here
    let y = x; // ownership of the string is moved to y.  x is now invalid and out of scope.

    // Uncomment line 21 to see the following error
    //
    // error[E0382]: borrow of moved value: `x`
    //   --> src/ex02.rs:21:25
    //    |
    // 2  |     let x = String::from("trust-rust");  // x establishes ownership of the string here
    //    |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
    // 3  |     let y = x;                           // ownership of the string is moved to y.  x is now invalid and out of scope.
    //    |             - value moved here
    // ...
    // 21 |     println!("x is {}", x);              // x is invalid and cannot be used.
    //    |                         ^ value borrowed here after move
    //
    // error: aborting due to previous error
    //
    // For more information about this error, try `rustc --explain E0382`.
    // error: could not compile `tr`
    // println!("x is {}", x); // x is invalid and cannot be used.

    println!("y is {}", y); // y owns the string, therefore it can be used.
}
