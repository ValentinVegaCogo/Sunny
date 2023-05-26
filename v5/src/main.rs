mod parse_args;
mod about;
mod stdlib;
mod types;
mod colors;
mod paths;
mod files;
mod toml_stuff;
mod errors;
mod namespaces;
mod words;
mod functions;
mod global;

use crate::types::{Type, Value, Dict};
use crate::errors::{debug, Warning};
use crate::files::read;
use crate::namespaces::parse_namespace;
use std::collections::HashMap;

// auto-imported toml, crossterm

fn main() {
	#[allow(unused)]
	let (exec_path, flags, file_path, args): _ = parse_args::parse();

	let exec_path: String = paths::resolve(exec_path);
	debug!("exec_path = {exec_path:?}");

	let file_path: String = paths::resolve_filename(if file_path == "." { String::from("main.sny") } else { file_path });
	debug!("file_path = {file_path:?}");

	let workspace: _ = paths::dirname(file_path.clone());

	let cfg_path: _ = workspace.join("Sunny.toml");
	if cfg_path.exists() {
		debug!("cfg file found: {cfg_path:?}");
		let cfg_file: String = files::read(cfg_path.to_string_lossy().to_string());
		let cfg: toml::map::Map<String, toml::Value> = toml_stuff::parse_toml(cfg_file);
		debug!("cfg = {cfg:?}");
	}
	// let std = std::init();

	#[allow(unused)]
	let mut global: Dict = global::init!();

	let chars: Vec<char> = read(file_path).chars().collect();
	debug!("code = {chars:?}");

	println!();

	#[allow(unused)]
	let mut i: usize = 0;

	let main: HashMap<String, Value> = parse_namespace(&chars, &mut i);

	if main.contains_key("exports") {
		Warning!("exported values found in the main file");
	}
}
