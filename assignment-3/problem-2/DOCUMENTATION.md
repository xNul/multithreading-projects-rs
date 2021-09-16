# Documentation and Writing for Assignment 3: Problem 1

### Approach Summary

For this problem, I used a Lock-Free Linked List implemented on my own in the Rust programming language. Overall it was very similar to the PowerPoint slides gone over in class except for a single large distinction: Rust. Rust makes data structures such as Linked Lists more complicated to implement, but faster than another language such as Java. To build a Lock-Free Linked List in Rust, in addition to all of the standard Lock-Free implementation, I had to constrain myself to Rust's strict ownership and borrowing rules and at the same time, manually manage my memory.

### Overview

The Lock-Free Linked List continually processes the input given by the sensors and there is no waiting for locks by the sensors. They can immediately add their sample to the shared Lock-Free Linked List memory and let the rest of the program take care of it. The report is generated/printed by utilizing the ordering property of Lock-Free Linked Lists to more efficiently grab the Lowest and Highest 5 temperatures per simulated hour.

### Experimental Evaluation Summary

There isn't much to evaulate for this experiment, however, you can tell how fast my program runs with an average of iterations. The runtime is also listed in the output of the program.

With a 2.2 Ghz processor and over 1,000 iterations, the average runtime is 0.000521 and a common output for the temperatures is
Lowest 5 Temperatures: [-100, -100, -100, -99, -98]
Highest 5 Temperatures: [67, 68, 68, 69, 69]
It wasn't specified if the temperatures were supposed to be unique, so I kept redundant information.