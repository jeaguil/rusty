fn greeting() -> &'static str {
    "Hello World!" // implicit return
}

// primitive types of input must match return type
fn compute(a: u32, b: u32) -> u32 {
    let x: u32 = 2; // explicit type
    let y = x; // compiler will infer the type based on the context
    a + b * y
}

// The `#[cfg(test)]` attribute tells the compiler to only compile the code below when
// running tests (i.e. when you run `cargo test`).
#[cfg(test)]
mod tests {
    use crate::{compute, greeting};

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "Hello World!");
    }

    #[test]
    fn test_compute() {
        assert_eq!(compute(2, 2), 6)
    }
}
