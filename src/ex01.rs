use slog::{info, Logger};

crate fn variable_scope(stdout: &Logger) {
    {
        // 'a_str' is not valid here, it’s not yet declared
        //
        // Uncomment line 20 to see the following error:
        //
        // error[E0425]: cannot find value `a_str` in this scope
        //   --> src/ex01.rs:20:38
        //    |
        // 20 |         info!(stdout, "a_str is {}", a_str);
        //    |                                      ^^^^^ not found in this scope
        //
        // error: aborting due to previous error
        //
        // For more information about this error, try `rustc --explain E0425`.
        // error: could not compile `rust_own`
        //
        // info!(stdout, "a_str is {}", a_str);

        let a_str = "trust-rust";               // 'a_str' is valid from this point forward
        info!(stdout, "a_str is {}", a_str);         // You can use 'a_str' while it is in scope
    }                                                // This curly brace indicates that our scope is now over, and 'a_str' is no longer valid

    // Uncomment line 27 to see a similar scope error to the above
    // info!(stdout, "a_str is {}", a_str);
}
