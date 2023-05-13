use std::env::args_os;
use std::collections::HashMap;
use crate::about::{NAME, VERSION};
use crate::colors::gray;
use crate::errors::ArgumentError;

pub fn parse() -> (String, Vec<String>, String, Vec<String>) {
	let mut flags: Vec<String> = vec![];
	let mut args: Vec<String> = vec![];

	for arg in args_os() {
		args.push(arg.to_string_lossy().to_string());
	}

	// drop args_os?

	let exec_path: String = args.remove(0);

	// #region flags
	let flag_map: HashMap<&str, &str> = HashMap::from([
		// ("-e", "--eval"),
		("-h", "--help"),
		("-t", "--test"),
		("-v", "--version"),
	]);

	let valid_flags: _ = [
		// "--eval",
		"--help",
		"--test",
		"--version",
	];

	let unique_flags: _ = [
		"--help",
		"--version",
	];

	for (i, arg) in args.clone().iter().enumerate() {
		// env::args() panics

		println!("[debug] args[{}] = {:?}", i, arg);
		let i: usize = i + 1;
		
		if arg.len() < 2 || !arg.starts_with('-') {
			break;
		}

		if arg == "-" {
			ArgumentError!("invalid flag at position {}", i);
		}

		let mut flag: &str = &args.remove(0)[..];

		if flag.len() == 2 {
			if !flag_map.contains_key(flag) {
				ArgumentError!("unknown flag '{}'", flag);
			}
			flag = flag_map[flag];
		}
		if !valid_flags.contains(&flag) {
			ArgumentError!("unknown flag '{}'", flag);
		}
		if !unique_flags.contains(&flag) {
			flags.push(flag.to_string());
			continue;
		}
		if i != 1 {
			ArgumentError!("unexpected flag '{}' at position {}", flag, i);
		}
		match flag {
			"--help" => {
				println!("{} | {:9 } Shows this message", gray("-h"), gray("--help"));
				println!("{} | {:9 } Prints the current {} version", gray("-v"), gray("--version"), NAME);
			}
			"--version" => {
				println!("{} {}", NAME, VERSION);
			}
			_ => {
				ArgumentError!("flag '{}' not implemented yet", flag);
			}
		}
	}
	// #endregion flags

	if args.is_empty() {
		ArgumentError!("missing file path");
	}

	let file: String = args.remove(0);

	(exec_path, flags, file, args)
}