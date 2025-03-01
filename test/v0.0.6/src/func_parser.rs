use crate::{
	context::Context,
	dict::Id,
	errors::SyntaxError,
	params::{Params, Param},
	types::Type,
	expressions::parse_expr
};

#[derive(Debug)]
pub enum Statment {
	#[allow(unused)]
	Call {
		name: Id,
		params: Params
	}
}

#[allow(unused)]
#[derive(Debug)]
pub struct Function {
	pub name: Id,
	params: Params,
	returns: Option<Type>,
	pub body: Vec<Statment>
}

pub fn parse_function(ctx: &mut Context) -> Function {
	ctx.ignore_spaces();

	#[allow(unused_mut)]
	let mut function = Function {
		name: ctx.collect_word(),
		params: Params::new(),
		returns: None,
		body: Vec::new()
	};

	ctx.ignore_spaces();

	if ctx.ch == '<' {
		while ctx.idx < ctx.char_count {
			ctx.ignore_spaces();
			match ctx.ch {
				'A'..='Z' | 'a'..='z' | '_' => {}
				'>' => {
					break;
				}
				_ => {
					SyntaxError!(ctx, "expected an identifier, got {:?}", ctx.ch);
				}
			}
			ctx.next_char();
		}
	}

	if ctx.ch != '(' {
		SyntaxError!(ctx, "expected '(', got {:?}", ctx.ch);
	}

	ctx.next_char();
	ctx.ignore_spaces();

	while ctx.idx < ctx.char_count {
		match ctx.ch {
			')' => {
				break;
			}
			'.' => {}
			'a'..='z' | '_' | 'A'..='Z' => {
				let mut param: Param = Param::new(ctx.collect_word(), None, None);
				ctx.ignore_spaces();
				if ctx.ch == ':' {
					let expr = parse_expr(ctx);
					todo!("x");
				}
			}
			_ => {
				SyntaxError!(ctx, "expected ')', an identifier, or '...'; got {:?}", ctx.ch);
			}
		}
		ctx.next_char();
	} // parse params
	ctx.next_char();
	ctx.ignore_spaces();
	while ctx.idx < ctx.char_count {
		match ctx.ch {
			'-' => {
				ctx.next_char();
				if ctx.ch != '>' {
					SyntaxError!(ctx, "expected '>' (to complete '->'), got {:?}", ctx.ch);
				}
				ctx.next_char();
				ctx.ignore_spaces();
				SyntaxError!(ctx, "todo: parse types");
			}
			'{' => {
				ctx.next_char();
				ctx.ignore_spaces();
				match ctx.ch {
					'}' => {
						break;
					}
					'a'..='z' | 'A'..='Z' | '_' => {
						let word = ctx.collect_word();
						ctx.ignore_spaces();
						match word.as_str() {
							"if" => {
								SyntaxError!(ctx, "todo: if");
							}
							"const" | "var" | "namespace" | "fun" | "struct" | "extend" => {
								SyntaxError!(ctx, "todo");
							}
							_identifier => {
								match ctx.ch {
									'=' => {
										SyntaxError!(ctx, "todo: assignment");
									}
									_ => {
										SyntaxError!(ctx, "unexpected character {:?}", ctx.ch);
									}
								}
							} // identifier
						} // match word
					} // collect word
					'+' => {
						SyntaxError!(ctx, "todo: add-assign");
					}
					'-' => {
						SyntaxError!(ctx, "todo: sub-assign");
					}
					_ => {
						SyntaxError!(ctx, "unexpected character {:?}", ctx.ch);
					}
				}
			} // '{'
			_ => {
				SyntaxError!(ctx, "expected '->' or '{{', got {:?}", ctx.ch);
			}
		}
	}
	println!("end of function {:?}", function.name);
	function
}
