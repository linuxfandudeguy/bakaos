use glob::glob;

/// Expand wildcards in arguments (like Bash)
pub fn expand_globs(args: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();

    for arg in args {
        if arg.contains('*') || arg.contains('?') || arg.contains('[') {
            // Try globbing
            match glob(&arg) {
                Ok(paths) => {
                    let mut matched = false;
                    for entry in paths {
                        if let Ok(path) = entry {
                            result.push(path.to_string_lossy().to_string());
                            matched = true;
                        }
                    }
                    if !matched {
                        // No matches, keep literal
                        result.push(arg);
                    }
                }
                Err(_) => result.push(arg),
            }
        } else {
            // Literal argument
            result.push(arg);
        }
    }

    result
}
