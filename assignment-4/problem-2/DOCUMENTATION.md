# Report for Assignment 4: Problem 2

The deleted node list can be obtained by uncommenting line 95 in `lockfree_elimination_stack.rs`, compiling, and running the program. It is optional because of the latency induced to record all of the deleted nodes.

Since it wasn't specified, the address of each node is what is recorded for each node.

## Implementation Summary

My implementation of a lock-free, concurrent elimination stack uses the standard Rust type `AtomicPtr`. `AtomicPtr` allows the user to atomically store and modify a pointer value within.

Each stack node contains two fields: a generic value field and an `AtomicPtr` next field. Every time I need to push or pop a node, I atomically update the head `AtomicPtr` and if I'm pushing to the stack, I set the next `AtomicPtr` field of the new node to the node head was pointing to.

The `push` and `pop` functions each have two parts. The first is to retry the operation until it is complete and the second is to actually perform the operation and report back to the first whether or not it was successful. I implemented exponential backoff to try and improve performance, but for whatever reason, and no matter what timeout value I entered, it only seemed to hurt my performance.

However, elimination backoff has worked very well. To implement it, I had to create everything from the ground up. The elimination array is an object with a `Vec` of exchanger objects. I tried different duration values and the best duration for my computer seems to be 100 nanoseconds.

My exchanger struct is built as you would expect, atomically loading, checking the stamp's status, and swapping it out as needed to complete exchanges, but it took a lot of time because I had to make my own `AtomicStampedReference` Rust equivalent. I call it `AtomicStampedPtr` as 'ptr' is the Rust convention. I do some advanced Rust memory transmutation to take advantage of the last 2 bits of the `AtomicPtr` datatype. Similar to `AtomicMarkableReference`, the last two bits hold the stamp values for `AtomicStampedPtr`. Unfortunately there is an edge case on some systems where if you have too much RAM, the address uses up too much of the value to use both bits at the end, though I imagine it is hard to run into. Otherwise, it works perfectly.

## Linearization Points

My code linearizes in the functions `pop`, `push`, and `exchange`. Every atomic operation that is performed, forces the computer to linearize.

## Experimental Results

With the experimental data I generated from benchmarking execution times (as mentioned in the README), I created four different graphs. There is a pop graph and a push graph for each problem.

It might look as though runtime is actually increasing with each thread added, but with each thread, comes 150,000 more operations to execute. In reality, the program is acting as intended and with more threads, the runtime decreases.

Each graph has average execution time (5 samples averaged for each plot) on the y-axis and number of threads on the x-axis. The different lines represent the different ratios the operation was executed at.

For example, Problem 1 Pop Ratios line 0.2 means on this line, the `pop` function was executed 20% of the time. Conversely, `push` was executed 80% of the time.

Comparing `pop` and `push` in both problems and focusing on the 1.0 ratio, we can see `pop` is significantly faster than `push`. This is because when `push` isn't putting things on the stack, `pop` has nothing to take off of it.

Now if we look at a `pop` ratio of 0.6, we know `pop` will be able to perform its operation and actually take things off of the stack. The `pop` 0.6 line is significantly higher, as expected.

Let's focus on comparing problems 1 and 2 now. It's clear that in both `pop` and `push`, problem 2 has a faster runtime than problem 1. All of the lines in problem 2 are lower than their problem 1 counterpart and it seems as though problem 2's runtime is increasing at a much slower rate.

It's clear from these four graphs that problem 2 is faster than problem 1 and `pop` is faster than `push`.

### Problem 1: Lock-free, Concurrent Stack

![pop ratios](https://i.imgur.com/lujSOpJ.png)

![push ratios](https://i.imgur.com/EvWwcGj.png)

### Problem 2: Lock-free, Concurrent, Elimination Stack

![pop ratios](https://i.imgur.com/ZGI4BeH.png)

![push ratios](https://i.imgur.com/IuKimbX.png)
