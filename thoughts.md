# Cool things in Rust 
- [Shadowing](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#shadowing)
 - Allows for nice type conversion
- [Matching Arms](https://doc.rust-lang.org/stable/book/ch06-02-match.html)
 - Returning values in the form of a `Result` enum
 - Binding a [value to a match](https://doc.rust-lang.org/stable/book/ch06-02-match.html#patterns-that-bind-to-values) 
  - Matches are exhaustive, you need to catch every condition, can use `other` as a catch all which automatically binds the variable use `_` if you want to catch all and not bind the variable
  - `match` vs. `if let` is a tradeoff between exhaustive checking or less more concise code
- No `return` statements, just write an expression if its the last line in the function
 - Concept of `expression` and `statement`
- One of the few languages without [garbage collection](https://wiki.c2.com/?LanguagesWithoutGarbageCollection) or the need for it since memory is managed through a system of ownership and rules
- [Ownership](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html) 
  - When an owner is out of scope the variable is dropped
  - Rust calls [drop](https://doc.rust-lang.org/stable/std/ops/trait.Drop.html#tymethod.drop) when a variable goes out of scope
  - When you declare a new value you call a `move` rather than a shallow / deep copy
    - Need to call `clone()` if you do not want to invalidate the first variable 
  - ```
    These ampersands are references, and they allow you to refer to some value without taking ownership of it
    ```
  - [References](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html) can be very helpful when you want to pass an immutable reference to a function. Unless you use a [mutable reference](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html#mutable-references)
- Tuple Structs
- Automatic referencing and dereferencing
  - If you are calling a method on a pointer, no need to dereference the pointer with `*` to get the object, rust will do this automatically
  ```
  p1.distance(&p2);
  (&p1).distance(&p2);
  ``` this makes some code much cleaner
- Attach data directly to an enum
- Add implementations into an enum
- No `null` values, instead using the `Option<T>` enum
- Everything is private by default you even need to re-export a imported module with `pub use` 
  - "With pub use, we can write our code with one structure but expose a different structure"
- [Lifetimes and Traits](https://doc.rust-lang.org/stable/book/ch10-02-traits.html)
  - 

### Development Resources
- [Playground](https://play.rust-lang.org/)
- [Easy rust](https://github.com/Dhghomon/easy_rust)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Awesome rust](https://github.com/rust-unofficial/awesome-rust)
- [Half hour to learn rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- [Learning Rust](https://learning-rust.github.io/docs/a1.why_rust.html)