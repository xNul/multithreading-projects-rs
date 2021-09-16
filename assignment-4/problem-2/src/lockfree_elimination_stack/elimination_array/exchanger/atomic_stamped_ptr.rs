use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem::transmute;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Status {
	EMPTY,
	WAITING,
	BUSY
}

// A basic implementation of Java's AtomicStampedReference. Uses the
// last 2 bits of the pointer value to indicate one of the three Statuses.
// There could be addressing conflicts if you have a lot of RAM or are
// using a 32 bit computer with a lot of RAM. Might be possible to overcome
// by reimplementing with AtomicU64 as the type that holds the data.
#[derive(Debug)]
pub struct AtomicStampedPtr<T> {
    ptr: AtomicPtr<T>,
}

impl<T> AtomicStampedPtr<T> {
    // Create a new AtomicStampedReference with the initial stamping set to stamp.
    pub fn new(mut p: *mut T, stamp: Status) -> Self {
		p = combine_stamp_ptr(p, stamp);

        Self {
            ptr: AtomicPtr::new(p)
        }
    }

    // Get a raw mutable reference to the underlying stamped pointer.
    pub fn get_mut(&mut self) -> &mut *mut T {
        self.ptr.get_mut()
    }

    // Consume the stamped reference and get the underlying pointer and
    // the underlying stamp, this seperates the stamp from the pointer.
    pub fn into_inner(self) -> (*mut T, Status) {
        let p = self.ptr.into_inner();

        separate_stamp_ptr(p)
    }

    // Load the stamped reference and get the underlying pointer and
    // the underlying stamp, this seperates the stamp from the pointer.
    pub fn load(&self, order: Ordering) -> (*mut T, Status) {
        let p = self.ptr.load(order);
		
		separate_stamp_ptr(p)
    }

    // Get the unstamped pointer.
    pub fn ptr(&self, order: Ordering) -> *mut T {
        self.load(order).0
    }

    // Get the stamp.
    pub fn stamp(&self, order: Ordering) -> Status {
        self.load(order).1
    }

    // Store the stamped reference and set the underlying pointer and
    // the underlying stamp, this seperates the stamp from the pointer.
    pub fn store(&self, mut p: *mut T, stamp: Status, order: Ordering) {
        p = combine_stamp_ptr(p, stamp);

        self.ptr.store(p, order);
    }

    // Swap the current stamped ptr with the given unstamped pointer stamped by stamp.
    pub fn swap(&self, mut p: *mut T, stamp: Status, order: Ordering) -> (*mut T, Status) {
        p = combine_stamp_ptr(p, stamp);

        p = self.ptr.swap(p, order);

        separate_stamp_ptr(p)
    }

    // Compare and swap the current stamped ptr with the given unstamped pointer stamped by stamp.
    pub fn compare_and_swap(&self, mut curr_p: *mut T, curr_stamp: Status, mut new_p: *mut T, new_stamp: Status, order: Ordering) -> (*mut T, Status) {
        new_p = combine_stamp_ptr(new_p, new_stamp);
        curr_p = combine_stamp_ptr(curr_p, curr_stamp);

        new_p = self.ptr.compare_and_swap(curr_p, new_p, order);

		separate_stamp_ptr(new_p)
    }
}

fn combine_stamp_ptr<T>(p: *mut T, stamp: Status) -> *mut T {
	let mut ptr: usize = unsafe { transmute(p) };

    // Pointer and stamp to stamped pointer.
	ptr = match stamp {
		Status::EMPTY => ptr & !(0b11),
		Status::WAITING => (ptr & !(0b11)) | 0b01,
		Status::BUSY => (ptr & !(0b11)) | 0b10
	};

	return unsafe { transmute(ptr) };
}

fn separate_stamp_ptr<T>(p: *mut T) -> (*mut T, Status) {
	let stamp_ptr: usize = unsafe { transmute(p) };

	// Stamped pointer to pointer and stamp.
	let stamp = match stamp_ptr & 0b11 {
		0b00 => Status::EMPTY,
		0b01 => Status::WAITING,
		0b10 => Status::BUSY,
		_ => panic!("Invalid Stamp. Address size too large to append Stamp. Try removing RAM or using a 64 bit computer.")
	};
	let ptr = stamp_ptr & !(0b11);

	(unsafe { transmute(ptr) }, stamp)
}