# Documentation and Writing for Assignment 1

### Brief Approach Summary

Performance is the top priority of this assignment. In order to achieve the best performance and parallelism, I intentionally chose a low-level language, known for its concurrency called Rust. I started with basic prime number searching implementations and worked my way up to the Sieve of Erastothenes, one of the most efficient prime number algorithms known. My final implementation is a concurrent Sieve of Erastothenes with atomic, lockless, and safe code. The atomicity is instruction-level so performance loss isn't bad and it is lockless because Mutex locks were taking up too much runtime and blocking eachother. Mutexes prevented me from gaining any significant improvements over my serial implementation of Sieve of Erastothenes. Work is distributed via an atomic acounter as needed and so, if the OS is scheduling the threads for equal amounts of time, then the threads are performing at equal levels.

### Informal Statement

Given Rust's complexity and my unfamiliarity to it, I have certainly missed tweaks here and there to optimize my code. I think my overall implementation is very good though. I would adapt it from Sieve of Erastothenes to Sieve of Atkin if I was able because the Sieve of Atkin seems to have marginally better performance. I'm sure there are other algorithms as well that, in an optimal situation, would be implemented and tested against each other.

### Experimental Evaluation Summary

I created a Bash script called `benchmark.sh` to sample runtime performance 100 times and verify primes.txt outputs. Using this script, we can evaluate the runtime performance of my different approaches.

1. `sieve_of_erastothenes_parallel_atomic`: average 0.362 seconds and valid outputs
2. `sieve_of_erastothenes`: average 0.8504 seconds and valid outputs
3. `sieve_of_erastothenes_parallel`: average 4.082 seconds and valid outputs
4. `sieve_of_erastothenes_parallel_deadlock`: deadlocked as expected, invalid outputs, and therefore runtime is invalid as well.

A description of each is commented in my code.
