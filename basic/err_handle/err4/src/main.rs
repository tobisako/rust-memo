//https://qiita.com/nirasan/items/321e7cc42e0e0f238254

// panic を起こさない unwrap の類似メソッド

fn main() {
    let result: Result<i32, String> = Err("error".to_string());
    println!("{:?}", result.unwrap_or(10)); //=> 10

    let result: Result<i32, String> = Err("error".to_string());
    println!("{:?}", result.unwrap_or_else(|_| 20)); //=> 20

    let option: Option<i32> = None;
    println!("{:?}", option.unwrap_or(100)); //=> 100

    let option: Option<i32> = None;
    println!("{:?}", option.unwrap_or_else(|| 200)); //=> 200
}
