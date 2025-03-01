use std::rc::Rc;
use crate::{
  args::{ ParsedArgs, parse_args },
  files::read_file,
  aliases::{ Id, Arguments },
  context::Context,
  namespaces::parse_namespace,
  values::Value,
  globals::make_global,
  expressions::Expression
};

/// TODO:
/// stack: Array<Dict> = [global, ...uppers, current, recent (maybe)]
/// check function arguments' types, quantity, etc
/// regex

fn main() {
  let mut args: ParsedArgs = parse_args();	
  let code = read_file(&mut args.main_path);

  println!("args = {args:#?}");
  println!("code = {code:?}");
  println!();

  let file_id = Id::from(args.main_path
    .file_name()
    .unwrap() // guaranteed to be a file
    .to_string_lossy()
    .to_string());
  let path_id = Id::from(args.main_path
    .to_string_lossy()
    .to_string());

  println!("file_id = {file_id:?}");
  println!("path_id = {path_id:?}");
  println!();

  let mut ctx = Context::new(path_id, &code);
  ctx.stack.preppend(make_global());
  let main = parse_namespace(&mut ctx, file_id);
  
  let entrypoint = match main.public.get(&Id::from("main")).cloned() {
    None => reference_error!("main function not found"; ctx),
    Some(value) => value
  };
  ctx.stack.preppend(main.public);
  dbg!(&ctx.stack);

  if let Value::Function(function) = entrypoint {
    let argv = Value::Array(args.args.iter().map(|v| Value::Id(Rc::clone(v))).collect());
    let arguments: Arguments = Rc::from([Expression::Literal(argv)]);
    function.call(arguments, &mut ctx);
  } else {
    type_error!("missing main function"; ctx);
  }
}

mod about;
mod macros;
mod args;
mod colors;
mod aliases;
mod files;
mod context;
mod namespaces;
mod values;
mod functions;
mod statements;
mod numbers;
mod expressions;
mod eval;
mod stack;
mod structs;
mod instances;
mod globals;

#[cfg(test)]
mod tests;