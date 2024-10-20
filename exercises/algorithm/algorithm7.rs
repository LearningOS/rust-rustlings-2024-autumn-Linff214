/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}

impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}

	fn is_empty(&self) -> bool {
		self.size == 0
	}

	fn len(&self) -> usize {
		self.size
	}

	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}

	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}

	fn pop(&mut self) -> Option<T> {
		if self.is_empty() {
			None
		} else {
			self.size -= 1;
			self.data.pop()
		}
	}

	fn peek(&self) -> Option<&T> {
		if self.is_empty() {
			None
		} else {
			self.data.get(self.size - 1)
		}
	}
}

// Helper function to check if brackets match
fn is_matching_pair(open: char, close: char) -> bool {
	match (open, close) {
		('(', ')') => true,
		('{', '}') => true,
		('[', ']') => true,
		_ => false,
	}
}

// Function to check if the brackets in the string are balanced
fn bracket_match(bracket: &str) -> bool {
	let mut stack = Stack::new();

	for ch in bracket.chars() {
		match ch {
			'(' | '{' | '[' => {
				stack.push(ch);
			}
			')' | '}' | ']' => {
				// If the stack is empty or the top of the stack doesn't match the closing bracket, return false
				if let Some(top) = stack.pop() {
					if !is_matching_pair(top, ch) {
						return false;
					}
				} else {
					return false; // No opening bracket for the closing one
				}
			}
			_ => continue, // Ignore non-bracket characters
		}
	}

	// If the stack is empty, all brackets are matched
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1() {
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s), true);
	}

	#[test]
	fn bracket_matching_2() {
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_3() {
		let s = "{{([])}}";
		assert_eq!(bracket_match(s), true);
	}

	#[test]
	fn bracket_matching_4() {
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_5() {
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s), false);
	}

	#[test]
	fn bracket_matching_6() {
		let s = "";
		assert_eq!(bracket_match(s), true);
	}
}
