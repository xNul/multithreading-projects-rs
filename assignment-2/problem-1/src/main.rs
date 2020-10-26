/* Blake Wyatt
   COP 6616
   Assignment 2: Problem 1 */

use std::{
	thread,
	sync::Arc,
	sync::atomic::{Ordering, AtomicBool},
	env
};
use rand::prelude::*;


fn main() {
	let args: Vec<String> = env::args().collect();
	let n = args.get(1).unwrap().parse::<usize>().unwrap();

	// Run problem 1
	minotaurs_birthday_party(n);
}

fn minotaurs_birthday_party(n: usize) {
	// Randomly arrange the starting order of guests
	let mut rng = rand::thread_rng();
	let mut guests: Vec<usize> = (0..n).collect();
	guests.shuffle(&mut rng);

	// Thread and async data initialization
	let mut handles = vec![];
	let cupcake = Arc::new(AtomicBool::new(true));
	let exit = Arc::new(AtomicBool::new(false));

	println!("Starting the Minotaur's Birthday Party game");

	// Start all threads
	for guest in guests.iter() {
		let cupcake = cupcake.clone();
		let exit = exit.clone();

		if *guest == 0 {
			// Thread for designated counter guest 0
			let handle = thread::spawn(move || {
				let mut counter = 0;

				// Send guest 0 into the labyrinth
				loop {
					// Atomically check if the cupcake has been eaten
					let cupcake = cupcake.fetch_or(true, Ordering::SeqCst);
					if !cupcake {
						counter += 1;
						println!("{}/{} guests completed", counter, n-1);
					}
					// Atomically set exit flag if all guests have made it through
					if counter == n-1 {
						exit.fetch_or(true, Ordering::SeqCst);
						break;
					}
				}
			});
			handles.push(handle);
		} else {
			// Thread for all other guests
			let handle = thread::spawn(move || {
				let mut status = true;

				// Send all other guests into the labyrinth
				loop {
					// Atomically eat your cake if it exists and haven't done so already
					if status {
						status = !cupcake.fetch_and(false, Ordering::SeqCst);
					} else {
						// Simulate cupcake viewing from going through the labyrinth once
						cupcake.load(Ordering::Acquire);
					}

					// If all guests have made it through, exit
					if exit.load(Ordering::Acquire) {
						break;
					}
				}
			});
			handles.push(handle);
		}
	}

	// Main thread awaits the completion of all other threads.
    for handle in handles {
        handle.join().unwrap();
	}

	println!("Program Success -- All guests have made it through the maze")
}