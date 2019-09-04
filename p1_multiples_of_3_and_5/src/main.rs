use std::collections::HashMap;

struct MultipleIterator {
    multiple: u64,
    max: u64,
    count: u64
}

impl MultipleIterator {
    fn new(multiple: u64, max: u64) -> Self {
        MultipleIterator {multiple, max, count: 0}
    }
}

impl Iterator for MultipleIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        let res = self.count * self.multiple;

        if res < self.max {
            Some(res)
        } else {
            None
        }
    }
}

enum Collect {
    IsCollected,
}

fn main() {
    let mut h: HashMap<u64, Collect> = HashMap::new();

    let m3: u64 = MultipleIterator::new(3, 1000)
        .inspect(|n: &u64| {
            h.insert(*n, Collect::IsCollected);
        })
        .sum();
    let m5: u64 = MultipleIterator::new(5, 1000).filter(
        |n: &u64| {
            match h.get(n) {
                None => true,
                _ => false
            }
        }
    ).sum();

    println!("{:?}", m3 + m5);
}
