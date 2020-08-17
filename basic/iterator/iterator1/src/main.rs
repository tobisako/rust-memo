// https://doc.rust-jp.rs/book/second-edition/ch13-02-iterators.html

// https://sodocumentation.net/ja/rust/topic/4657/%E3%82%A4%E3%83%86%E3%83%AC%E3%83%BC%E3%82%BF
// //         Iterator  Adapter
// //             |       |
// let my_map = (1..6).map(|x| x * x);
// println!("{:?}", my_map);

// 標準ライブラリの定義
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     // デフォルト実装のあるメソッドは省略
// }
    // type ItemとSelf::Item は１９章

fn main() {

    // 1
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    println!("Hello, world!");
}

// 2 nextの呼び出しで得られる値は、ベクタの値への不変な参照
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
    //  iterメソッドは、不変参照へのイテレータを生成します。
    // v1の所有権を奪い、所有された値を返すイテレータを生成したいなら、 iterではなくinto_iterを呼び出すことができます。
    // 同様に、可変参照を繰り返したいなら、 iterではなくiter_mutを呼び出せます。

// 3 イテレータは怠惰。
// 下記sum()の内部ではnext()が繰り返し呼び出され、イテレータを消費している。
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
    // sum()の様に、nextを呼び出すメソッドは、消費アダプタ(consuming adaptors)と呼ばれます。
    // sumは呼び出し対象のイテレータの所有権を奪うので、sum呼び出し後にv1_iterを使用することはできません。

// 4 イテレータアダプタメソッドのmapの呼び出し例を示し、各要素に対して呼び出すクロージャを取り、 新しいイテレータを生成
#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];
    //let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    //assert_eq!(v2, vec![2, 3, 4]);
    let v2: Vec<_> = v1.iter().map(|x| x * 2).collect();
    assert_eq!(v2, vec![2, 4, 6]);
}
    // イテレータを消費するには、collectメソッドを使用





