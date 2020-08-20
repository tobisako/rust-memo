// https://qiita.com/nirasan/items/321e7cc42e0e0f238254

// Option は空になる可能性のある値を表現するために用意された列挙型

// Optionは標準定義されている。
    // enum Option<T> {
    //     None, // 値が空
    //     Some(T), // 値がある
    // }

fn main() {
    let options = [Some(-1), None];

    for o in options.iter() {
        println!("Option: {:?}", o);

        let double = match o {
            Some(v) => v * 2,
            None => 0
        };
        println!("match result: {}", double); //=> -2

        if let Some(v) = o {
            println!("if let result: {}", v);
        }

        println!("")
    }
}
