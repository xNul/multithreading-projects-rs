# Assignment 4 - Problem 1

## Compiling Rust from Command Prompt

1. If you do not have Cargo (the Rust package manager) and the Rust compiler, those need to be installed. To get them, Windows users should go to https://rustup.rs/ and install rustup with the `rustup-init.exe`.
2. You should now have rustup and be able to use it from Command Prompt. You now need to install a Rust toolchain. To do this, you may run the command `rustup install stable` in Command Prompt.
3. Cargo should now be available from Command Prompt and with Cargo, my Rust program can be compiled. Use `cd` to enter my project folder in Command Prompt. Once there, run `cargo run --release`. This should compile my program and run it automatically. Note the `--release` flag is very important for runtime peformance.

Any issues in the installation process such as environment variables or otherwise can be referenced [here](https://www.rust-lang.org/tools/install), [here](https://www.rust-lang.org/learn/get-started), and [here](https://forge.rust-lang.org/infra/other-installation-methods.html).

If issues toolchain issues persist, feel free to reach out to me. Rust has a very developed ecosystem and shouldn't be very hard to set up.

## Running the Experiments

To test my implementation and create the required graphs, I created some helper functions. These helper functions can be uncommented in the main loop to be run. They exist in my problem-1 solution and my problem-2 solution so that the graph results can be compared.

These helper functions print results to the console in a CSV format, meaning, the console output can be piped to a CSV file and then those files can be loaded in Excel or another tool to graph. However, before they are run, print statements inside the `lock_free_stack_problem` function have to be commented out so that they do not break the CSV formatting.
