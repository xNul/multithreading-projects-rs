# Documentation and Writing for Assignment 2: Problem 1

### Approach Summary

First, to simulate the minotaur's selection of guests, I randomly assign guests to their own thread. To achieve parallelism and maintain randomization of the ordering of guests, I perform guest calculates in their own thread. The OS scheduler then decides which guest goes in what order, but it is pretty random.

Once a thread/guest reaches the end of the labyrinth and its own cupcake check, a cupcake boolean is atomically accessed. If there is a cupcake and the current guest has not eaten one, it will eat the cupcake. If there isn't a cupcake or the guest has already eaten their cupcake, the cupcake state isn't changed.

All of the above holds true except for a single guest with the number 0. This guest is designated the cupcake counter. They will be the sole guest responsible for replacing cupcakes and with each cupcake replaced, guest 0 will increment a counter. When this counter reaches `n-1`, it is guaranteed all guests have made it through the labyrinth at least once.

When all guests have made it through the labyrinth, an atomic exit flag is set to true. All threads check for this flag and when it is true, they exit naturally. At this point the program is finished and the minotaur's problem has been solved.

### Experimental Evaluation Summary

I created a Bash script called `benchmark.sh` to sample runtime performance 1000 times at 50 guests and 100 times with increments of 10 guests. The program stops when all guests have entered the vase room at least once to test this measurement. When I needed to check the performance of my program and any changes I made, I ran this script to see how it performed.

1. 100 times at 50 guests: average 1.1500 seconds per iteration
2. 10 times with increments of 10 guests: average 0.0345 seconds per guest

A description of each is commented in my code.
