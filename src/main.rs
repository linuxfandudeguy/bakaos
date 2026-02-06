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

mod readline;
use readline::BakaHelper;

/// Run a command via BusyBox first, fallback to system command if admin_mode
fn run_command(cmd: &str, args: &[&str], busybox_path: &str, admin_mode: bool) {
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

/// Expand aliases recursively
fn expand_alias(cmd_parts: Vec<&str>, aliases: &HashMap<String, String>) -> Vec<String> {
    if cmd_parts.is_empty() {
        return vec![];
    }
    let cmd = cmd_parts[0];
    if let Some(expanded) = aliases.get(cmd) {
        let mut expanded_parts: Vec<String> =
            expanded.split_whitespace().map(|s| s.to_string()).collect();
        expanded_parts.extend(cmd_parts[1..].iter().map(|s| s.to_string()));

        // Check recursive expansion
        let expanded_parts_str: Vec<&str> = expanded_parts.iter().map(|s| s.as_str()).collect();
        return expand_alias(expanded_parts_str, aliases);
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
    let mut sh_mode = false;

    println!("\x1b[1;32mヽ(#`Д´)ﾉ バカOS (bakashell)\x1b[0m");
    println!("BusyBox: {}", busybox_path);
    println!("Type 'help' to list commands (BusyBox).");
    println!("Type 'exit' to quit.");
    println!("Type 'admin' to toggle admin mode.");
    println!("Press \"CTRL+L\" to clear the terminal");
    println!("Type 'sh' to enter POSIX compatibility mode (sh-5.3).\n");

    let commands = vec![
        "cd".into(),
        "exit".into(),
        "alias".into(),
        "admin".into(),
        "clear".into(),
        "sh".into(),
    ];
    let helper = BakaHelper::new(commands, aliases.clone());

    let mut rl: Editor<BakaHelper, DefaultHistory> =
        Editor::new().expect("failed to create editor");
    rl.set_helper(Some(helper));
    let _ = rl.load_history(".bakaos_history");

    loop {
        let cwd = env::current_dir().unwrap_or_else(|_| "/".into());

        // --- Stylish バカ prompt ---
        let prompt = if sh_mode {
            format!(
                "\x1b[1;35mヽ(°□° )ノ バカ(sh-5.3) [{}]$ \x1b[0m",
                cwd.display()
            )
        } else if admin_mode {
            format!(
                "\x1b[1;31m(；￣Д￣) バカ(admin) [{}]# \x1b[0m",
                cwd.display()
            )
        } else {
            format!("\x1b[1;34mヽ(#`Д´)ﾉ バカ [{}]$ \x1b[0m", cwd.display())
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line).ok();

                // --- SH mode ---
                if line == "sh" && !sh_mode {
                    sh_mode = true;
                    println!("ヽ(；￣Д￣)ノ Entering shell mode (sh-5.3). Type 'exit' to return.");
                    continue;
                }

                if sh_mode {
                    if line == "exit" {
                        sh_mode = false;
                        println!("(￣Д￣)ﾉ Returning to バカShell.");
                        continue;
                    }
                    let status = std::process::Command::new("/bin/sh")
                        .arg("-c")
                        .arg(line)
                        .stdin(std::process::Stdio::inherit())
                        .stdout(std::process::Stdio::inherit())
                        .stderr(std::process::Stdio::inherit())
                        .status();

                    if let Err(e) = status {
                        eprintln!("sh: command failed: {}", e);
                    }
                    continue;
                }

                // --- Builtins ---
                if line == "exit" {
                    println!("ヽ(´Д｀ヽ) Bye-bye!");
                    break;
                } else if line.starts_with("cd") {
                    let args: Vec<&str> = line.split_whitespace().collect();
                    let target = args.get(1).copied().unwrap_or("/");
                    if let Err(e) = env::set_current_dir(target) {
                        eprintln!("bakaos: cd: {}", e);
                    }
                    continue;
                } else if line.starts_with("alias") {
                    let args: Vec<&str> = line.split_whitespace().collect();

                    if args.len() == 1 {
                        println!("(*≧ω≦) Current aliases:");
                        for (k, v) in &aliases {
                            println!("alias {}='{}'", k, v);
                        }
                    } else {
                        // alias name="command"
                        for arg in &args[1..] {
                            if let Some((name, value)) = arg.split_once('=') {
                                let value = value.trim_matches('"');
                                aliases.insert(name.to_string(), value.to_string());
                                println!("Alias created: {}='{}'", name, value);
                            } else {
                                eprintln!("bakaos: invalid alias format: {}", arg);
                            }
                        }
                    }
                    continue;
                } else if line == "admin" {
                    admin_mode = !admin_mode;
                    println!(
                        "(¬‿¬) Admin mode {}",
                        if admin_mode { "enabled" } else { "disabled" }
                    );
                    continue;
                } else if line == "clear" {
                    print!("\x1B[2J\x1B[H");
                    io::stdout().flush().unwrap();
                    continue;
                }

                // --- Pipeline or single command ---
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
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let parts = expand_alias(parts, &aliases);
                    let mut parts: Vec<String> = parts.into_iter().map(|s| s.to_string()).collect();
                    parts = expand_globs(parts);

                    if parts.is_empty() {
                        continue;
                    }

                    let cmd = &parts[0];
                    let args: Vec<&str> = parts[1..].iter().map(|s| s.as_str()).collect();

                    // Run aliases first, then real commands
                    if aliases.contains_key(cmd) {
                        let expanded: Vec<String> = expand_alias(vec![cmd], &aliases);
                        let cmd = &expanded[0];
                        let args: Vec<&str> = expanded[1..].iter().map(|s| s.as_str()).collect();
                        run_command(cmd, &args, busybox_path, admin_mode);
                    } else {
                        run_command(cmd, &args, busybox_path, admin_mode);
                    }
                }
            }

            Err(ReadlineError::Interrupted) => println!("^C ヽ(｀⌒´)ノ"),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("bakaos: error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(".bakaos_history");
}
