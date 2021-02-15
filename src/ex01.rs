crate fn ex01_scope() {
    {
        // 'a_str' is not valid here, it’s not yet declared
        //
        // Uncomment line 18 to see the following error:
        //
        // error[E0425]: cannot find value `a_str` in this scope
        //   --> src/ex01.rs:18:33
        //    |
        // 17 |         println!("a_str is {}", a_str);
        //    |                                 ^^^^^ not found in this scope
        //
        // error: aborting due to previous error
        //
        // For more information about this error, try `rustc --explain E0425`.
        // error: could not compile `rust_own`
        //
        // println!("a_str is {}", a_str);

        let a_str = "trust-rust"; // 'a_str' is valid from this point forward
        info!(stdout, "a_str is {}", a_str); // You can use 'a_str' while it is in scope
    } // This curly brace indicates that our scope is now over, and 'a_str' is no longer valid

    // Uncomment line 25 to see a similar scope error to the above
    // println!("a_str is {}", a_str);
}
