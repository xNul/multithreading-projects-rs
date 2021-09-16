/* Blake Wyatt
   COP 6616
   Assignment 3: Problem 2 */

use std::{
	time::Instant,
	ptr,
	thread,
	sync::Arc,
	sync::atomic::{Ordering, AtomicBool, AtomicUsize, AtomicPtr},
	collections::VecDeque
};
use rand::prelude::*;
use atomic_markable_ptr::*;

struct Node {
	tag: i32,
	next: AtomicMarkablePtr<Node>
}

fn main() {
	// Run problem 2
	atmospheric_temperature_reading_module();
}

fn atmospheric_temperature_reading_module() {
	// Thread and async data initialization
	let mut handles = vec![];

	// Start timer
	let now = Instant::now();
	println!("Starting the Atmospheric Temperature Reading Module report simulation");

	let list = Arc::new(AtomicMarkablePtr::new(Arc::into_raw(Arc::new(Node {tag: -1, next: AtomicMarkablePtr::new(ptr::null_mut(), false)})) as *mut _, false));

	for _ in 0..8 {
		let list = list.clone();

		// Thread code
		let handle = thread::spawn(move || {
			let mut rng = rand::thread_rng();

			// Interval of one minute for each sensor. There is
			// 60 minutes in an hour and so we simulate 60 inputs per sensor
			for _ in 0..60 {
				let temperature_sample = rng.gen_range(-100, 70);

				add(&list, temperature_sample+100);
			}
		});

		handles.push(handle);
	}

	// Main thread awaits the completion of all other threads.
    for handle in handles {
        handle.join().unwrap();
	}

	// Use data to generate the hourly report
	generate_report(&list);

	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	println!("Program Success -- Temperature report simulation completed");
	println!("Total execution time: {}", execution_time);
}

fn generate_report(head: &AtomicMarkablePtr<Node>) {
	// Handle null head reference passed
	let markable_ptr = head.load(Ordering::SeqCst);
	let mut pred: *mut Node = markable_ptr.0;
	if pred == ptr::null_mut() {
		return;
	}
	// Handle empty list passed
	let markable_ptr = unsafe { &*pred }.next.load(Ordering::SeqCst);
	let mut curr: *mut Node = markable_ptr.0;
	let mut succ: *mut Node; let mut marked = false; let mut tag = -1;

	// Collect top 5 lowest temperatures
	let mut highest_temperatures: VecDeque<i32> = VecDeque::new();
	let mut lowest_temperatures: Vec<i32> = Vec::new();
	for _ in 0..5 {
		if curr == ptr::null_mut() {
			break;
		}

		// Obtain successor node
		let marked_ptr = unsafe { &*curr }.next.load(Ordering::SeqCst);
		succ = marked_ptr.0;
		marked = marked_ptr.1;

		// Check for ordered location in the list
		let ctag = unsafe { &*curr }.tag-100;
		//println!("{}", ctag);
		lowest_temperatures.push(ctag);
		highest_temperatures.push_back(ctag);

		// Continue through the list
		pred = curr;
		curr = succ;
	}

	loop {
		if curr == ptr::null_mut() {
			break;
		}

		// Obtain successor node
		let marked_ptr = unsafe { &*curr }.next.load(Ordering::SeqCst);
		succ = marked_ptr.0;
		marked = marked_ptr.1;

		// Check for ordered location in the list
		let ctag = unsafe { &*curr }.tag-100;
		//println!("{}", ctag);
		highest_temperatures.pop_front();
		highest_temperatures.push_back(ctag);

		// Continue through the list
		pred = curr;
		curr = succ;
	}

	println!("Lowest 5 Temperatures: {:?}\nHighest 5 Temperatures: {:?}", lowest_temperatures, highest_temperatures);
}

// Given list head and tag, find the pred and curr nodes that have the nearest tag
fn find<'a>(head: &AtomicMarkablePtr<Node>, tag: i32) -> (*mut Node, *mut Node) {
	let mut pred: *mut Node; let mut curr: *mut Node; let mut succ: *mut Node;
	let mut marked = false; let snip: bool;
	
	// If fails, can use to retry
	'retry: loop {
		// Handle null head reference passed
		let markable_ptr = head.load(Ordering::SeqCst);
		pred = markable_ptr.0;
		if pred == ptr::null_mut() {
			return (pred, pred);
		}
		// Handle empty list passed
		let markable_ptr = unsafe { &*pred }.next.load(Ordering::SeqCst);
		curr = markable_ptr.0;
		
		loop {
			if curr == ptr::null_mut() {
				return (pred, curr)
			}

			// Obtain successor node
			let marked_ptr = unsafe { &*curr }.next.load(Ordering::SeqCst);
			succ = marked_ptr.0;
			marked = marked_ptr.1;
			
			// If node marked to be deleted, delete.
			while marked {
				// Delete node here
				let temp = unsafe { &*curr }.tag;
				if unsafe { &*pred }.next.compare_and_swap(curr, false, succ, false, Ordering::SeqCst) != (curr, false) {
					continue 'retry;
				} else {
					//println!("Present {} deleted!", temp);
				}
				drop(temp);

				curr = succ;
				// End of list, no more to be seen, return
				if curr == ptr::null_mut() {
					return (pred, curr)
				}
				
				// Continue to next node to check if marked
				let marked_ptr = unsafe { &*curr }.next.load(Ordering::SeqCst);
				succ = marked_ptr.0;
				marked = marked_ptr.1;
			}

			// Check for ordered location in the list
			let ctag = unsafe { &*curr }.tag;
			//println!("{}", ctag);
			if ctag >= tag {
				return (pred, curr);
			}

			// Continue through the list
			pred = curr;
			curr = succ;
		}
	};
}

// Adds a node to the list
fn add(head: &AtomicMarkablePtr<Node>, tag: i32) -> bool {
	// Retry loop
	loop {
		// Find edge nodes for adding the new node
		let (pred, curr) = find(head, tag);

		// If invalid head/value, return
		if pred == ptr::null_mut() {
			return false;
		// Add node
		} else {
			let new_node = Arc::into_raw(Arc::new(Node {
				tag,
				next: AtomicMarkablePtr::new(curr, false)
			})) as *mut _;
			
			// Attempt to insert node into list, if fail, retry
			if unsafe { &*pred }.next.compare_and_swap(curr,  false, new_node, false, Ordering::SeqCst) == (curr, false) {
				return true;
			}
		}
	}
}

// Removes/marks-for-removal node from list
fn remove(head: &AtomicMarkablePtr<Node>, tag: i32) -> bool {
	// Retry loop
	loop {
		// Find node for removal, if exists
		let (pred, curr) = find(head, tag);

		// Invalid values
		if pred == ptr::null_mut() {
			return false;
		// Invalid values or the node looking for does not exist
		} else if curr == ptr::null_mut() || unsafe { &*curr }.tag != tag {
			return false;
		// Remove/mark-to-remove node
		} else {
			let marked_ptr = unsafe { &*curr }.next.load(Ordering::SeqCst);
			let succ = marked_ptr.0;
			
			// Mark the node for removal
			if unsafe { &*curr }.next.compare_and_swap(succ,  false, succ, true, Ordering::SeqCst) != (succ, false) {
				continue;
			}
			
			// Attempt to remove the node. If fails, no worries. Taken care of in "find"
			if unsafe { &*pred }.next.compare_and_swap(curr,  false, succ, false, Ordering::SeqCst) == (curr, false) {
				//println!("Present {} deleted", tag);
			}
			return true;
		}
	}
}