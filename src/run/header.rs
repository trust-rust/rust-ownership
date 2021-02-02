// Copyright (c) 2021 rust-own developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `rust_own` header

use colored::{Color, Colorize};
use rand::Rng;

fn random_color() -> Color {
    let num = rand::thread_rng().gen_range(0..7);

    match num {
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Blue,
        4 => Color::Magenta,
        5 => Color::Cyan,
        6 => Color::White,
        _ => Color::Red,
    }
}

/// Generate the `kca` header.
crate fn header() {
    let color = random_color();

    println!("{}", "######## ########  ##     ##  ######  ########    ########  ##     ##  ######  ######## ".color(color));
    println!("{}", "   ##    ##     ## ##     ## ##    ##    ##       ##     ## ##     ## ##    ##    ##    ".color(color));
    println!("{}", "   ##    ##     ## ##     ## ##          ##       ##     ## ##     ## ##          ##    ".color(color));
    println!("{}", "   ##    ########  ##     ##  ######     ##       ########  ##     ##  ######     ##    ".color(color));
    println!("{}", "   ##    ##   ##   ##     ##       ##    ##       ##   ##   ##     ##       ##    ##    ".color(color));
    println!("{}", "   ##    ##    ##  ##     ## ##    ##    ##       ##    ##  ##     ## ##    ##    ##    ".color(color));
    println!("{}", "   ##    ##     ##  #######   ######     ##       ##     ##  #######   ######     ##    ".color(color));


    println!();
    println!("4a61736f6e204f7a696173");
    println!();
    println!(
        "{}:    {}",
        "Build Version".bold(),
        env!("VERGEN_SEMVER").bold().green()
    );
    println!(
        "{}:  {}",
        "Build Timestamp".bold(),
        env!("VERGEN_BUILD_TIMESTAMP").bold().green()
    );
    println!(
        "{}:  {}",
        "Last Commit SHA".bold(),
        env!("VERGEN_SHA").bold().green()
    );
    println!(
        "{}: {}",
        "Last Commit Date".bold(),
        env!("VERGEN_COMMIT_DATE").bold().green()
    );
    println!();
}
