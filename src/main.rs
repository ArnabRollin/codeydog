//! Codeydog is a simple tool installer with the advantage of activating and deactivating tools.
#![deny(missing_docs)]

use about::{about, help};
use activator::{activate, deactivate};
use argparser::argparse;
use installer::{install, reinstall};
use std::{env::args, process::exit};

mod about;
mod activator;
mod argparser;
mod installer;

fn main() {
    let mut args = args();

    // Skip program name
    args.next();

    let arguments = argparse(args);

    if arguments.options.is_empty() && arguments.flags.is_empty() && arguments.arguments.is_empty()
    {
        about();
        exit(1)
    }

    if arguments.options.contains(&"help".to_string()) || arguments.flags.contains(&"h".to_string())
    {
        about();
        exit(0);
    }

    match arguments.command.as_str() {
        "help" => help(arguments.arguments.get(0)),
        "install" => install(arguments.arguments.get(0)),
        "reinstall" => reinstall(arguments.arguments.get(0)),
        "activate" => activate(arguments.arguments.get(0)),
        "deactivate" => deactivate(arguments.arguments.get(0)),
        unknown_command => eprintln!("Unknown command: {}", unknown_command),
    }
}
