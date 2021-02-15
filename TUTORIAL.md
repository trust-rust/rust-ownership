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

For each example, there will be a corresponding branch, e.g. Example 1 will have code in the `ex01` branch.  I will call out when you should grab a new branch.

I suggest switching to the appropriate branch for each example so you can tinker with actual code, i.e. `git checkout -t origin/ex01`

# 5. Ownership By Example
## A. Variable Scope
### Variable Scope Example
As in many other languages, scope is the range within which a variable is valid.  Take a minute to study the scope example below, in particular the comments and the scoope error:

```rust
File: ex01.rs
04:     {   // This curly brace indicates a scope has been started
05:         // 'a_str' is not valid here, it’s not yet declared
06:         //
07:         // Uncomment line 20 to see the following error:
08:         //
09:         // error[E0425]: cannot find value `a_str` in this scope
10:         //   --> src/ex01.rs:20:38
11:         //    |
12:         // 20 |         info!(stdout, "a_str is {}", a_str);
13:         //    |                                      ^^^^^ not found in this scope
14:         //
15:         // error: aborting due to previous error
16:         //
17:         // For more information about this error, try `rustc --explain E0425`.
18:         // error: could not compile `rust_own`
19:         //
20:         // info!(stdout, "a_str is {}", a_str);
21:
22:         let a_str = "trust-rust";                    // 'a_str' is valid from this point forward
23:         info!(stdout, "a_str is {}", a_str);         // You can use 'a_str' while it is in scope
24:     }   // This curly brace indicates that our scope is now over, and 'a_str' is no longer valid
25:
26:     // Uncomment the following line to see a similar scope error to the above
27:     // info!(stdout, "a_str is {}", a_str);
```

As you can see, variable scope is similar in nature to many other languages.

### Variable Scope in Action
We will be using `cargo` to interact with most of the examples.  `cargo` is the Rust package manager and build tool.

The full code for this example is in the `ex01.rs` file in the `src/` directory.

1. Execute `cargo run ex01` at the command line in the base directory of the project.  What did you expect to see?
1. Uncomment line 20 and re-run `cargo run ex01`.  What do you see this time?
1. Comment line 20, uncomment line 27, and re-run `cargo run ex01`.  What changed from the previous results?

## B. Move
### But first...
The previous example is pretty common amongst programming languages.  The following is an example that is more unique to Rust.

Before we dive into move, take a look at the code below.  What would you expect to happen if you ran this with `cargo run`?
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

On line 5, we can see a message regarding line 4.  Again, a move is referenced in regards to the type 'String'.  But what is the 'Copy' trait?  We will get to that when we discuss borrow.

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

### **Move In Action**
## C. Borrow