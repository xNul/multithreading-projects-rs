/* Blake Wyatt
   COP 6616
   Assignment 2: Problem 2 */
// I chose to implement a Queueable Mutex Lock

use std::{
	thread,
	sync::{Arc, Mutex},
	sync::atomic::{Ordering, AtomicBool, AtomicUsize},
	env
};
use rand::prelude::*;

mod ordered_mutex;
use ordered_mutex::OrderedMutex;

fn main() {
	let args: Vec<String> = env::args().collect();
	let n = args.get(1).unwrap().parse::<usize>().unwrap();

	// Run problem 2 with a Queueable Mutex Lock
	// this function ends when all guests have entered the
	// room for our performance testing script
	minotaurs_crystal_vase_queueable_mutex_lock(n);
}

fn minotaurs_crystal_vase_queueable_mutex_lock(n: usize) {
	// Randomly arrange the starting order of guests
	let mut rng = rand::thread_rng();
	let mut guests: Vec<usize> = (0..n).collect();
	guests.shuffle(&mut rng);

	// Thread and async data initialization
	let mut handles = vec![];
	let vase = Arc::new(OrderedMutex::new(42));
	let counter = Arc::new(AtomicUsize::new(0));
	let exit = Arc::new(AtomicBool::new(false));

	println!("Starting the Minotaur's Crystal Vase party");

	// Start all threads
	for &guest in guests.iter() {
		let vase = vase.clone();
		let counter = counter.clone();
		let exit = exit.clone();

		// Thread for all guests
		let handle = thread::spawn(move || {
			let mut status = false;
			// Guest attempts to view the vase and enters queue
			loop {
				let vase_value = vase.lock().unwrap();
				println!("guest {}: {}", guest, vase_value.mg);
				drop(vase_value);

				// If the current guest gets the mutex and hasn't
				// been in the room before, a counter is incremented
				if !status {
					let count = counter.fetch_add(1, Ordering::SeqCst);
					//println!("{}", count);
					// If this is the last guest to enter the room, set exit flag
					if count+1 == n {
						exit.fetch_or(true, Ordering::SeqCst);
						break;
					}

					status = true;
				}

				// If all guests have made it through, exit
				if exit.load(Ordering::Acquire) {
					break;
				}
			}
		});
		handles.push(handle);
	}

	// Main thread awaits the completion of all other threads.
    for handle in handles {
        handle.join().unwrap();
	}

	println!("The Minotaur's Crystal Vase party has ended!")
}