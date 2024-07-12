fn greeting() -> &'static str {
    "Hello World!" // implicit return
}

// primitive types of input must match return type
fn compute(a: u32, b: u32) -> u32 {
    let x: u32 = 2; // explicit type
    let y = x; // compiler will infer the type based on the context
    a + b * y
}

fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    if time_elapsed == 0 {
        panic!("The journey took no time at all, that's impossible!")
    }
    let distance: u32 = end - start;
    distance / time_elapsed
}

pub fn factorial(n: u32) -> u32 {
    // Rust variables are immutable by default.
    // In order to change the value of a variable, it must be declared mutable
    let mut result: u32 = 1;
    let mut i = n;
    while i >= 1 {
        result *= i;
        i -= 1;
    }
    result
}

pub fn factorial_for(n: u32) -> u32 {
    let mut result: u32 = 1;

    // There are five kinds of ranges in Rust:
    //    1..5: A (half-open) range. It includes all numbers from 1 to 4. It doesn't include the last value, 5.
    //    1..=5: An inclusive range. It includes all numbers from 1 to 5. It includes the last value, 5.
    //    1..: An open-ended range. It includes all numbers from 1 to infinity (well, until the maximum value of the integer type).
    //    ..5: A range that starts at the minimum value for the integer type and ends at 4. It doesn't include the last value, 5.
    //    ..=5: A range that starts at the minimum value for the integer type and ends at 5. It includes the last value, 5.

    for i in 1..=n {
        result *= i;
    }
    result
}

// The `#[cfg(test)]` attribute tells the compiler to only compile the code below when
// running tests (i.e. when you run `cargo test`).
#[cfg(test)]
mod tests {
    use crate::{compute, factorial, factorial_for, greeting, speed};

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "Hello World!");
    }

    #[test]
    fn test_compute() {
        assert_eq!(compute(2, 2), 6)
    }

    #[test]
    // With the `#[should_panic]` annotation we can assert that we expect the code
    // under test to panic. We can also check the panic message by using `expected`.
    // This is all part of Rust's built-in test framework
    #[should_panic(expected = "The journey took no time at all, that's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120)
    }

    #[test]
    fn test_factorial_for() {
        assert_eq!(factorial_for(5), 120)
    }
}
