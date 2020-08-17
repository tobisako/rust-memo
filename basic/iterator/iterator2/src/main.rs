// https://doc.rust-jp.rs/book/second-edition/ch13-02-iterators.html

// 環境をキャプチャするクロージャを使用する
// ここでいう環境って何だ？

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// イテレータの.filter()は、イテレータの各要素を取り、論理値を返すクロージャを取ります
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}
    // shoes_in_my_size関数は、引数として靴のベクタとサイズの所有権を奪います。
    // 指定されたサイズの靴だけを含むベクタを返します。

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
