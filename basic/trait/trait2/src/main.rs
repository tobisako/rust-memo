// https://doc.rust-jp.rs/book/second-edition/ch19-03-advanced-traits.html

// デフォルトのジェネリック型引数と演算子オーバーロード
// Rustでは、独自の演算子を作ったり、任意の演算子をオーバーロードすることはできません。
// しかし、演算子に紐づいたトレイトを実装することで
// std::opsに列挙された処理と対応するトレイトをオーバーロードできます。

// ジェネリックな型に既定の型を指定する記法は、
// ジェネリックな型を宣言する際に<PlaceholderType=ConcreteType>です。

// https://doc.rust-jp.rs/rust-by-example-ja/trait/ops.html

use std::ops::Add;
    // trait Add<RHS=Self> {
    //     type Output;
    //     fn add(self, rhs: RHS) -> Self::Output;
    // }

use std::ops::AddAssign;
    // pub trait AddAssign<Rhs = Self> {
    //     fn add_assign(&mut self, Rhs);
    // }

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        println!("オーバーロード！");
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
