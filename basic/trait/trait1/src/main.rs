
// トレイト内の「関連型」と「ジェネリック」の違い
// 

// 「ジェネリック」例：この様にジェネリクスを使用すると、各実装で型を注釈しなければならない
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

// 「関連型」例：同じ型に対してトレイトを複数回実装できないので、型を注釈する必要はありません。
// Counterにnextを呼び出す度に、 u32値のイテレータが欲しいと指定しなくてもよいわけです。
// このtypeでは、１回しか型を選択（指定）しないで以後使い続ける事ができて便利。
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// ※後ほど動作サンプルをここに持ってくる。

fn main() {
    println!("Hello, world!");
}
