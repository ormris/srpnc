use std::io::{self, Write};
use std::process;
use std::collections::HashMap;

fn parse(line: String, stack: &mut Vec<i64>, macros: &mut HashMap<String, String>, recurse: i64) {
	if line.is_empty() {
		return;
	}

	let mut it = line.split_whitespace();

	let mut macro_name = String::new();

	match it.next() {
		Some(val) => macro_name = String::from(val),
		_ => { },
	}

	match macro_name.pop() {
		Some(':') => {
            let mut macro_body = String::new();
            for token in it {
                macro_body = macro_body + " " + token;
            }
            macros.insert(macro_name, macro_body);
            return;
		},
		_ => { },
	}

	for token in line.split_whitespace() {
		match token.parse::<i64>() {
			Ok(val) => {
				stack.push(val);
				continue;
			},
			_ => { },
		}

		match token.as_ref() {
            "+" => {
                let a = match stack.pop() {
                            Some(val) => val,
                            _ => {
                                println!("stack empty");
                                continue;
                            }
                        };
                let b = match stack.pop() {
                            Some(val) => val,
                            _ => {
                                stack.push(a);
                                println!("stack empty");
                                continue;
                            }
                        };
                stack.push(a + b);
			},
			"-" => {
				let a = match stack.pop() {
                            Some(val) => val,
                            _ => {
                                println!("stack empty");
                                continue;
                            }
                        };
				let b = match stack.pop() {
                            Some(val) => val,
                            _ => {
                                stack.push(a);
                                println!("stack empty");
                                continue;
                            }
                        };
				stack.push(a - b);
			},
			"*" => {
				let a = match stack.pop() {
                            Some(val) => val,
                            _ => {
                                println!("stack empty");
                                continue;
                            }
                        };
				let b = match stack.pop() {
                            Some(val) => val,
                            _ => {
                                stack.push(a);
                                println!("stack empty");
                                continue;
                            }
                        };
				stack.push(a * b);
			},
			"/" => {
				let a = match stack.pop() {
                            Some(val) => val,
                            _ => {
                                println!("stack empty");
                                continue;
                            }
                        };
				let b = match stack.pop() {
                            Some(val) => val,
                            _ => {
                                stack.push(a);
                                println!("stack empty");
                                continue;
                            }
                        };
				stack.push(a / b);
			},
			"d" => {
				let result = match stack.pop() {
                                Some(val) => val,
                                _ => {
                                    println!("stack empty");
                                    continue;
                                }
                            };
				stack.push(result);
				stack.push(result);
			},
			"?" => {
				let result = match stack.pop() {
						Some(val) => val,
						_ => {
							println!("stack empty");
							continue;
						}
					};
				if result == 0 {
					parse(String::from("false"), stack, macros, recurse + 1);
				} else {
					parse(String::from("true"), stack, macros, recurse + 1);
				}
			},
			"clear" => {
				stack.clear();
			},
			"exit" | "quit" => {
				process::exit(0);
			},
			_ => {
				let mut program = String::new();
				let mut found = false;
				match macros.get(token) {
					Some(program_) => {
						program = program_.clone();
						found = true;
					},
					None => {
						println!("no macro of that name");
					},
				}

				if recurse > 1000 {
					println!("recursion limit reached");
				} else {
					if found {
						parse(program, stack, macros, recurse + 1);
					} 
				}
			},
		}
	}

	if recurse <= 0 && !stack.is_empty() {
		match stack.last() {
			Some(val) => println!("{}", val),
			_ => println!("stack empty"),
		}
	}
}

fn main() {
	let mut stack: Vec<i64> = Vec::new();
	let mut macros: HashMap<String, String> = HashMap::new();

	parse(String::from("true: "), &mut stack, &mut macros, 0);
	parse(String::from("false: "), &mut stack, &mut macros, 0);
	loop {
		print!("> ");
		if io::stdout().flush().is_err() {
			return;
		}

		let mut line = String::new();

		if io::stdin().read_line(&mut line).is_err() {
			return;
		}

		parse(line, &mut stack, &mut macros, 0);
	}
}
