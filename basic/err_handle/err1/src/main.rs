// https://qiita.com/nirasan/items/321e7cc42e0e0f238254
// Rust のエラーハンドリングはシンタックスシュガーが豊富で完全に初見殺しなので自信を持って使えるように整理してみたら完全に理解した

// Result は失敗する可能性のある関数の返り値のために用意された列挙型

// 「Result」は標準で定義されている。
    // enum Result<T, E> {
    //     Ok(T), // 成功
    //     Err(E), // 失敗
    // }

fn main() {
    let results = [Ok(100), Err("oops!")];

    for r in results.iter() {
        println!("Result: {:?}", r);
        let double = match r {
            Ok(v) => v * 2,
            Err(_) => 0,
        };
        println!("match result: {}", double);

        if let Ok(v) = r {
            println!("if let result: {}", v);
        }

        println!("")
    }
}

