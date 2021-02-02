# Introduction

A brief introduction about memory safety without GC (a distinguishing feature of Rust)

# Install

How to install rust.  How to install visual studio code.

# Ownership in Rust

A brief description about ownership.

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.


# Example 1
## Variable Scope
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