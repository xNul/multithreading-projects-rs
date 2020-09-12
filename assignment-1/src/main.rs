/* Blake Wyatt
   COP 6616
   Assignment - 1 */

use std::{
	time::Instant,
    fs::File,
	io::{BufWriter, Write},
	thread,
	sync::{Arc, Mutex, RwLock, Barrier},
	sync::atomic::{Ordering, AtomicBool, AtomicUsize},
	cell::UnsafeCell,
};

fn main() {
	let n = 100000000;

	// Best performance, safe, atomic, parallel, optimized.
	sieve_of_erastothenes_parallel_atomic_optimized(n);

	/*let mut j = 2;
	let mut rnum;
	//while j < 3500 {
		rnum = (j+(j+1)/2+1)*2-1;
		println!("{} : {} : {}", j, rnum, (((rnum*rnum) as f64+1.0)*2.0/6.0).ceil() as usize-1);//, if rnum < (n as f64).sqrt() as usize { (((((rnum*rnum+1)*2) as f64)/6.0).ceil() as usize)-1 } else { n });//((((rnum*rnum) as f64+1.0)*2.0/6.0).ceil() as usize)-1);
		let mut i = rnum;
		while i < rnum+100 {
			if i%2 != 0 && i%3 != 0 {
				println!("{}", i);
			}
			i += 1;
		}
		j += 1;
	//}*/
	// (((((rnum*rnum+1)*2) as f64)/6.0).ceil() as usize)-1

	// Safe, atomic, and parallel.
	//sieve_of_erastothenes_parallel_atomic(n);

	// Atomic, parallel, but different prime number search algorithm
	//sieve_of_atkin_parallel_atomic(n);

	// Parallel, atomic, safe, with final processing parallel
	//sieve_of_erastothenes_parallel_atomic_pprocessing(n);

	// Serial, worst performance.
	//sieve_of_erastothenes(n) // Serial

	// Parallel but made irrelevant by Mutex delay.
	//sieve_of_erastothenes_parallel(n)

	// Parallel and attempted to improve performance,
	// while retaining safety, but resulted in deadlock.
	//sieve_of_erastothenes_parallel_deadlock(n)
}

fn sieve_of_erastothenes_parallel_atomic_optimized(n: usize) {
	println!("Prime search initiated.");
	print!("Initializing...");

	// Variables are promptly initialized. These atomic
	// variables utilize atomic processor instructions to
	// perform memory operations safely and efficiently.
	// Arc enables the transfer of variables to other threads, safely.
	let counter = Arc::new(AtomicUsize::new(1));
	let primes: Arc<[AtomicBool]> = (0..n_to_three(n)).map(|_| AtomicBool::new(true)).collect::<Vec<_>>().into();
	let mut handles = vec![];

	print!("Done\nSearching...");

	// All eight threads start and the program begins. The
	// search for prime numbers is a parallel, atomic, and
	// lock-free implementation of the Sieve of Erastothenes.
	// A shared counter is used to distribute work items, as needed.
	// Some primes are accounted for before searching to drastically
	// decrease the runtime. The timer starts before threads, as
	// specified in the assignment requirements.
	let now = Instant::now();
	for _ in 0..8 {
		let counter = counter.clone();
		let primes = primes.clone();
		let handle = thread::spawn(move || {
			let mut i = counter.fetch_add(1, Ordering::SeqCst);
			let mut rnum = three_to_i(i);

			while rnum <= (n as f64).sqrt() as usize {
				if primes[i].load(Ordering::Acquire) {
					// In order to skip across the array and
					// find composites, math has to performed.
					// This is a result of accounting for
					// small primes to reduce runtime.
					let base = i*2+1;
					let adder = ((i-1)/2+1)*4;
					let mut j = i_to_three(rnum*rnum);
					let mut alt = i_to_three(rnum)%2 == 0;
					while j < n_to_three(n) {
						primes[j].store(false, Ordering::Release);
						j += base;
						if alt {
							j += adder;
							alt = false;
						}
						else {
							alt = true;
						}
					}
				}

				i = counter.fetch_add(1, Ordering::SeqCst);
				rnum = three_to_i(i);
			}
		});
		handles.push(handle);
	}

	// Main thread awaits the completion of 8 other threads.
    for handle in handles {
        handle.join().unwrap();
	}

	print!("Done\nProcessing results...");

	// We now know threads are finished and our final, sieve
	// array is ready for indexing. We take care to reverse
	// the order of the loop to consume the largest primes
	// first and add them to our max_primes array. This
	// simple action reduces a large number of memory accesses.
	// This part of the code is not concurrent because serial
	// processing is faster on my hardware.
	let mut count = 2;
	let mut sum = 2+3;
	let mut max_primes: [usize; 10] = [3, 2, 0, 0, 0, 0, 0, 0, 0, 0];
	let mut max_primes_counter = 0;
	for i in (1..n_to_three(n)).rev() {
		if primes[i].load(Ordering::Acquire) {
			count += 1;
			let val = three_to_i(i);
			sum += val;
			if max_primes_counter < 10 {
				max_primes[max_primes_counter] = val;
				max_primes_counter += 1;
			}
		}
	}

	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	// primes.txt file writing and error handling.
	let write_file = match File::create("primes.txt") {
		Err(why) => panic!("Can't create primes.txt: {}", why),
		Ok(file) => file
	};

	let mut writer = BufWriter::new(&write_file);
	match write!(&mut writer, "{} {} {}\n", execution_time, count, sum) {
		Err(why) => panic!("Can't write to primes.txt: {}", why),
		Ok(file) => file
	};
	for i in max_primes.iter().rev() {
		match write!(&mut writer, "{} ", i) {
			Err(why) => panic!("Can't write to primes.txt: {}", why),
			Ok(file) => file
		};
	}

	print!("Done\nSearch complete. File written.\n");
}

