//! Help generators for CodeyDog.

/// Generates help about CodeyDog.
///
/// Examples:
/// ```rust
/// about() // Prints `Usage: codeydog [options...] [command] [args...]`
/// ```
pub fn about() {
    let about = r#"
	Usage: codeydog [options...] [command] [args...]

	"#
    .trim();

    println!("{}", about);
}

/// Generates help about a command.
///
/// Examples
/// ```rust
/// help() // Prints -->
/// /*
/// Usage: codeydog help [command]
///
/// Shows information about a command and how to use it.
///  */
/// ```
pub fn help(command: Option<&String>) {
    let info_help: &str = r#"
Usage: codeydog help [command]

Shows information about a command and how to use it.
"#
    .trim();
    match command.unwrap_or(&String::new()).as_str() {
        "help" => println!("{}", info_help),
        command => println!("Unknown command: {}", command),
    }
}
