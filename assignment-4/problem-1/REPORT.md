# Report for Assignment 4: Problem 1

The deleted node list can be obtained by uncommenting line 75 in `lockfree_stack.rs`, compiling, and running the program. It is optional because of the latency induced to record all of the deleted nodes.

Since it wasn't specified, the address of each node is what is recorded for each node.

## Implementation Summary

My implementation of a lock-free, concurrent stack uses the standard Rust type `AtomicPtr`. `AtomicPtr` allows the user to atomically store and modify a pointer value within.

Each stack node contains two fields: a generic value field and an `AtomicPtr` next field. Every time I need to push or pop a node, I atomically update the head `AtomicPtr` and if I'm pushing to the stack, I set the next `AtomicPtr` field of the new node to the node head was pointing to.

The `push` and `pop` functions each have two parts. The first is to retry the operation until it is complete and the second is to actually perform the operation and report back to the first whether or not it was successful. I implemented exponential backoff to try and improve performance, but for whatever reason, and no matter what timeout value I entered, it only seemed to hurt my performance.

The experimental evaluation can be found in problem 2's documentation.