// Standard array index to array index without twos.
fn i_to_two(i: usize) -> usize {
	(i+1)/2-1
}

// Standard array index to array index without twos and threes.
/*fn i_to_three(i: usize) -> usize {
	let m = ((i+1)*2);
	m/6 + (m%6 != 0) as usize-1
}*/

// Standard array index to array index without twos and threes.
fn i_to_three(i: usize) -> usize {
	i/3 // What wizardry is this??
}

// Standard array index to array index without twos and threes.
// Goes i to two to three.
fn i_to_three_u(i: usize) -> usize {
	let m = ((i_to_two(i)+1)*2);
	m/3 + (m%3 != 0) as usize-1
}

// Array index without twos to standard array index.
fn two_to_i(t2: usize) -> usize {
	(t2+1)*2-1
}

// Array index without twos and threes to standard array index.
fn three_to_i(t3: usize) -> usize {
	two_to_i(t3+(t3+1)/2)
}

// Find array size for n without twos and threes.
fn n_to_three(n: usize) -> usize {
	n_to_two(n)-n/2/3
}

// Find array size for n without twos.
fn n_to_two(n: usize) -> usize {
	n-n/2
}

// ----------------------- Experiments ----------------------- \\
// The code below was work that led to the final product. It remains
// to show my efforts.

fn sieve_of_erastothenes_parallel_atomic(n: usize) {
	println!("Prime search initiated.");
	print!("Initializing...");

	// Variables are promptly initialized. These atomic
	// variables utilize atomic processor instructions to
	// perform memory operations safely and efficiently.
	// Arc enables the transfer of variables to other threads, safely.
	let counter = Arc::new(AtomicUsize::new(2));
	let primes: Arc<[AtomicBool]> = (0..n).map(|_| AtomicBool::new(true)).collect::<Vec<_>>().into();
	let mut handles = vec![];

	print!("Done\nSearching...");

	// All eight threads start and the program begins. The
	// search for prime numbers is a parallel, atomic, and
	// lock-free implementation of the Sieve of Erastothenes.
	// A shared counter is used to distribute work items, as needed.
	// The timer starts before threads, as specified in assignment.
	let now = Instant::now();
	for _ in 0..8 {
		let counter = counter.clone();
		let primes = primes.clone();
		let handle = thread::spawn(move || {
			let mut i = counter.fetch_add(1, Ordering::SeqCst);

			while i*i <= n {
				if primes[i].load(Ordering::Acquire) {
					for prime in primes.iter().skip(i*i).step_by(i) {
						prime.store(false, Ordering::Release);
					}
				}

				i = counter.fetch_add(1, Ordering::SeqCst);
			}
		});
		handles.push(handle);
	}

	// Main thread awaits the completion of 8 other threads.
    for handle in handles {
        handle.join().unwrap();
	}

	print!("Done\nProcessing results...");

	// We now know threads are finished and our final, sieve
	// array is ready for indexing. We take care to reverse
	// the order of the loop to consume the largest primes
	// first and add them to our max_primes array. This
	// simple action reduces a large number of memory accesses.
	// This part of the code is not concurrent because serial
	// processing is faster on my hardware.
	let mut count = 0;
	let mut sum = 0;
	let mut max_primes: [usize; 10] = [0; 10];
	let mut max_primes_counter = 0;
	for i in (2..n).rev() {
		if primes[i].load(Ordering::Acquire) {
			count += 1;
			sum += i;
			if max_primes_counter < 10 {
				max_primes[max_primes_counter] = i;
				max_primes_counter += 1;
			}
		}
	}

	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	// primes.txt file writing and error handling.
	let write_file = match File::create("primes.txt") {
		Err(why) => panic!("Can't create primes.txt: {}", why),
		Ok(file) => file
	};

	let mut writer = BufWriter::new(&write_file);
	match write!(&mut writer, "{} {} {}\n", execution_time, count, sum) {
		Err(why) => panic!("Can't write to primes.txt: {}", why),
		Ok(file) => file
	};
	for i in max_primes.iter().rev() {
		match write!(&mut writer, "{} ", i) {
			Err(why) => panic!("Can't write to primes.txt: {}", why),
			Ok(file) => file
		};
	}

	print!("Done\nSearch complete. File written.\n");
}

