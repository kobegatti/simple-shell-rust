use std::collections::BTreeMap;

#[derive(Debug)]
pub struct TodoList {
	current_index: i32,
	items: BTreeMap<i32, String>,
}

impl TodoList {
	pub fn new() -> Self {
		TodoList { 
			current_index: 1,
			items: BTreeMap::new() 
		}
	}

	pub fn add_item(&mut self, item: Option<String>) {
		match item {
			Some(s) => { 
				self.items.insert(self.current_index, s);
				self.current_index += 1;
			},
			None => eprintln!("Error: no item entered...")
		}
	}

	pub fn print(&mut self) {
		for (idx, item) in &self.items {
			println!("{}: {}", idx, item);
		}
	}

	pub fn remove_item(&mut self, idx: Option<String>) {
		match idx {
			Some(s) => {
				match s.parse::<i32>() {
					Ok(num) => { 
						if self.items.remove(&num).is_none() {
							eprintln!("item number '{}' is not on the Todo-List...", &num);
						}
						else {
							self.current_index -= 1;
						}
					},
					Err(_) => { 
						eprintln!("Error: <item_number> is not an i32...");
					}
				}
			},
			None => eprintln!("Error: no index entered...")
		}
	}
}
