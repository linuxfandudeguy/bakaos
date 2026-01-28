use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;

use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

mod pipes;
use pipes::run_pipeline;

mod globb;
use globb::expand_globs;

fn run_command(cmd: &str, args: &[&str], busybox_path: &str, admin_mode: bool) {
    // Try BusyBox first
    let status = std::process::Command::new(busybox_path)
        .arg(cmd)
        .args(args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => return,
        _ => {
            if admin_mode {
                // Admin fallback to PATH binaries
                if let Err(e) = std::process::Command::new(cmd)
                    .args(args)
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .status()
                {
                    eprintln!("bakaos(admin): {}: command not found ({})", cmd, e);
                }
            } else {
                eprintln!("bakaos: {}: applet not found in BusyBox", cmd);
            }
        }
    }
}

/// Expand aliases and return Vec<String>
fn expand_alias(cmd_parts: Vec<&str>, aliases: &HashMap<String, String>) -> Vec<String> {
    if cmd_parts.is_empty() {
        return vec![];
    }
    let cmd = cmd_parts[0];
    if let Some(expanded) = aliases.get(cmd) {
        let mut expanded_parts: Vec<String> =
            expanded.split_whitespace().map(|s| s.to_string()).collect();
        expanded_parts.extend(cmd_parts[1..].iter().map(|s| s.to_string()));
        expanded_parts
    } else {
        cmd_parts.iter().map(|s| s.to_string()).collect()
    }
}

fn main() {
    let busybox_path = "/usr/bin/busybox";
    let mut aliases: HashMap<String, String> = HashMap::new();
    aliases.insert("ll".into(), "ls -al".into());
    aliases.insert("help".into(), busybox_path.into());

    let mut admin_mode = false;

    println!("\x1b[1;32mバカOS  (bakashell)\x1b[0m");
    println!("BusyBox: {}", busybox_path);
    println!("Type 'help' to list commands (BusyBox).");
    println!("Type 'exit' to quit.");
    println!("Type 'admin' to toggle admin mode.\n");

    let mut rl: Editor<(), DefaultHistory> =
        Editor::new().expect("failed to create editor");
    let _ = rl.load_history(".bakaos_history");

    loop {
        let cwd = env::current_dir().unwrap_or_else(|_| "/".into());
        let prompt = if admin_mode {
            format!("\x1b[1;31mバカ(admin){}$ \x1b[0m", cwd.display())
        } else {
            format!("\x1b[1;34mバカ:{}$ \x1b[0m", cwd.display())
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line).ok();

                // Builtins
                if line == "exit" {
                    break;
                } else if line.starts_with("cd") {
                    let args: Vec<&str> = line.split_whitespace().collect();
                    let target = args.get(1).copied().unwrap_or("/");
                    if let Err(e) = env::set_current_dir(target) {
                        eprintln!("bakaos: cd: {}", e);
                    }
                    continue;
                } else if line == "alias" {
                    for (k, v) in &aliases {
                        println!("alias {}='{}'", k, v);
                    }
                    continue;
                } else if line == "admin" {
                    admin_mode = !admin_mode;
                    println!(
                        "Admin mode {}",
                        if admin_mode { "enabled" } else { "disabled" }
                    );
                    continue;
                } else if line == "clear" {
                    // Clear screen
                   print!("\x1B[2J\x1B[H"); // clear screen + move cursor to top-left
                   io::stdout().flush().unwrap(); // flush to terminal
                    continue;
                }

                // Pipeline support
                if line.contains('|') {
                    let pipeline: Vec<Vec<String>> = line
                        .split('|')
                        .map(|cmd| {
                            let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
                            let parts = expand_alias(parts, &aliases);
                            let parts: Vec<String> =
                                parts.into_iter().map(|s| s.to_string()).collect();
                            expand_globs(parts)
                        })
                        .collect();

                    run_pipeline(pipeline, busybox_path, admin_mode);
                } else {
                    // Single command
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let parts = expand_alias(parts, &aliases);
                    let mut parts: Vec<String> = parts.into_iter().map(|s| s.to_string()).collect();

                    // Glob expansion
                    parts = expand_globs(parts);

                    let cmd = &parts[0];
                    let args: Vec<&str> = parts[1..].iter().map(|s| s.as_str()).collect();
                    run_command(cmd, &args, busybox_path, admin_mode);
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("bakaos: error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(".bakaos_history");
}
