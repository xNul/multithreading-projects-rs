use rand::prelude::*;

mod exchanger;
use exchanger::Exchanger;

// An atomic elimination array implementation.
pub struct EliminationArray<T> {
	duration: i32, // In nanoseconds
	exchanger: Vec<Exchanger<T>>
}

impl<T> EliminationArray<T> {
	// Create a new elimination array.
	pub fn new(capacity: i32) -> Self {
		let mut exchanger: Vec<Exchanger<T>> = Vec::new();

		// Create the exchanger array.
		for _ in 0..capacity {
			exchanger.push(Exchanger::new());
		}

		Self {
			duration: 100,
			exchanger
		}
	}

	// Attempt to do an exchange with a random exchanger.
	pub fn visit(&self, value: *mut T, range: i32) -> Option<*mut T> {
		let mut rng = rand::thread_rng();
		let slot = rng.gen_range(0, range) as usize;
		let nanodur = self.duration as u128;

		self.exchanger[slot].exchange(value, nanodur)
	}
}