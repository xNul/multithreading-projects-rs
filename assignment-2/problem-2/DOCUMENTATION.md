# Documentation and Writing for Assignment 2: Problem 2

### The Three Strategies

// figure out these names and make sure is readable and check if is random access assignment?

Three different strategies were given to discuss.

The first strategy describes a scenario where guests are trying to get into the room with the vase but they have no idea or guarantee how long it will take or if they will ever be able to get into the room. This scenario is analogous to a Mutex Lock. Only one guest may be in the room at a time. That guest may stay in there as long as they wish. New guests enter at random. Just as with a Mutex Lock, only one thread may write to a particular location of memory at a time, the thread may keep the Mutex Lock as long as they wish, and normally Mutex Locks are assigned to waiting threads at basically random.

As with the given scenario, a lock prevents all others from accessing memory. If another thread needs to access that memory, it will be stalled until it can obtain the lock. In a bad design, this might mean never. The entire thread could be stalled forever. This is a big problem because the point of parallelism is to improve performance, but the lock blocks the thread, rendering it useless! In the worst case, a deadlock can be caused, likely crashing the entire program. That being said, locks aren't bad and if designed correctly and implemented for the right situation, are great tools.

The second strategy describes a Non-Blocking Mutex Lock where no thread is stalled and threads can only read the current state of the lock. If another thread holds the lock, it is known. If the lock is free, it is known as well. Just as the sign by the door will either say "BUSY" or "AVAILABLE".

Non-Blocking Mutex Locks are similar to normal Mutex Locks except, they do not have the ability to stall the thread and wait for the Mutex Lock to be released. They only have the ability to tell the current status of the Mutex Lock. Thus, threads can only use the Mutex Lock if it happens to be free when the thread checks. Of course, if a thread finds that the Mutex Lock is busy, it could loop until the lock is available but the lock is not designed for this purpose. It is not intended to be used in that way. Though, it makes it effectively the same as a normal Mutex Lock.

The third strategy is a Queueable Mutex Lock. A Queueable Mutex Lock blocks threads and causes them to stall similar to the first strategy, but when a thread tries to obtain a lock it is put in a queue for access to the lock and not randomly selected. Over time and one-by-one the threads which were queued before the current thread will get the lock and release it until it finally reaches our thread. Our thread is dequeued, it gets the lock, and it can modify/read the memory.

In a Queueable Mutex Lock, as long as threads release the lock fairly quickly and for small amounts of time during the runtime of the program, there is good parallelism, no data races, and minimal stalling.

Out of these three strategies, a Queueable Mutex Lock seems the best. Yes, threads will be blocked unlike strategy 2, but in this scenario, they would be blocked anyway. The code would have to loop to wait for the lock to be open, meanwhile making, extra, unnecessary memory accesses to figure out the state of the lock. Additionally, normal Mutex Lock access would basically be assigned at random. We don't want a thread (or rather a guest) to potentially never enter the room. Another guest could enter the room five times by chance while the original guest is still waiting.

### Brief Approach Summary

Overall, the program creates threads and randomly assigns a guest to each one. Each guest attempts to enter the room and see the vase aka a location in memory with a Queueable Mutex Lock. The lock blocks the thread and it waits in a queue to recieve the lock if it is already being used. A counter is incremented for the first time a guest has entered the room with the vase. Once all guests make it through, the program ends. This is done to test the duration of the program with the experimental evaluation script.

A `Mutex` object is shared between threads and when a thread attempts to use that object to lock the mutex, a queue is checked. The current thread is added to the queue and if no thread has the lock, it is given to the current thread. Any threads which attempt to obtain the lock at this time, are enqueued. When the current thread finishes or drops the lock, it is dequeued and given to the next thread in the queue. The queue is FIFO.

### Experimental Evaluation Summary

I created a Bash script called `benchmark.sh` to sample runtime performance 100 times at 50 guests and 10 times with increments of 10 guests. When I needed to check the performance of my program and any changes I made, I ran it to see how they performed.

1. 1000 times at 50 guests: average 0.0090 seconds per iteration
2. 100 times with increments of 10 guests: average 0.0002 seconds per guest

A description of each is commented in my code.
