//! Generators
//!
//! HINT: Look at the `generator_grade.rs` file to see how the generator is used.

/// Yielded value. It can be either a value or a stop signal.
enum Yielded<T> {
    Value(T),
    Stop,
}

/// Generator
/// - You can call `next()` method to get the next value.
/// - The generator should stop when it yields `Yielded::Stop`.
///
/// Reference:
/// - [Python generator](https://python-reference.readthedocs.io/en/latest/docs/generator/)
#[derive(Debug)]
pub struct Generator<T, S> {
    state: S,
    f: fn(&mut S) -> Yielded<T>,
}

impl<T, S> Iterator for Generator<T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.f)(&mut self.state) {
            Yielded::Value(v) => Some(v),
            Yielded::Stop => None,
        }
    }
}

/// Returns a generator that yields fibonacci numbers.
///
/// HINT: Consult <https://en.wikipedia.org/wiki/Fibonacci_sequence>
pub fn fib_generator(first: usize, second: usize) -> Generator<usize, (usize, usize)> {
    let f = |state: &mut (usize, usize)| -> Yielded<usize> {
        let next = state.0 + state.1;
        let prev = state.0;
        state.0 = state.1;
        state.1 = next;
        Yielded::Value(prev)
    };
    let mut init_state = (first, second);
    Generator {
        state: init_state,
        f: f,
    }
}

/// Returns a generator that yields collatz numbers.
///
/// HINT: Consult <https://en.wikipedia.org/wiki/Collatz_conjecture>
pub fn collatz_conjecture(start: usize) -> Generator<usize, usize> {
    let f = |state: &mut usize| -> Yielded<usize> {
        let prev = state.clone();
        if prev == 0 {
            return Yielded::Stop;
        }
        if prev == 1 {
            *state = 0; // 标记状态为终止
            return Yielded::Value(1);
        }
        if *state % 2 == 0 {
            *state = *state / 2;
        } else {
            *state = 3 * *state + 1;
        }
        Yielded::Value(prev)
    };
    Generator { state: start, f: f }
}
