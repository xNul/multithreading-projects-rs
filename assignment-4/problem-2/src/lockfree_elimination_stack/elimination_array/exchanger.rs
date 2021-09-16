use std::{
	time::SystemTime,
	ptr,
	sync::atomic::Ordering
};

mod atomic_stamped_ptr;
use atomic_stamped_ptr::AtomicStampedPtr;
use atomic_stamped_ptr::Status;

// An atomic elimination array exchanger implementation.
pub struct Exchanger<T> {
	slot: AtomicStampedPtr<T>
}

impl<T> Exchanger<T> {
	// Create a new exchanger.
	pub fn new() -> Self {
		Self {
			slot: AtomicStampedPtr::new(ptr::null_mut(), Status::EMPTY)
		}
	}

	// Exchange one item for another with a timeout.
	pub fn exchange(&self, my_item: *mut T, nanos: u128) -> Option<*mut T> {
		let current_time = SystemTime::now();

		// Attempt to exchange unless timeout.
		while current_time.elapsed().unwrap().as_nanos() < nanos {
			let (her_item, stamp) = self.slot.load(Ordering::SeqCst);

			// Perform the relevant step of the exchange
			match stamp {
				// Insert a value to start the exchange. Attempt to complete the exchange afterwards.
				Status::EMPTY => {
					if self.slot.compare_and_swap(her_item, Status::EMPTY, my_item, Status::WAITING, Ordering::SeqCst) == (her_item, Status::EMPTY) {
						while current_time.elapsed().unwrap().as_nanos() < nanos {
							let (her_item, stamp) = self.slot.load(Ordering::SeqCst);

							if stamp == Status::BUSY {
								self.slot.store(ptr::null_mut(), Status::EMPTY, Ordering::SeqCst);
								return Some(her_item);
							}
						}

						// Timeout has been reached, let's give one more attempt and then finish.
						if self.slot.compare_and_swap(my_item, Status::WAITING, ptr::null_mut(), Status::EMPTY, Ordering::SeqCst) == (her_item, Status::EMPTY) {
							return None;
						} else {
							let (her_item, _) = self.slot.load(Ordering::SeqCst);
							self.slot.store(ptr::null_mut(), Status::EMPTY, Ordering::SeqCst);

							return Some(her_item);
						}
					}
				},

				// A value has already been inserted and so we attempt to exchange.
				Status::WAITING => {
					if self.slot.compare_and_swap(her_item, Status::WAITING, my_item, Status::BUSY, Ordering::SeqCst) == (her_item, Status::WAITING) {
						return Some(her_item);
					}
				},

				// An exchange is already being taken place. Do nothing.
				Status::BUSY => {}
			}
		}

		None
	}
}