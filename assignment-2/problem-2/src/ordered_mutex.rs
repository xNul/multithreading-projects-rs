use std::collections::VecDeque;
use std::sync::{Mutex, LockResult, MutexGuard, PoisonError};
use std::ops::{Deref, DerefMut};
use std::thread;
use std::thread::ThreadId;

pub struct OrderedMutex<T> {
	queue: Mutex<VecDeque<ThreadId>>,
	value: Mutex<T>
}

pub struct OrderedMutexGuard<'a, T> {
	pub mg: MutexGuard<'a, T>,
	queue: &'a Mutex<VecDeque<ThreadId>>
}

impl<T> OrderedMutex<T> {
	pub fn new(value: T) -> OrderedMutex<T> {
		let queue = VecDeque::new();

		OrderedMutex {
			queue: Mutex::new(queue),
			value: Mutex::new(value)
		}
	}

	pub fn lock(&self) -> LockResult<OrderedMutexGuard<T>> {
		let thread_id = thread::current().id();
		let mut queue = self.queue.lock().unwrap();
		queue.push_back(thread_id);

		if queue.len() == 1 {
			drop(queue)
		} else {
			drop(queue);
			while *self.queue.lock().unwrap().front().unwrap() != thread_id {}
		}

		match self.value.lock() {
			Ok(value) => Ok(OrderedMutexGuard { mg: value, queue: &self.queue }),
			Err(error) => Err(PoisonError::new(OrderedMutexGuard { mg: error.into_inner(), queue: &self.queue }))
		}
	}
}

impl<T> Deref for OrderedMutex<T> {
	type Target = Mutex<T>;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T> DerefMut for OrderedMutex<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
	}
}

impl<'a, T> Drop for OrderedMutexGuard<'a, T> {
	fn drop(&mut self) {
		self.queue.lock().unwrap().pop_front().unwrap();
	}
}