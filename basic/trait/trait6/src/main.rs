// https://doc.rust-jp.rs/book/second-edition/ch19-03-advanced-traits.html
// ニュータイプパターンを使用して外部の型に外部のトレイトを実装する

// ニュータイプという用語は、 Haskellプログラミング言語に端を発しています。

// 例として、Vec<T>にDisplayを実装したい
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
    // このテクニックを使用する欠点は、Wrapperが新しい型なので、
    // 保持している値のメソッドがないことです。 
    // self.0に委譲して、WrapperをVec<T>と全く同様に扱えるように、
    // Wrapperに直接Vec<T>の全てのメソッドを実装しなければならないでしょう。
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
