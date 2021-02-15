# 1. Introduction

Rust is my favorite language, hands down.  This wasn't always the case, even after I had started using the language. But now it is.  This is the first post in a series of 3 that goes over some of the features of Rust that have landed it squarely atop the language pile for me.

First however, I'd like to take a bit of time explaning the history of my journey to Rust.

I was really introduced to programming in college.  C was the language and it was painful for me.  Next was assembly, which somehow was worse.   Finally, my senior year I took a class on C++.  However, there was this new language on the block called Java and they had decided to teach us both at the same time.   Java was easy for me to understand and use.   When I graduated and got my first job, it was Java.   I loved Java for a long time.  As I gained more experience, I began to see some of the warts.  They weren't terrible, but they were certainly there (NPEs anyone?).  About 13 years into my career using Java, a co-worker introduced me to Clojure.   My programming world changed again.  What were these wonderous concepts in functional programming?  I started learning as much as I could about the concepts.   This included attendeing a local functional programming group.  I still attend this group and still learn about new tools to use in my programming career.  It was also the place that I was introduced to Rust.   After hearing about Rust at this meetup I decided to give it a try.  If I recall correctly, the Rust team had just released version 0.6.   I was hooked almost immediately.

The journey to my favorite language was filled with potholes.   Rust can be a bear to learn.  If you do any digging you'll eventually come across stories about people fighting the borrow checker and losing.  My only advice is this:  Give the language a try.  If you get stuck, set it down for a few weeks.  Come back fresh and give the language another go.  I had to re-frame how I thought about problems when programming with Rust, but in the end I think this makes me a better programmer and I now rank Rust as my favorite.

So, with that behind us, what is it about Rust that I love.

* The memory safety
* The documentation
* The dev tools
* Async/Await

In this post, I will be going over the first bullet.   The others I will cover in the follow up posts.
# 2. Installation

As this is the first post in a series I will briefly go over getting Rust installed.

