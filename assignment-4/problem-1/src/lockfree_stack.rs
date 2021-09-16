use std::{
	ptr,
	sync::Arc,
	sync::atomic::{Ordering, AtomicPtr}
};

struct Node<T> {
	value: T,
	next: AtomicPtr<Node<T>>
}

// An atomic, lock-free, concurrent stack.
pub struct Stack<T> {
	head: AtomicPtr<Node<T>>
}

impl<T: Copy> Stack<T> {
	// Create a new stack.
	pub fn new() -> Self {
		Self {
			head: AtomicPtr::new(ptr::null_mut())
		}
	}

	// Push an item onto the stack.
	pub fn push(&self, value: T) {
		loop {
			// Attempt to push item. Try again if fail.
			if self.try_push(value) {
				return;
			}
		}
	}
	
	// Deligated attempts to push.
	fn try_push(&self, value: T) -> bool {
		let head_node = self.head.load(Ordering::SeqCst);
	
		// Create a new node with item.
		let new_node = Arc::into_raw(Arc::new(Node {
			value,
			next: AtomicPtr::new(head_node)
		})) as *mut Node<T>;
		
		// Attempt to atomically replace in stack.
		self.head.compare_and_swap(head_node, new_node, Ordering::SeqCst) == head_node
	}
	
	// Pop an item from the stack.
	pub fn pop(&self) -> Option<T> {
		loop {
			// Attempt to pop item. Try again if fail.
			let (result, value) = self.try_pop();
			
			if result {
				return value
			}
		}
	}
	
	// Deligated attempts to pop.
	fn try_pop(&self) -> (bool, Option<T>) {
		// Handle null head reference passed
		let head_node = self.head.load(Ordering::SeqCst);
		if head_node == ptr::null_mut() {
			return (true, None);
		}
		
		// Attempt to obtain node and item from stack.
		let next_node = unsafe { &*head_node }.next.load(Ordering::SeqCst);
		let value = unsafe { &*head_node }.value;
		let result = self.head.compare_and_swap(head_node, next_node, Ordering::SeqCst) == head_node;
	
		// Uncomment this line to print the deleted node list to the console.
		//if result {println!("{:p}", head_node);}
	
		if result {
			(true, Some(value))
		} else {
			(false, None)
		}
	}
}