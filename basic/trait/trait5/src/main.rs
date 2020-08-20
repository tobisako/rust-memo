// https://doc.rust-jp.rs/book/second-edition/ch19-03-advanced-traits.html
// スーパートレイトを使用して別のトレイト内で、あるトレイトの機能を必要とする

use std::fmt;

// 「trait」に「:」を付けて、別トレイト
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {

    // Pointインスタンスに対してoutline_printを呼び出し、アスタリスクのふちの中に表示することができます。
    let p = Point {x: 2, y: 4};
    p.outline_print();

    println!("Hello, world!");
}