1. Use your browser to navigate to [rustup.rs](https://rustup.rs/)
2. Copy the curl script presented to you and run it at the command line.
3. You will be prompted to select some options.  For our purposes, you can use the default.
4. Make note of the final bit of output.  You should ensure your path is set correctly per the instructions.
5. Run `rustc -V` and `cargo -V` from the command line.  If everything has been setup correctly you will see output like below (the versions may be different):
````
    ab@domino ~ λ rustc -V
    rustc 1.49.0 (e1884a8e3 2020-12-29)
    ab@domino ~ λ cargo -V
    cargo 1.49.0 (d00d64df9 2020-12-05)
````

If you prefer not to run a script from the web, there are other installation options [here](https://rust-lang.github.io/rustup/installation/other.html)

As for IDEs, I prefer to use Visual Studio Code.  For a better experience I suggest installing the `rust-analyzer` extension.  This extension enables fun IDE features like error highlighting, auto-complete, etc.
# 3. Memory Safety and Ownership Defined

One of the distinguishing features of Rust is memory safety at performance levels near the C language.
## A. Memory Safety
For the uninitiated, memory safety refers to a class of bugs that your program is protected from at runtime.  These bugs include some well known issues such as buffer overflows, use after free, and dangling pointers.  Normally, memory safety is achieved by means of garbage collection or GC.  A full discussion about GC is out of scope here, but GC can cause other issues (raise your hand if you ever hooked up to a memory starved JVM to watch the GC thrash).  Rust on the other hand achieves memory safety through the concept of ownership where memory safety checks happen at compile time.  Completing the checks at compile time means there is no runtime impact on performance.

## B. Ownership
So what is ownership?  First the ownership rules:

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

That's it.  Now, we will grab some code and dive into some examples to illustrate in a more concerete manner what ownership is.

# 4. The Code
These examples loosely follow the examples documented in the [Rust book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).  I hightly recommend taking some time to peruse the entire book, it's very good.  I've added some additional examples and context where I've run into ownership conundrums in the past.

The code for the examples in this post is hosted at https://github.com/trust-rust/rust-ownership.git.

# 5. Ownership By Example
## A. Variable Scope
### Variable Scope Example
As in many other languages, scope is the range within which a variable is valid.  Take a minute to study the scope example below, in particular the comments and the scope error:

```rust
02:     {
03:         // 'a_str' is not valid here, it’s not yet declared
04:         //
05:         // Uncomment line 18 to see the following error:
06:         //
07:         // error[E0425]: cannot find value `a_str` in this scope
08:         //   --> src/ex01.rs:18:33
09:         //    |
10:         // 17 |         println!("a_str is {}", a_str);
11:         //    |                                 ^^^^^ not found in this scope
12:         //
13:         // error: aborting due to previous error
14:         //
15:         // For more information about this error, try `rustc --explain E0425`.
16:         // error: could not compile `rust_own`
17:         //
18:         // println!("a_str is {}", a_str);
19:
20:         let a_str = "trust-rust"; // 'a_str' is valid from this point forward
21:         println!("a_str is {}", a_str); // You can use 'a_str' while it is in scope
22:     } // This curly brace indicates that our scope is now over, and 'a_str' is no longer valid
23:
24:     // Uncomment line 25 to see a similar scope error to the above
25:     // println!("a_str is {}", a_str);
```

As you can see, variable scope is similar in nature to many other languages.

### Run the Code
We will be using `cargo` to interact with most of the examples.  `cargo` is the Rust package manager and build tool.

The full code for this example is in the `ex01.rs` file in the `src/` directory.

1. Execute `cargo run ex01` at the command line in the base directory of the project.  What did you expect to see?
1. Uncomment line 20 and re-run `cargo run ex01`.  What do you see this time?
1. Comment line 20, uncomment line 27, and re-run `cargo run ex01`.  What changed from the previous results?

## B. Move
### First a question...
The previous example is pretty common amongst programming languages.  The following is an example that is more unique to Rust.

Before we dive into move, take a look at the code below.  What would you expect to happen if you ran this?
````rust
{
    let x = String::from("trust-rust");
    let y = x;
    println!("x is {}", x);
    println!("y is {}", y);
}
````

Most people from a Java background like myself would probably answer **"It prints 'trust-rust' twice"**, however, this is incorrect.  If you answered, **"A compilation failure"**, congratulations!  A compiler error similar to below would be generated.

````rust
01: error[E0382]: borrow of moved value: `x`
02:   --> src/ex02.rs:23:30
03:    |
04: 4  |     let x = String::from("trust-rust");
05:    |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
06: 5  |     let y = x;
07:    |             - value moved here
08: ...
09: 23 |     info!(stdout, "x is {}", x);
10:    |                              ^ value borrowed here after move
11:
12: error: aborting due to previous error
13:
14: For more information about this error, try `rustc --explain E0382`.
15: error: could not compile `tr`
````

That error may seem dense with unfamiliar terms, so lets look at the relevant output line by line.

#### **Line 1**
````rust
01: error[E0382]: borrow of moved value: `x`
````

Here we can see the base error.  We are talking about move and we can see we have generated a move error.  But what is borrow?  We will get to that.

#### **Line 2**
````rust
02:   --> src/ex02.rs:23:30
````
Here we get the line and column of where the error was generated.  If you take a moment and look ahead to line 9, you will see the actual line from the code in the output with some error information on line 10.

#### **Lines 3 - 7**
````rust
03:    |
04: 4  |     let x = String::from("trust-rust");
05:    |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
06: 5  |     let y = x;
07:    |             - value moved here
````

Lines 3-7 are the "more information" block of the error.  This block is intended to help guide you to the solution.

On line 5, we can see a message regarding line 4.  Again, a move is referenced in regards to the type 'String'.  But what is the 'Copy' trait?  We will get to that after we discuss borrow.

On line 7, we can see that x was moved via the code on line 6.

#### **Lines 9 - 10**
````rust
09: 23 |     info!(stdout, "x is {}", x);
10:    |                              ^ value borrowed here after move
````
This is where the error was generated.  The helper text on line 10 is pointing to the location in the code where the error occurred.

Phew!  You made it through.   So what are all of these new terms, move, borrow, copy?

### Now back to move...
For me it helps if you think of the equals (`=`) operator as taking ownership rather than assignment.  When a variable takes ownership of something, we say that something has "moved" into that variable.

In my head I "read" the code
````rust
let x = String::from("trust-rust");
````
as "The string "trust-rust" has moved into x".  You can also think of the interaction as the inverse.  `x` has taken ownership of the string "trust-rust".  This is a slight over-simplification but it works for now.

Now we can see that the following is transferring ownership from `x` to `y`.  What `x` once owned (the string), now belongs to `y`.
```rust
let y = x;
```
At this point `x` no longer owns anything, so is invalid.  Therefore, we can no longer refer to `x` after this point in the code.

### Run the Code
The full code for this example is in the `ex02.rs` file in the `src/` directory.

1. Execute `cargo run ex02` at the command line in the base directory of the project.  What did you expect to see?
1. Uncomment line 22 and re-run `cargo run ex02`.  What do you see this time?

## C. Functions and Move
You will also experience move when working with functions.

### Moving values into functions
Again we will start with a question.  What would you expect to happen if you ran the following code?

```rust
    let x = String::from("  trust-rust  ");
    let y = modify_string(x);
    println!("x is '{}'", x);
    println!("y is '{}'", y);
```

With the knowledge of the previous section at your disposal, hopefully you answered **"A compilation failure"**.  If not, don't worry.  Move semantics can take some getting used to.

Lets take a look at the fully annotated code:

```rust
02:     let x = String::from("  trust-rust  "); // x establishes ownership of the string here
03:     let y = modify_string(x); // x is moved into the function 'modify_string'
04:
05:     // Uncomment line 23 to see the following error
06:     // error[E0382]: borrow of moved value: `x`
07:     //   --> src/ex03.rs:23:27
08:     //    |
09:     // 2  |     let x = String::from("  trust-rust  "); // x establishes ownership of the string here
10:     //    |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
11:     // 3  |     let y = modify_string(x); // x is moved into the function 'modify_string'
12:     //    |                           - value moved here
13:     // ...
14:     // 23 |     println!("x is '{}'", x);
15:     //    |                           ^ value borrowed here after move
16:
17:     // error: aborting due to previous error
18:
19:     // For more information about this error, try `rustc --explain E0382`.
20:     // error: could not compile `tr`
21:
22:     // To learn more, run the command again with --verbose.
23:     // println!("x is '{}'", x);
24:     println!("y is '{}'", y);
```

You can see above that when you call `modify_string(x)` the value `x` is moved into the function.  You can also think of it as the `modify_string` function taking ownership of the value `x`.  You may also occasionally see that the value `x` is consumed by the function `modify_string` in the wild.  No matter which way you view it, when you call a function as above, the value is moved into the function and unusable in the original scope.

### Run the Code
The full code for this example is in the `ex03.rs` file in the `src/` directory.

1. Take a minute to review the code in `ex03.rs`.
1. Execute `cargo run ex03` at the command line in the base directory of the project.  What did you expect to see?
1. Uncomment line 23 and re-run `cargo run ex03`.  What do you see this time?

### Moving values out of functions
Returning values from functions also transfers ownership.  Take a look at the following annotated example.

```rust
02:     let mine_now = give_ownership(); // The return value of give_ownership is moved into mine_now
03:     let given_back = take_and_give_back(mine_now); // mine_now is moved into take_and_give_back
04:                                                    // the return value of take_and_give_back is moved into given_back
05:
06:     // Uncomment line 24 to see the following error
07:     //     error[E0382]: borrow of moved value: `mine_now`
08:     //   --> src/ex04.rs:24:34
09:     //    |
10:     // 2  |     let mine_now = give_ownership(); // The return value of give_ownership is moved into mine_now
11:     //    |         -------- move occurs because `mine_now` has type `String`, which does not implement the `Copy` trait
12:     // 3  |     let given_back = take_and_give_back(mine_now); // mine_now is moved into take_and_give_back
13:     //    |                                         -------- value moved here
14:     // ...
15:     // 24 |     println!("mine_now is '{}'", mine_now);
16:     //    |                                  ^^^^^^^^ value borrowed here after move
17:
18:     // error: aborting due to previous error
19:
20:     // For more information about this error, try `rustc --explain E0382`.
21:     // error: could not compile `tr`
22:
23:     // To learn more, run the command again with --verbose.
24:     // println!("mine_now is '{}'", mine_now);
25:     println!("given_back is '{}'", given_back);

28: fn give_ownership() -> String {
29:     let string_to_give = String::from("move everywhere"); // string_to_give takes ownership of the String
30:     string_to_give // The value is retuned here, giving ownership to the caller
31: }
32:
33: fn take_and_give_back(taken: String) -> String {
34:     // taken comes into scope
35:     taken // The value is retuned here, giving ownership to the caller
36: }
```

Here you can see that a value generated within a function can be moved back to the caller as with `give_ownership`.  You can also see that a function can take ownership of a value and give ownership of that value back to the caller as in `take_and_give_back`.   One important note however, is that the value given back by `take_and_give_back` is assigned to a new variable.  The variable `mine_now` cannot be used again as seen if you uncomment line 24.

Taking ownership in a function and then returning it back can be useful.  I see this frequently with the Builder pattern in Rust, where you build up or configure something, and then at the end call `something.build()` where everything is consumed and you are given back ownership of the item you were building. That may be a topic in a followup post.  More often you will want to use the value within your function but not take ownership of said value.  In Rust, this is accomplished by 'borrowing' the value.  I told you we would get to it eventually....

### Run the Code
The full code for this example is in the `ex04.rs` file in the `src/` directory.

1. Take a minute to review the code in `ex04.rs`.
1. Execute `cargo run ex04` at the command line in the base directory of the project.  What did you expect to see?
1. Uncomment line 24 and re-run `cargo run ex04`.  What do you see this time?

## D. Borrow
### References
Armed with your move knowledge so far, how would you create a function to check if the given string started with 'Rust'?  Below is an example using the move semantics without borrow.

```rust
02:     let maybe_rusty = String::from("Rust is Good");                  // maybe_rusty takes ownership of the String
03:     let (maybe_rusty_now, is_rusty) = starts_with_rust(maybe_rusty); // ownership of maybe_rusty is transferred to starts_with_rust
04:                                                                      // The return value (a tuple) is moved into maybe_rusty_now and is_rusty here
05:     if is_rusty {                                                    // We can use is_rusty because we own it in this scope
06:         println!("Your string '{}' is rusty", maybe_rusty);          // We can use maybe_rusty_now because we own it in this scope
07:     }
09:
10: fn starts_with_rust(a_string: String) -> (String, bool) {  // a_string comes into scope
11:     let is_rusty = a_string.starts_with("Rust");           // Figure out if the string starts with "Rust" and give ownership of the result to is_rusty
12:     (a_string, is_rusty)                                   // Transfer ownership of the values we care about back to the caller
13: }
```

I don't know how you feel about that, but that seems a bit kludgy.  Also, that pattern wouldn't scale well the more information you wanted to extract from that string.  What if you wanted the length as well?  Luckily, the designers of Rust took this into consideration and we have the concept of references and borrowing.  Take a look at another example that does the same as above, but with borrow.

```rust
02:     let maybe_rusty = String::from("Rust is Good");          // maybe_rusty takes ownership of the String
03:
04:     if starts_with_rust(&maybe_rusty) {                      // starts_with rust borrows maybe_rusty to use.
                                                                 // when starts_with rust returns the bool value is moved into this scope (and used by if)
                                                                 // the borrow of maybe_rusty also ends, so it can be used again in this scope
05:         println!("Your string '{}' is rusty", maybe_rusty);  // We use maybe_rusty again as we have ownership
06:     }
08:
09: fn starts_with_rust(a_string: &String) -> bool {  // a_string comes into scope.  This is a reference to the original.
10:     a_string.starts_with("Rust")                  // Figure out if the string starts with "Rust" and give ownership of the resulting bool back to the caller
11: }
```

Did you catch the borrow?  The `&` symbol before the `maybe_rusty` argument informs the compiler that you are giving a reference of that value to the `starts_with_rust` function.  Another way to look at this is that `starts_with_rust` is borrowing a string to check if the string starts with "Rust".

In my opinion, the code above is more concise and clear.   At this point, you should be able to see that the term borrow makes sense here.   When you've given a function a reference to something you own in your scope, they are borrowing the value to look at for a moment, and giving it back when they are done.   Just like when your neighbor borrows your hammer to work on his deck.  He takes it for a bit, does some work with it, then gives it back (hopefully).

For a bit of a challenge here, take a moment and think about how you would add the calculation of the string length into the following code.  I've left some commented code in the examples if you would like to see how I would do it.

### Run the Code
The full code for the previous examples are in the `ex05.rs` and `ex06.rs` files in the `src/` directory.

1. Take a minute to review the code in `ex05.rs` and `ex06.rs`
1. Execute `cargo run ex05` at the command line in the base directory of the project.  What did you expect to see?
1. Execute `cargo run ex06` at the command line in the base directory of the project.  What did you expect to see?
1. Uncomment the commented out code in the examples.  Is this how you would've approached the line length?

### Let's Change Things - Mutable References
What if you want to modify a value?  Or have your function modify a value given to it? Take a look at the following.

```rust
let x = String::from("try to change me");
x.push_str(", fine I will");
println!("x is '{}'", x);
```

Again, if you come from a Java background like myself, you'd expect this to print out `try to change me, fine I will` when you ran this.  However, if you've come this far, you know that this generates a compiler error.

```rust
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/ex07.rs:3:5
  |
2 |     let x = String::from("try to change me");
  |         - help: consider changing this to be mutable: `mut x`
3 |     x.push_str(", fine I will");
  |     ^ cannot borrow as mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
error: could not compile `tr`

To learn more, run the command again with --verbose.
```

Variables in Rust are immutable by default.  As you can see from the error above, you are not allowed to mutate immutable variables.  If you add `mut` in front of the `x` as the message suggests you should, you will fix this particular error.

What if you want to change a value within a function?  If you try it with a reference

```rust
let x = String::from("try to change me");
change_it(&x);
println!("x is '{}'", x);

fn change_it(x: &String) {
    x.push_str(", fine I will");
}
```

you will get the following compiler error

```rust
error[E0596]: cannot borrow `*x` as mutable, as it is behind a `&` reference
  --> src/ex08.rs:12:5
   |
11 | fn change_it(x: &String) {
   |                 ------- help: consider changing this to be a mutable reference: `&mut String`
12 |     x.push_str(", fine I will");
   |     ^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
error: could not compile `tr`

To learn more, run the command again with --verbose.
```

The error message is guiding us to the solution.  You can use a mutable reference to tell the compiler that you would like to be able to mutate the value within your function.  The following example demonstrates using mutable references.

```rust
let mut x = String::from("try to change me");  // x takes ownership of the string, and also tells the compiler we will be changing it.
change_it(&mut x);                             // Give a mutable reference to the change_it function to allow it to modify our variable.
println!("x is '{}'", x);                      // We retain ownership, but at this point the change_it function has modified our string.

fn change_it(x: &mut String) {   // Tell the Rust compiler we plan to mutate this argument by declaring it a mutable reference
    x.push_str(", fine I will");  // Do the mutating
}
```

### Reference Rules
There are a couple of important rules around references and mutable references.

1. You may have many immutable references at one time to a variable in a given scope.
1. You may only have one mutable reference to a variable at a time in a given scope.
1. You may not have a mutable referece to a variable that also has immutable references in a given scope.

I think of this in terms of read/write.  You may have multiple readable reference on the same variable, but you may only have one writeable reference within a given scope.  Rust enforces this rule to prevent data races.  This basically reduces all mutations to atomic operations.  Some examples should help clarify these rules.

#### Multiple Immutable References
```rust
let mut s = String::from("many immutable references");

let r1 = &s;
let r2 = &s;

println!("{}, {}", r1, r2);
```

The previous code illustrate that you can have multiple immutable references to the same variable in the same scope.

#### Multiple Mutable References
```rust
let mut s = String::from("");
let a = &mut s;
let b = &mut s;

println!("a is {}, b is {}", a, b);
```

The code above will generate the following compiler error

```rust
error[E0499]: cannot borrow `s` as mutable more than once at a time
  --> src/ex08.rs:14:13
   |
13 |     let a = &mut s;
   |             ------ first mutable borrow occurs here
14 |     let b = &mut s;
   |             ^^^^^^ second mutable borrow occurs here
15 |
16 |     println!("a is {}, b is {}", a, b);
   |                                  - first borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
error: could not compile `tr`

To learn more, run the command again with --verbose.
```

#### Multiple Mixed References
```rust
let mut s = String::from("");
let a = &s;
let b = &s;
let c = &mut s;

println!("a is {}, b is {}, c is {}", a, b, c);
```

The code above will generate the following compiler error

```rust
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/ex08.rs:15:13
   |
13 |     let a = &s;
   |             -- immutable borrow occurs here
14 |     let b = &s;
15 |     let c = &mut s;
   |             ^^^^^^ mutable borrow occurs here
16 |
17 |     println!("a is {}, b is {}, c is {}", a, b, c);
   |                                           - immutable borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
error: could not compile `tr`

To learn more, run the command again with --verbose.
```

# F. The End, or hopefully The Beginning
This marks the end of our exploration of the Rust concepts of move, borrow, and ownership.  Hopefully you have a better understanding of one of the features that helps Rust stand out in the field of languages.  As I mentioned before, you should take some time to explore the Rust book.   This post draws from the fourth chapter in the book 'What is Ownership?'.  In that chapter they take a deeper dive into these concepts.  And while this marks the end of this post, I'd like to think that for some of you, this marks the beginning of your jouney into a language that has become my favorite.
