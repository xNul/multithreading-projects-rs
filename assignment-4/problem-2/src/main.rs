/* Blake Wyatt
   COP 6616
   Assignment 4: Problem 2 */

use std::{
	time::Instant,
	thread,
	sync::Arc
};
use rand::prelude::*;

mod lockfree_elimination_stack;
use lockfree_elimination_stack::Stack;

fn main() {
	// Use these to pipe benchmarks to CSV
	//pop_ratios();
	//push_ratios();

	// Run problem 2
	lock_free_stack_elimination_problem(8, 0.5);
}

fn lock_free_stack_elimination_problem(t: i32, r: f64) -> f32 {
	let mut handles = vec![];

	// Decided to not pre-populate the list as the pop function accounts for missing nodes
	let stack = Arc::new(Stack::new());
	
	// Start timer
	println!("Starting Problem 2");
	let now = Instant::now();

	// Spawn 8 threads
	for _ in 0..t {
		let stack = stack.clone();

		// Thread code
		let handle = thread::spawn(move || {
			let mut rng = rand::thread_rng();

			// Perform an operation 150,000 times
			for _ in 0..150000 {
				// Select either push or pop with a percent chance
				let function_id = rng.gen_bool(r);
				
				// Randomly select an operation
				if function_id {
					// x = 0 in all cases
					stack.push(0);
				} else {
					stack.pop();
				}
			}
		});

		handles.push(handle);
	}

	// Main thread awaits the completion of all other threads.
    for handle in handles {
        handle.join().unwrap();
	}

	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	println!("Program Success -- Problem 2 Completed");
	println!("Total execution time: {}", execution_time);

	execution_time
}

fn push_ratios() {
	for r in vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0] {
		for t in 1..9 {
			let mut sum = 0.0;
			for _ in 0..5 {
				sum += lock_free_stack_elimination_problem(t, r);
			}
			println!("{},{},{}", r, t, sum/5.0);
		}
	}
}

fn pop_ratios() {
	for r in vec![1.0, 0.8, 0.6, 0.4, 0.2, 0.0] {
		for t in 1..9 {
			let mut sum = 0.0;
			for _ in 0..5 {
				sum += lock_free_stack_elimination_problem(t, r);
			}
			println!("{},{},{}", 1.0-r, t, sum/5.0);
		}
	}
}