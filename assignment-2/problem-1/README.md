# Assignment 2 - Problem 1

## Compiling Rust from Command Prompt

1. If you do not have Cargo (the Rust package manager) and the Rust compiler, those need to be installed. To get these two, Windows users must go to https://rustup.rs/ and install rustup with the `rustup-init.exe`.
2. You should now have rustup and be able to use it from Command Prompt. You now need to install a Rust toolchain. To do this, you may run the command `rustup install stable` in Command Prompt.
3. Cargo should now be available from Command Prompt and with Cargo, my Rust program can be compiled. Use `cd` to enter my project folder in Command Prompt. Once there, run `cargo run --release 100`. This should compile my program and run it automatically with 100 guests. Note, the `--release` flag is very important for runtime peformance.

Any issues in the installation process such as environment variables or otherwise can be referenced [here](https://www.rust-lang.org/tools/install), [here](https://www.rust-lang.org/learn/get-started), and [here](https://forge.rust-lang.org/infra/other-installation-methods.html).

If issues toolchain issues persist, feel free to reach out to me. Rust has a very developed ecosystem and shouldn't be very hard to set up.
