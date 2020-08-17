// https://doc.rust-jp.rs/book/second-edition/ch13-02-iterators.html

// Iteratorトレイトで独自のイテレータを作成する
// Iteratorトレイトを自分で実装することで、したいことを何でもするイテレータを作成することもできます。

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// ※「std::iter::Iterator」に、「Counter」を組み込む。
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
    // 詳しくは１９章で。

fn main() {
    println!("Hello, world!");
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
}
