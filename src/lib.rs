//! # slowprint
//!
//! A small library for printing texts slowly which gives scrolling effect to the texts shown.
use std::io::{self, Write};
use std::ops::Add;
use std::thread;
use std::time::Duration;

const DEFAULT_PRINT_DELAY: Duration = Duration::from_millis(100);

/// Print given text slowly with delay of given Duration.
///
/// # Examples
///
/// ```no_run
/// use slowprint::slow_print;
///
/// fn main() {
///     let delay = std::time::Duration::from_millis(200);
///     slow_print("Hello, Rust.", delay);
///     slow_print(&"Hello, Rust.".to_string(), delay);
///     slow_print(&String::from("Hello, Rust."), delay);
/// }
/// ```
pub fn slow_print(data: &str, delay: Duration) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for character in data.chars() {
        handle.write_all(character.to_string().as_bytes()).unwrap();
        handle.flush().unwrap();
        thread::sleep(delay);
    }
}

/// Print given text slowly with delay of given Duration and add a newline at the end of text.
///
/// # Examples
///
/// ```no_run
/// use slowprint::slow_println;
///
/// fn main() {
///     let delay = std::time::Duration::from_millis(200);
///     slow_println("Hello, Rust.", delay);
///     slow_println(&"Hello, Rust.".to_string(), delay);
///     slow_println(&String::from("Hello, Rust."), delay);
/// }
/// ```
pub fn slow_println(data: &str, delay: Duration) {
    slow_print(&data.to_string().add(&"\n"), delay);
}

/// Print given text slowly with delay of fixed Duration `Duration::from_millis(1000)`.
///
/// # Examples
///
/// ```no_run
/// use slowprint::print;
///
/// fn main() {
///     print("Hello, Rust.");
///     print(&"Hello, Rust.".to_string());
///     print(&String::from("Hello, Rust."));
/// }
/// ```
pub fn print(data: &str) {
    slow_print(data, DEFAULT_PRINT_DELAY);
}

/// Print given text slowly with delay of fixed Duration `Duration::from_millis(1000)` and add a newline at the end of the text.
// # Examples
///
/// ```no_run
/// use slowprint::println;
///
/// fn main() {
///     println("Hello, Rust.");
///     println(&"Hello, Rust.".to_string());
///     println(&String::from("Hello, Rust."));;
/// }
/// ```
pub fn println(data: &str) {
    slow_println(data, DEFAULT_PRINT_DELAY);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_slow_print() {
        let delay = std::time::Duration::from_millis(50);
        let now = std::time::Instant::now();
        slow_print("Hello, Rust.", delay);
        assert!(now.elapsed() >= delay);
    }

    #[test]
    fn test_slow_println() {
        let delay = std::time::Duration::from_millis(100);
        let now = std::time::Instant::now();
        slow_println("Hello, Rust.", delay);
        assert!(now.elapsed() >= delay);
    }
    #[test]
    fn test_print() {
        let now = std::time::Instant::now();
        print("Hello, Rust.");
        assert!(now.elapsed() >= DEFAULT_PRINT_DELAY);
    }

    #[test]
    fn test_println() {
        let now = std::time::Instant::now();
        println("Hello, Rust.");
        assert!(now.elapsed() >= DEFAULT_PRINT_DELAY);
    }
}