struct BoolCell(UnsafeCell<bool>);

impl BoolCell {
	fn get(&self) -> *mut bool {
		self.0.get()
	}

	fn into_inner(self) -> bool {
		self.0.into_inner()
	}
}

impl std::clone::Clone for BoolCell {
	fn clone(&self) -> BoolCell {
		BoolCell(UnsafeCell::new(true))
	}
}

unsafe impl Sync for BoolCell {}
unsafe impl Send for BoolCell {}

fn sieve_of_erastothenes(n: usize) {
	let mut primes = vec![true; n];

	println!("Started prime search");

	let now = Instant::now();
	let mut i = 2;
	while i*i <= n {
		if primes[i] {
			for j in (i*i..n).step_by(i) {
			  primes[j] = false;
			}
		}
		i += 1;
	}

	let mut count = 0;
	let mut sum = 0;
	let mut max_primes: [usize; 10] = [0; 10];
	let mut max_primes_counter = 0;
	for i in (2..n).rev() {
		if primes[i] {
			count += 1;
			sum += i;
			if max_primes_counter < 10 {
				max_primes[max_primes_counter] = i;
				max_primes_counter += 1;
			}
		}
	}

	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	// primes.txt file writing and error handling.
	let write_file = match File::create("primes.txt") {
		Err(why) => panic!("Can't create primes.txt: {}", why),
		Ok(file) => file
	};

	let mut writer = BufWriter::new(&write_file);
	match write!(&mut writer, "{} {} {}\n", execution_time, count, sum) {
		Err(why) => panic!("Can't write to primes.txt: {}", why),
		Ok(file) => file
	};
	for i in max_primes.iter().rev() {
		match write!(&mut writer, "{} ", i) {
			Err(why) => panic!("Can't write to primes.txt: {}", why),
			Ok(file) => file
		};
	}
}

fn sieve_of_erastothenes_parallel(n: usize) {
	let counter = Arc::new(Mutex::new(2));
	let primes_mutex = Arc::new(Mutex::new(vec![true; n]));
    let mut handles = vec![];

	println!("Started prime search");

	let now = Instant::now();
    for _ in 0..8 {
		let counter = counter.clone();
		let primes_mutex = primes_mutex.clone();
        let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			let mut i = *num;
			*num += 1;
			drop(num);

			while i*i <= n {
				let mut primes = primes_mutex.lock().unwrap();
				if primes[i] {
					for j in (i*i..n).step_by(i) {
					  primes[j] = false;
					}
				}
				drop(primes);

				num = counter.lock().unwrap();
				i = *num;
				*num += 1;
				drop(num);
			}
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
	}

	let mut count = 0;
	let mut sum = 0;
	let mut max_primes: [usize; 10] = [0; 10];
	let mut max_primes_counter = 0;
	for i in (2..n).rev() {
		let primes = primes_mutex.lock().unwrap();
		if primes[i] {
			count += 1;
			sum += i;
			if max_primes_counter < 10 {
				max_primes[max_primes_counter] = i;
				max_primes_counter += 1;
			}
		}
	}
	
	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	// primes.txt file writing and error handling.
	let write_file = match File::create("primes.txt") {
		Err(why) => panic!("Can't create primes.txt: {}", why),
		Ok(file) => file
	};

	let mut writer = BufWriter::new(&write_file);
	match write!(&mut writer, "{} {} {}\n", execution_time, count, sum) {
		Err(why) => panic!("Can't write to primes.txt: {}", why),
		Ok(file) => file
	};
	for i in max_primes.iter().rev() {
		match write!(&mut writer, "{} ", i) {
			Err(why) => panic!("Can't write to primes.txt: {}", why),
			Ok(file) => file
		};
	}
}

