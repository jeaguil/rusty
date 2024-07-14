# rusty - personal learn-by-doing refernce to the syntax, type system, and ecosystem of rust 

# rust ecosystem

**profiles** -
A profile is a set of configuration options that can be used to customize the way Rust code is compiled.

Cargo provides two built-in profiles: dev and release.
The dev profile is used every time you run `cargo build`, `cargo run` or `cargo test`. It's aimed at local development, therefore it sacrifices runtime performance in favor of faster compilation times and a better debugging experience.
The release profile, instead, is optimized for runtime performance but incurs longer compilation times. You need to explicitly request via the --release flag—e.g. `cargo build --release` or `cargo run --release`.

# rust key concetps

**ownership, references, and borrowing** -
Rust's system ensures memory safety without needing a garbage collector. It prevents data races and ensures that memory is cleaned up when no longer needed.

- Ownership: Each value in Rust has a single owner, and when the owner goes out of scope, the value is dropped.
- References: Allow you to refer to some value without taking ownership. They can be immutable (`&T`) or mutable (`&mut T`).
- Borrowing: When you pass references to functions, you're borrowing the value. Mutable borrows (`&mut T`) allow you to modify the value.

**memory management** -
Efficient memory management is key for performance and safety. Rust's system avoids common bugs like null pointer dereferencing, double frees, and use-after-free.

- Stack: Used for fixed-size, simple data (like integers). It’s fast because the allocation and deallocation happen in a predictable manner.
- Heap: Used for dynamic, complex data (like `String` or `Vec`). It’s slower due to the need for allocating and freeing memory.
- Pointers: `Box<T>, Rc<T>, and Arc<T>` are types that manage heap data. `Box<T>` provides single ownership, `Rc<T> and Arc<T>` provide reference-counted ownership.
- Destructors: Automatically clean up resources when a value goes out of scope using Rust’s `Drop` trait.
