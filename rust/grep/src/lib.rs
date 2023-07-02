use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::{Error, Ok};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,
    names: bool,
    case_insensitive: bool,
    invert: bool,
    entire_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            line_numbers: flags.contains(&"-n"),
            names: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            entire_lines: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res = Vec::new();
    let pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };
    'files: for file_name in files {
        let file = File::open(file_name)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        for (i, line) in contents.lines().enumerate() {
            let line_correct_case = if flags.case_insensitive {
                line.to_lowercase()
            } else {
                line.to_string()
            };
            if ((!flags.entire_lines && line_correct_case.contains(&pattern))
                || line_correct_case == pattern)
                ^ flags.invert
            {
                if flags.names {
                    res.push(file_name.to_string());
                    continue 'files;
                } else {
                    res.push(format!(
                        "{}{}{}",
                        if files.len() > 1 {
                            format!("{}:", file_name)
                        } else {
                            "".to_string()
                        },
                        if flags.line_numbers {
                            format!("{}:", i + 1)
                        } else {
                            "".to_string()
                        },
                        line
                    ));
                }
            }
        }
    }
    Ok(res)
}
