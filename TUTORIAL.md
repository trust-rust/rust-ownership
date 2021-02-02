# Introduction

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
# Installation

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
# Memory Safety and Ownership

One of the distinguishing features of Rust is memory safety at performance levels near the C language.
## Memory Safety
For the uninitiated, memory safety refers to a class of bugs that your program is protected from while at runtime.  These bugs include some well known issues such as buffer overflows, use after free, and dangling pointers.  Normally, memory safety is achieved by means of garbage collection or GC.  A full discussion about GC is out of scope here, but GC can cause other issues (raise your hand if you ever hooked up to a memory staved JVM to watch the GC thrash).  Rust on the other hand achieves memory safety through the concept of ownership where memory safety checks happen at compile time.  Completing the checks at compile time means there is no runtime impact on performance due to memory safety checks in.

## Ownership
So what is ownership?  First the ownership rules:

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

That's it.  Now, we will dive into some ownership examples to illustrate in a more concerete manner what ownership is.

# The Code
These examples loosely follow the examples documented in the [Rust book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).  I hightly recommend taking some time to peruse the entire book, it's very good.  I've added some additional examples where I've run into ownership conundrums in the past.

The code for these examples is hosted at https://github.com/trust-rust/rust-ownership.git.

For each example, there will be a corresponding branch, e.g. Example 1 will have code in the `ex01` branch.

I suggest switching to the appropriate branch for each example so you can tinker with actual code.

# Example 1 - Variable Scope
The full code for this example is in the `ex01.rs` file in the `src/` directory.

```rust
{
    let s = "hello";
}
```

# Example 2
# Taking Ownership / Move
```rust
{
    let x = 5;
    let y = x;
}