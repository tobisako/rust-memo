// https://qiita.com/nirasan/items/321e7cc42e0e0f238254

// unrwap

// // 使い方
// fn main() {
//     let result: Result<i32, String> = Ok(2);
//     println!("{:?}", result.unwrap()); //=> 2

//     let result: Result<i32, String> = Err("error".to_string());
//     println!("{:?}", result.unwrap()); //=> panic!
// }

// Option も unwrap メソッドを持っており、Result と同じように Some なら中の値を返し、None なら panic を起こします
fn main() {
    let option: Option<i32> = Some(2);
    println!("{:?}", option.unwrap()); //=> 2

    let option: Option<i32> = None;
    println!("{:?}", option.unwrap()); //=> panic
}
