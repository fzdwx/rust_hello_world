fn main() {

    let sum: u32 = Counter::create(10).zip(Counter::create(10).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count -= 1;
        if self.count == 0 {
            None
        } else {
            Some(self.count)
        }
    }
}

impl Counter {
    /// create counter
    fn create(count: u32) -> Counter {
        Counter {
            count
        }
    }
}