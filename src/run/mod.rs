// Copyright (c) 2021 rust-own developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `rust_own` runtime

mod cli;
mod header;
mod log;

use crate::{
    error::{ErrCode, Error, Result},
    ex01::ex01_scope,
    ex02::ex02_move,
    ex03::ex03_move_fn,
    ex04::ex04_fn_return,
    ex05::ex05_move_only,
    ex06::ex06_borrow,
    ex07::ex07_modify,
    ex08::ex08_mutable_reference,
};
use colored::Colorize;
use slog::info;
use std::fs::DirBuilder;

crate fn run() -> Result<()> {
    // Parse the command line
    let matches = cli::app().get_matches_safe()?;

    // Setup the config directory
    let mut config_dir = dirs2::config_dir().ok_or_else(no_config_dir)?;
    config_dir.push("rust-own");
    let _ = DirBuilder::new()
        .recursive(true)
        .create(config_dir.clone())?;

    // Output the pretty header
    header::header();

    // Setup logging
    let stdout = log::initialize(&matches)?;
    info!(stdout, "tr Started!");

    match matches.subcommand() {
        ("ex01", _sub_m) => {
            info!(stdout, "{}", "Example 1 - Variable Scope".blue());
            ex01_scope();
        }
        ("ex02", _sub_m) => {
            info!(stdout, "{}", "Example 2 - Move".blue());
            ex02_move();
        }
        ("ex03", _sub_m) => {
            info!(
                stdout,
                "{}",
                "Example 3 - Functions Arguments are Moved".blue()
            );
            ex03_move_fn();
        }
        ("ex04", _sub_m) => {
            info!(
                stdout,
                "{}",
                "Example 4 - Function Return Values are Moved".blue()
            );
            ex04_fn_return();
        }
        ("ex05", _sub_m) => {
            info!(stdout, "{}", "Example 5 - If you had Move only".blue());
            ex05_move_only();
        }
        ("ex06", _sub_m) => {
            info!(stdout, "{}", "Example 6 - Borrow".blue());
            ex06_borrow();
        }
        ("ex07", _sub_m) => {
            info!(stdout, "{}", "Example 7 - Modify".blue());
            ex07_modify();
        }
        ("ex08", _sub_m) => {
            info!(stdout, "{}", "Example 8 - Mutable References".blue());
            ex08_mutable_reference();
        }
        _ => {}
    }
    Ok(())
}

fn no_config_dir() -> Error {
    Error::new(ErrCode::Protocol, "There is no valid config dir", None)
}
