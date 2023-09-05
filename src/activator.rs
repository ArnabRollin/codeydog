//! Tool Activator and Deactivator for CodeyDog.

use std::{env, fs, path::Path, process::exit};

/// Activates a tool.
///
/// Examples:
/// ```rust
/// activate("task") // This activates the `task` tool.
/// ```
pub fn activate(tool: Option<&String>) {
    let tool_default = String::new();
    let tool = tool.unwrap_or(&tool_default).as_str();

    if tool == "" {
        eprintln!("Error: Please provide a tool to activate!");
        exit(1);
    }

    let home = env::var("HOME").expect("Error: HOME directory not found!");
    let tool_dir = Path::new(&home).join(".codeydog/");

    match fs::create_dir_all(&tool_dir.join("activate")) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }

    match fs::rename(tool_dir.join(tool), tool_dir.join("activate").join(tool)) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}

/// Deactivates a tool.
///
/// Examples:
/// ```rust
/// deactivate("task") // This deactivates the `task` tool.
/// ```
pub fn deactivate(tool: Option<&String>) {
    let tool_default = String::new();
    let tool = tool.unwrap_or(&tool_default).as_str();

    if tool == "" {
        eprintln!("Error: Please provide a tool to activate!");
        exit(1);
    }

    let home = env::var("HOME").expect("Error: HOME directory not found!");
    let tool_dir = Path::new(&home).join(".codeydog/");

    match fs::create_dir_all(&tool_dir.join("activate")) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }

    match fs::rename(tool_dir.join("activate").join(tool), tool_dir.join(tool)) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
