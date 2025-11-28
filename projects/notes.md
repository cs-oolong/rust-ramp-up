- `println!("Hello")`: `!` means it's a macro, macros are a way to write code that generates code to extend Rust, macros don't always follow the same rules as functions

- Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

- `cargo check` to verify if it compiles, but doesn't generate executables
    - why? faster

- `cargo build` to build, file goes to `target/debug`

- `cargo run` to build + run

- `cargo build --release` for optimized build

