# slowprint
A small library for printing texts slowly which gives scrolling effect to the texts shown.

## Install
To install slowprint, please run the following command :

```bash
cargo install slowprint
```

## Examples

We can import/use slowprint library functions in either of the three ways:
1. Import all functions of slowprint library crate into our local namespace. Example : `use slowprint::*;`

2. Import only required functions of slowprint library crate into our local namespace. Example : `use slowprint::slow_println;` to use `slow_println` function.

3. Use fully qualified name to use any method of slowprint library crate. Example: `use slowprint;` and then we can call method `println` of slowprint crate like this `slowprint::println("Hello, World");`.


As per the standards, third way is the preferred way, but other ways make it easier to use methods of crate/library repeatedly.

Some examples are provided below for user's references.

### Example 1
Use the default delay time to print slowly.
```rust
use slowprint::print;

print("Hello, Rust.");
print(&"Hello, Rust.".to_string());
print(&String::from("Hello, Rust."));
``` 

### Example 2
Use the default delay time to print slowly with newline added at the end.
```rust
use slowprint::println;

println("Hello, Rust.");
println(&"Hello, Rust.".to_string());
println(&String::from("Hello, Rust."));
``` 

### Example 3
Pass the delay time to print slowly.
```rust
use slowprint::slow_print;

let delay = std::time::Duration::from_millis(200);
slow_print("Hello, Rust.", delay);
slow_print(&"Hello, Rust.".to_string(), delay);
slow_print(&String::from("Hello, Rust."), delay);
``` 
### Example 4 
Pass the delay time to print slowly with newline added at the end.
```rust
use slowprint::slow_println;

let delay = std::time::Duration::from_millis(200);
slow_println("Hello, Rust.", delay);
slow_println(&"Hello, Rust.".to_string(), delay);
slow_print(&String::from("Hello, Rust."), delay);
``` 