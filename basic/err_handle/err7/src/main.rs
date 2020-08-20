// https://qiita.com/nirasan/items/321e7cc42e0e0f238254

// Option のいろいろな操作

fn main() {

    // 値がある時だけ中の値を加工する map
    let options = [Some(100), None];
    println!("{:?}", options[0].map(|n| n * 2)); //=> Some(200)
    println!("{:?}", options[1].map(|n| n * 2)); //=> None


    // 値がある時だけ処理を実行する and_then
    println!("---");
    let options = [Some(100), None];

    let f = |n| Some(n * 2);
    println!("{:?}", options[0].and_then(f)); //=> Some(200)
    println!("{:?}", options[1].and_then(f)); //=> Err("Error")

    let f = |n| Some(format!("number is {}", n));
    println!("{:?}", options[0].and_then(f)); //=> Some("number is 100")


    // Option から Result に変換する ok_or
    println!("---");
    let options = [Some(100), None];
    println!("{:?}", options[0].ok_or("err")); //=> Ok(100)
    println!("{:?}", options[1].ok_or("err")); //=> Err("err")


    // Option 同士の和や積を求める and or


}
