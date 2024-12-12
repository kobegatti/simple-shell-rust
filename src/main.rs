use std::io::{self, Write};

mod todo_list;

const ADD: &str = "add";
const EMPTY_STRING: &str = "";
const EXIT: &str = "exit";
const HELP: &str = "help";
const PRINT: &str = "print";
const REMOVE: &str = "remove";


fn main() {
	let mut todo_map = todo_list::TodoList::new();

	loop {
    	print!("Todo-List> ");
		io::stdout().flush().unwrap();
	
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(..) => {
				let (cmd, arg) = get_cmd_and_arg(input);

				match &cmd[..] {
					ADD => todo_map.add_item(arg),
					EMPTY_STRING => continue,
					EXIT => break, 
					HELP => print_usage(),
					PRINT => todo_map.print(),
					REMOVE => todo_map.remove_item(arg),
					_ => println!("{}", "Unknown command. Type 'help' for usage.")
				}
			}
			Err(error) => println!("error: {error}"),
		}
	}
}

fn get_cmd_and_arg(input: String) -> (String, Option<String>) {
	let trimmed_input = input.trim_end();
	
	match trimmed_input.split_once(char::is_whitespace) {
		Some((cmd, arg)) => {
			(String::from(cmd), Some(String::from(arg)))
		}
		None => {
			(String::from(trimmed_input), None)
		}
	}
}

fn print_usage() {
	println!("Usage:");
	println!("\tadd <item> - Add an item to the Todo-List.");
	println!("\tremove <item_number> - Remove an item to the Todo-List.");
	println!("\tprint - Print all items on the Todo-List.");
	println!("\texit - Exit the program.");
	println!("\thelp - Show this help message.");
}
