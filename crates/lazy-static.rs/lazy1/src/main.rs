#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

// rust は static でグローバル変数を作る事ができるが、
// config 等ファイルを読み込んで static 化させるには「lazy_static!」を使うと便利。

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        println!("start");
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        println!("END");
        m
    };
}

fn func1() {
    // Any further access to `HASHMAP` just returns the computed value
    println!("The entry for `1` is \"{}\".", HASHMAP.get(&1).unwrap());
}

fn main() {
    println!("main()");

    func1();

    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

}
