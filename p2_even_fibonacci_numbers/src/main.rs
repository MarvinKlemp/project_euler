struct Fib {
    curr: u32,
    next: u32,
}

impl Fib {
    fn new() -> Self {
        Fib{curr: 1, next: 1}
    }
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new = self.curr + self.next;

        self.curr = self.next;
        self.next = new;

        Some(self.curr)
    }
}


fn main() {
    let sum: u32 = Fib::new().
        take_while(|&x| x < 4000000).
        filter(|&x| x % 2 == 0).
        sum();

    println!("sum: {}", sum);
}