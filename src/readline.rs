use rustyline::completion::{Completer, Pair};
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{Validator, ValidationContext, ValidationResult};
use rustyline::{Context, Helper};
use rustyline::CompletionType;

use std::collections::HashMap;
use std::path::Path;

/// A full `rustyline` helper for BakaShell.
pub struct BakaHelper {
    pub commands: Vec<String>,
    pub aliases: HashMap<String, String>,
    pub highlighter: MatchingBracketHighlighter,
    pub hinter: HistoryHinter,
}

impl BakaHelper {
    /// Construct from a list of commands and alias map.
    pub fn new(commands: Vec<String>, aliases: HashMap<String, String>) -> Self {
        BakaHelper {
            commands,
            aliases,
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
        }
    }
}

// Completion logic: commands, aliases, filenames
impl Completer for BakaHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
        // find start of current word
        let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);
        let prefix = &line[start..pos];

        let mut matches: Vec<Pair> = vec![];

        // command / alias matching
        for cmd in &self.commands {
            if cmd.starts_with(prefix) {
                matches.push(Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                });
            }
        }

        for (alias, expansion) in &self.aliases {
            if alias.starts_with(prefix) {
                matches.push(Pair {
                    display: alias.clone(),
                    replacement: expansion.clone(),
                });
            }
        }

        // filename completion
        if prefix.starts_with('.') || prefix.starts_with('/') || Path::new(prefix).exists() {
            if let Ok(dir) = std::fs::read_dir(".") {
                for entry in dir.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.starts_with(prefix) {
                            matches.push(Pair {
                                display: name.clone(),
                                replacement: name,
                            });
                        }
                    }
                }
            }
        }

        matches.sort_by(|a, b| a.display.cmp(&b.display));
        Ok((start, matches))
    }
}

// Inline hints from history
impl Hinter for BakaHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

// Syntax highlighting & prompt coloring
impl Highlighter for BakaHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> std::borrow::Cow<'b, str> {
        std::borrow::Cow::Borrowed(prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Borrowed(hint)
    }

    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str,
        _completion_type: CompletionType,
    ) -> std::borrow::Cow<'c, str> {
        std::borrow::Cow::Borrowed(candidate)
    }

    fn highlight<'l>(&self, input: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        std::borrow::Cow::Borrowed(input)
    }
}

// Validator for multi‑line support (lines ending in `\`)
impl Validator for BakaHelper {
    fn validate(
        &self,
        ctx: &mut ValidationContext,
    ) -> Result<ValidationResult, rustyline::error::ReadlineError> {
        let input = ctx.input();
        if input.ends_with("\\") {
            Ok(ValidationResult::Incomplete)
        } else {
            Ok(ValidationResult::Valid(None))
        }
    }
}

impl Helper for BakaHelper {}