fn sieve_of_erastothenes_parallel_deadlock(n: usize) {
	let counter = Arc::new(RwLock::new(2));
	let primes_mutex = Arc::new(RwLock::new(vec![true; n]));
    let mut handles = vec![];

	println!("Started prime search");

	let now = Instant::now();
    for _ in 0..8 {
		let counter = counter.clone();
		let primes_mutex = primes_mutex.clone();
        let handle = thread::spawn(move || {
			let mut num = counter.write().unwrap();
			let mut i = *num;
			*num += 1;
			drop(num);

			while i*i <= n {
				let mut primes = primes_mutex.read().unwrap();
				if primes[i] {
					for j in (i*i..n).step_by(i) {
						if primes[j] {
							drop(primes);
							let mut primesw = primes_mutex.write().unwrap();
							primesw[j] = false;
							primes = primes_mutex.read().unwrap();
						}
					}
				}
				drop(primes);

				num = counter.write().unwrap();
				i = *num;
				*num += 1;
				drop(num);
			}
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
	}

	let mut count = 0;
	let mut sum = 0;
	let mut max_primes: [usize; 10] = [0; 10];
	let mut max_primes_counter = 0;
	for i in (2..n).rev() {
		let primes = primes_mutex.read().unwrap();
		if primes[i] {
			count += 1;
			sum += i;
			if max_primes_counter < 10 {
				max_primes[max_primes_counter] = i;
				max_primes_counter += 1;
			}
		}
	}

	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	// primes.txt file writing and error handling.
	let write_file = match File::create("primes.txt") {
		Err(why) => panic!("Can't create primes.txt: {}", why),
		Ok(file) => file
	};

	let mut writer = BufWriter::new(&write_file);
	match write!(&mut writer, "{} {} {}\n", execution_time, count, sum) {
		Err(why) => panic!("Can't write to primes.txt: {}", why),
		Ok(file) => file
	};
	for i in max_primes.iter().rev() {
		match write!(&mut writer, "{} ", i) {
			Err(why) => panic!("Can't write to primes.txt: {}", why),
			Ok(file) => file
		};
	}
}

fn sieve_of_erastothenes_parallel_atomic_pprocessing(n: usize) {
	println!("Prime search initiated.");
	print!("Initializing...");

	// Variables are promptly initialized. These atomic
	// variables utilize atomic processor instructions to
	// perform memory operations safely and efficiently.
	// Arc enables the transfer of variables to other threads, safely.
	let counter = Arc::new(AtomicUsize::new(2));
	let count = Arc::new(AtomicUsize::new(0));
	let sum = Arc::new(AtomicUsize::new(0));
	let max_primes: Arc<[AtomicUsize]> = (0..10).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>().into();
	let primes: Arc<[AtomicBool]> = (0..n).map(|_| AtomicBool::new(true)).collect::<Vec<_>>().into();
	let barrier = Arc::new(Barrier::new(8));
	let mut handles = vec![];

	print!("Done\nSearching...");

	// All eight threads start and the program begins. The
	// search for prime numbers is a parallel, atomic, and
	// lock-free implementation of the Sieve of Erastothenes.
	// A shared counter is used to distribute work items, as needed.
	// The timer starts before threads, as specified in assignment.
	let now = Instant::now();
	for t in 0..8 {
		let counter = counter.clone();
		let count = count.clone();
		let sum = sum.clone();
		let max_primes = max_primes.clone();
		let primes = primes.clone();
		let barrier = barrier.clone();
		let handle = thread::spawn(move || {
			let mut i = counter.fetch_add(1, Ordering::SeqCst);

			while i*i <= n {
				if primes[i].load(Ordering::Acquire) {
					for j in (i*i..n).step_by(i) {
						primes[j].store(false, Ordering::Release);
					}
				}

				i = counter.fetch_add(1, Ordering::SeqCst);
			}

			barrier.wait();

			let mut max_primes_counter = 0;
			for i in (t*n/8..(t+1)*n/8).rev() {
				if i != 0 && i != 1 && primes[i].load(Ordering::Acquire) {
					count.fetch_add(1, Ordering::SeqCst);
					sum.fetch_add(i, Ordering::SeqCst);
					if t == 7 && max_primes_counter < 10 {
						max_primes[max_primes_counter].store(i, Ordering::Release);
						max_primes_counter += 1;
					}
				}
			}
		});
		handles.push(handle);
	}

	// Main thread awaits the completion of 8 other threads.
    for handle in handles {
        handle.join().unwrap();
	}

	print!("Done\nProcessing results...");

	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	// primes.txt file writing and error handling.
	let write_file = match File::create("primes.txt") {
		Err(why) => panic!("Can't create primes.txt: {}", why),
		Ok(file) => file
	};

	let mut writer = BufWriter::new(&write_file);
	match write!(&mut writer, "{} {} {}\n", execution_time, count.load(Ordering::Acquire), sum.load(Ordering::Acquire)) {
		Err(why) => panic!("Can't write to primes.txt: {}", why),
		Ok(file) => file
	};
	for i in max_primes.iter().rev() {
		match write!(&mut writer, "{} ", i.load(Ordering::Acquire)) {
			Err(why) => panic!("Can't write to primes.txt: {}", why),
			Ok(file) => file
		};
	}

	print!("Done\nSearch complete. File written.\n");
}

fn sieve_of_atkin_parallel_atomic(n: usize) {
	println!("Prime search initiated.");
	print!("Initializing...");

	// Variables are promptly initialized. These atomic
	// variables utilize atomic processor instructions to
	// perform memory operations safely and efficiently.
	// Arc enables the transfer of variables to other threads, safely.
	let counter = Arc::new(AtomicUsize::new(1));
	let primes: Arc<[AtomicBool]> = (0..n).map(|_| AtomicBool::new(false)).collect::<Vec<_>>().into();
	let mut handles = vec![];

	primes[2].store(true, Ordering::Release);
	primes[3].store(true, Ordering::Release);

	print!("Done\nSearching...");

	// All eight threads start and the program begins. The
	// search for prime numbers is a parallel, atomic, and
	// lock-free implementation of the Sieve of Erastothenes.
	// A shared counter is used to distribute work items, as needed.
	// The timer starts before threads, as specified in assignment.
	let now = Instant::now();
	for _ in 0..8 {
		let counter = counter.clone();
		let primes = primes.clone();
		let handle = thread::spawn(move || {
			let mut x = counter.fetch_add(1, Ordering::SeqCst);

			while x * x < n {
				let mut y = 1;
				while y * y < n {
					let mut z = (4 * x * x) + (y * y);
					if z <= n && (z % 12 == 1 || z % 12 == 5) {
						primes[z].fetch_xor(true, Ordering::SeqCst);
					}
					z = (3 * x * x) + (y * y);
					if z <= n && z % 12 == 7 {
						primes[z].fetch_xor(true, Ordering::SeqCst);
					}
					z = (3 * x * x) - (y * y);
					if x > y && z <= n && z % 12 == 11 {
						primes[z].fetch_xor(true, Ordering::SeqCst);
					}
					y += 1;
				}

				x = counter.fetch_add(1, Ordering::SeqCst);
			}
		});
		handles.push(handle);
	}

	// Main thread awaits the completion of 8 other threads.
    for handle in handles {
        handle.join().unwrap();
	}

	let mut r = 5;
	while r * r < n {
		if primes[r].load(Ordering::Acquire) {
			for prime in primes.iter().skip(r*r).step_by(r*r) {
				prime.store(false, Ordering::Release);
			}
		}

		r += 1;
	}

	print!("Done\nProcessing results...");

	// We now know threads are finished and our final, sieve
	// array is ready for indexing. We take care to reverse
	// the order of the loop to consume the largest primes
	// first and add them to our max_primes array. This
	// simple action reduces a large number of memory accesses.
	// This part of the code is not concurrent because serial
	// processing is faster on my hardware.
	let mut count = 0;
	let mut sum = 0;
	let mut max_primes: [usize; 10] = [0; 10];
	let mut max_primes_counter = 0;
	for i in (2..n).rev() {
		if primes[i].load(Ordering::Acquire) {
			count += 1;
			sum += i;
			if max_primes_counter < 10 {
				max_primes[max_primes_counter] = i;
				max_primes_counter += 1;
			}
		}
	}

	// Execution time in seconds
	let execution_time = now.elapsed().as_secs_f32();

	// primes.txt file writing and error handling.
	let write_file = match File::create("primes.txt") {
		Err(why) => panic!("Can't create primes.txt: {}", why),
		Ok(file) => file
	};

	let mut writer = BufWriter::new(&write_file);
	match write!(&mut writer, "{} {} {}\n", execution_time, count, sum) {
		Err(why) => panic!("Can't write to primes.txt: {}", why),
		Ok(file) => file
	};
	for i in max_primes.iter().rev() {
		match write!(&mut writer, "{} ", i) {
			Err(why) => panic!("Can't write to primes.txt: {}", why),
			Ok(file) => file
		};
	}

	print!("Done\nSearch complete. File written.\n");
}