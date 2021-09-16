# Documentation and Writing for Assignment 3: Problem 1

### Approach Summary

For this problem, I used a Lock-Free Linked List implemented on my own in the Rust programming language. Overall it was very similar to the PowerPoint slides gone over in class except for a single large distinction: Rust. Rust makes data structures such as Linked Lists more complicated to implement, but faster than another language such as Java. To build a Lock-Free Linked List in Rust, in addition to all of the standard Lock-Free implementation, I had to constrain myself to Rust's strict ownership and borrowing rules and at the same time, manually manage my memory.

### Thank You Notes

At the end of the day, the servants had more presents than thank you notes because many 'presents' were marked to receive a 'thank you note' (aka be removed from the list), but were unable to be removed immediately during the `remove` function and so instead were marked to be removed for later clean up by the `find` function, however, once the program finished, some lingering, marked nodes for clean up remained in the list. To fix this, there are a number of things that can be done. The program can be made to, at the end of the standard process, to clean up the remaining marked nodes or the threads can be made to recognize the marked nodes when they remain and each take on work to remove those nodes from the list.

### Improving their strategy

Besides the improvements I mentioned above for cleaning up the marked nodes at the end of the program, I'm not sure what else can be done. I solved the problem in the most efficient way I know how so I'm not sure what else can be done to improve.

### Experimental Evaluation Summary

For this experiment I deviated from my usual bash script for running the program and calculating the results automatically to manually reporting my results, however, my program does report the runtime in seconds when the program is run.

2.2 Ghz processor
N = 500,000
Time = 0.133 Seconds

N = 1,000,000
Time = 0.298 Seconds

N = 2,000,000
Time = 0.591 Seconds

As you can see, a pretty impressive performance was achieved. 2.2 Ghz is the fastest CPU I have access to at this time, however, I imagine most conventional CPU will give similar results.