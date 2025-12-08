- `println!("Hello")`: `!` means it's a macro, macros are a way to write code that generates code to extend Rust, macros don't always follow the same rules as functions

- Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

- `cargo check` to verify if it compiles, but doesn't generate executables
    - why? faster

- `cargo build` to build, file goes to `target/debug`

- `cargo run` to build + run

- `cargo build --release` for optimized build

### Macros
- declarative macros (`macro_rules!`)
- procedural macros
    - custom `#[derive]`: specify code added, used on structs and enums
    - attribute-like macros
    - function-like macros

- macros vs functions
    - macros: a way of writing code that writes other code (_metaprogramming_)
    - reduce the amount of code you have to write and maintain
    - support variable number of parameters, not just a fixed amount like functions
    - expanded before the compiler interprets the meaning of the code
    - functions just get called in runtime
    - macro definitions are more complex than function definitions though, more difficult to read, understand etc
    - macros must be brought to scope before being called in a file, while functions can be defined anywhere and called anywhere

Declarative macro aka "macros by example", "macro_rules! macros", "macros".

- The `#[macro_export]` annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope. Without this annotation, the macro can’t be brought into scope.