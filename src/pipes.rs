use duct::cmd;

/// Run a pipeline of commands connected by pipes
pub fn run_pipeline(commands: Vec<Vec<String>>, busybox_path: &str, _admin_mode: bool) {
    if commands.is_empty() {
        return;
    }

    let mut pipeline = None;

    for cmd_parts in commands {
        if cmd_parts.is_empty() {
            continue;
        }

        let cmd_name = &cmd_parts[0];
        let args = &cmd_parts[1..];

        // Convert all arguments to &str
        let duct_args: Vec<&str> = std::iter::once(cmd_name.as_str())
            .chain(args.iter().map(|s| s.as_str()))
            .collect();

        // BusyBox first
        let duct_cmd = cmd(busybox_path, duct_args);

        // Chain with previous pipeline
        pipeline = match pipeline {
            None => Some(duct_cmd),
            Some(prev) => Some(prev.pipe(duct_cmd)),
        };
    }

    // Run the pipeline
    if let Some(p) = pipeline {
        if let Err(e) = p.run() {
            eprintln!("bakaos: pipeline failed: {}", e);
        }
    }
}
