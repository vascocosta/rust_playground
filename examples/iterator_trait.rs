struct FibonacciIterator {
    current: usize,
    next: usize,
}

impl FibonacciIterator {
    fn new() -> Self {
        Self {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for FibonacciIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next += current;

        Some(current)
    }
}

struct Fibonacci;

impl IntoIterator for Fibonacci {
    type Item = usize;
    type IntoIter = FibonacciIterator;
    fn into_iter(self) -> Self::IntoIter {
        FibonacciIterator::new()
    }
}

fn main() {
    let fib = Fibonacci;

    for n in fib.into_iter().take(20) {
        println!("{n}");
    }
}
