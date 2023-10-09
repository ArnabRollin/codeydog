//! Installer for CodeyDog.

use git2::Repository;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::{env, fs};

/// Installs a package.
///
/// Examples:
/// ```rust
/// install("task") // The `task` package is installed now!
/// ```
pub fn install(pkg: Option<&String>) {
    let pkg_default = String::new();
    let pkg = pkg.unwrap_or(&pkg_default).as_str();
    let packages = vec!["task-rust", "dwn"];

    let _repo = match pkg {
        "" => {
            eprintln!("Error: Please provide a package to install!");
            exit(1)
        }
        pkg => {
            if !packages.contains(&pkg) {
                eprintln!("Error: Package `{pkg}` not found!");
                exit(1);
            }
            let home = env::var("HOME").expect("Error: HOME directory not found!");
            let install_dir = Path::new(&home)
                .join(".codeydog_tools/")
                .join(format!("{}/", pkg));

            let mut tools = match fs::read_dir(&install_dir.join("..")) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            };

            match tools.find(|el| match el {
                Ok(entry) => entry.file_name() == pkg,
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1)
                }
            }) {
                Some(_) => {
                    eprintln!("Package {} is already installed!", pkg);
                    eprintln!("Use `codeydog reinstall {}` to reinstall it.", pkg);
                    exit(1);
                }
                None => {}
            }

            let tool_dir = Path::new(&home).join(".codeydog/");

            match fs::create_dir_all(&tool_dir) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1)
                }
            }

            clone_repo(
                format!(
                    "https://github.com/ArnabRollin/{}",
                    if pkg == "task-rust" { "Task-rust" } else { pkg }
                )
                .as_str(),
                &install_dir,
            );
            match Command::new("cargo")
                .arg("build")
                .arg("--release")
                .arg(format!(
                    "--manifest-path={}/Cargo.toml",
                    install_dir
                        .to_str()
                        .expect("Error: The install path is not valid unicode!")
                ))
                .output()
            {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }

            println!("Copying binary to binaries directory...");
            match fs::copy(
                install_dir.join(format!(
                    "target/release/{}",
                    if pkg == "task-rust" { "task" } else { pkg }
                )),
                install_dir.join(tool_dir.join(if pkg == "task-rust" { "task" } else { pkg })),
            ) {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e),
            }

            println!("Done!");

            println!("Use `codeydog activate {pkg}` to use this tool.");
            println!("Note: If you still can't access the binary using `{pkg}`, the Activated binaries directory might not be included in the PATH.");
            println!(
                "Please use `echo 'export PATH=\"{}:${{PATH}}\"' >> [your_config_file]` and reload your terminal to add it to the PATH. (Replace [your_config_file] with your config file e.g. `.zshrc`",
                tool_dir
                    .join("activate")
                    .to_str()
                    .expect("Error: Tool install dir is not valid unicode!")
            )
        }
    };
}

/// Installs a package.
///
/// Examples:
/// ```rust
/// reinstall("task") // The `task` package is reinstalled now!
/// ```
pub fn reinstall(pkg: Option<&String>) {
    let pkg_default = String::new();
    let pkg = pkg.unwrap_or(&pkg_default).as_str();
    let packages = vec!["task-rust", "dwn"];

    let _repo = match pkg {
        "" => {
            eprintln!("Error: Please provide a package to reinstall!");
            exit(1)
        }
        pkg => {
            if !packages.contains(&pkg) {
                eprintln!("Error: Package `{pkg}` not found!");
                exit(1);
            }
            let home = env::var("HOME").expect("Error: HOME directory not found!");
            let install_dir = Path::new(&home)
                .join(".codeydog_tools/")
                .join(format!("{}/", pkg));

            if !install_dir.exists() {
                eprintln!("The package {} is not installed!", pkg);
                eprintln!("Please use `codeydog install {pkg} to install it.`");
            }

            let tool_dir = Path::new(&home).join(".codeydog/");

            match fs::create_dir_all(&tool_dir) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1)
                }
            }

            clone_repo(
                format!(
                    "https://github.com/ArnabRollin/{}",
                    if pkg == "task-rust" { "Task-rust" } else { pkg }
                )
                .as_str(),
                &install_dir,
            );
            match Command::new("cargo")
                .arg("build")
                .arg("--release")
                .arg(format!(
                    "--manifest-path={}/Cargo.toml",
                    install_dir
                        .to_str()
                        .expect("Error: The reinstall path is not valid unicode!")
                ))
                .output()
            {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }

            println!("Copying binary to binaries directory...");
            match fs::copy(
                install_dir.join(format!(
                    "target/release/{}",
                    if pkg == "task-rust" { "task" } else { pkg }
                )),
                install_dir.join(tool_dir.join(if pkg == "task-rust" { "task" } else { pkg })),
            ) {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e),
            }

            println!("Done!");

            println!("Use `codeydog activate {pkg}` to use this tool.");
            println!("Note: If you still can't access the binary using `{pkg}`, the Activated binaries directory might not be included in the PATH.");
            println!(
                "Please use `echo 'export PATH=\"{}:${{PATH}}\"' >> [your_config_file]` and reload your terminal to add it to the PATH. (Replace [your_config_file] with your config file e.g. `.zshrc`",
                tool_dir
                    .join("activate")
                    .to_str()
                    .expect("Error: Tool install dir is not valid unicode!")
            )
        }
    };
}

/// Clone a git repository fancily.
///
/// Examples:
/// ```rust
/// clone_repo("https://github.com/ArnabRollin/Task-rust", "task-rust") // This clones "https://github.com/ArnabRollin/Task-rust" into the "task-rust" folder.
/// ```
fn clone_repo(url: &str, into: &PathBuf) -> Repository {
    println!("Trusting {url} ...");
    println!("Fetching {url} ...");
    println!("Putting package in {:#?} ...", into);

    if into.exists() {
        match fs::remove_dir_all(into) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        };
    }

    match Repository::clone(url, into) {
        Ok(repo) => repo,
        Err(e) => panic!("Error: Failed to clone: {}", e),
    }
}
