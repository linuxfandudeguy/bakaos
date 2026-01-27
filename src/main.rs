use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;

use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio};

fn run_command(cmd: &str, args: &[&str], busybox_path: &str, admin_mode: bool) {
    // Always try BusyBox first
    let busybox_status = Command::new(busybox_path)
        .arg(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match busybox_status {
        Ok(s) if s.success() => return, // ran successfully via BusyBox
        _ => {
            if admin_mode {
                // Admin mode: fallback to PATH binaries
                let path_status = Command::new(cmd)
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status();

                if let Err(e) = path_status {
                    eprintln!("bakaos(admin): {}: command not found or failed ({})", cmd, e);
                }
            } else {
                // Not admin: BusyBox failed, don't run PATH
                eprintln!("bakaos: {}: applet not found in BusyBox", cmd);
            }
        }
    }
}

fn main() {
    let busybox_path = "/usr/bin/busybox";

    let mut aliases: HashMap<String, String> = HashMap::new();
    aliases.insert("ll".into(), "ls -al".into());
    aliases.insert("help".into(), busybox_path.into()); // help runs BusyBox

    // Admin mode flag
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

                let parts: Vec<&str> = line.split_whitespace().collect();
                let mut cmd = parts[0];
                let mut args: Vec<&str> = parts[1..].to_vec();

                // Builtins
                match cmd {
                    "exit" => break,
                    "cd" => {
                        let target = args.get(0).copied().unwrap_or("/");
                        if let Err(e) = env::set_current_dir(target) {
                            eprintln!("bakaos: cd: {}", e);
                        }
                        continue;
                    }
                    "alias" => {
                        for (k, v) in &aliases {
                            println!("alias {}='{}'", k, v);
                        }
                        continue;
                    }
                    "admin" => {
                        admin_mode = !admin_mode;
                        println!(
                            "Admin mode {}",
                            if admin_mode { "enabled" } else { "disabled" }
                        );
                        continue;
                    }
                    _ => {}
                }

                // Alias expansion
                if let Some(expanded) = aliases.get(cmd) {
                    let expanded_parts: Vec<&str> = expanded.split_whitespace().collect();
                    cmd = expanded_parts[0];
                    args.splice(0..0, expanded_parts[1..].iter().copied());
                }

                // Execute command
                run_command(cmd, &args, busybox_path, admin_mode);
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
