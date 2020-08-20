// https://qiita.com/nirasan/items/321e7cc42e0e0f238254

fn main() {

    // 成功した時だけ中の値を加工する map
    let results = [Ok(100), Err("Error")];
    println!("{:?}", results[0].map(|n| n * 2)); //=> Ok(200)
    println!("{:?}", results[1].map(|n| n * 2)); //=> Err("Error")
        // map はクロージャを受け取り Result の値が Ok の時だけクロージャを実行して中の値を加工します。
        // 値が Err なら何もしません。
        // ※失敗した時だけ中の値を加工したい場合は map_err メソッドが用意されています。

    
    // 成功した時だけ処理を実行する and_then
    println!("---");
    let results = [Ok(100), Err("Error")];

    let f = |n| Ok(n * 2);
    println!("{:?}", results[0].and_then(f)); //=> Ok(200)
    println!("{:?}", results[1].and_then(f)); //=> Err("Error")

    let f = |n| Ok(format!("number is {}", n));
    println!("{:?}", results[0].and_then(f)); //=> Ok("number is 100") 
        // and_then は map と同様に Result の値が Ok の時だけクロージャを実行します。
        // map が Ok の中の値を加工するのに対して、
        // and_then はその名の通りに任意の処理を実行して新たな Result を作成して返却します。
        // and_then の入力値と出力値で Result の型が変わる様な使い方もできます。
        // ※失敗した時だけ処理を実行したい場合は or_else メソッドが用意されています。


    // Result から Option に変換する ok err
    println!("---");
    let results = [Ok(100), Err("err")];

    println!("{:?}", results[0].ok()); //=> Some(100)
    println!("{:?}", results[0].err()); //=> None
    println!("{:?}", results[1].ok()); //=> None
    println!("{:?}", results[1].err()); //=> Some("err")
        // ok は Result<T, E> を Option<T> に変換します
        // err は Result<T, E> を Option<E> に変換します


    // 中の値を参照にする as_ref as_mut
        // as_ref は Result<T, E> を Result<&T, &E> に変換します
        // as_mut は Result<T, E> を Result<&mut T, &mut E> に変換します


    // Result 同士の和や積を求める and or
    let results = [Ok(1), Ok(2), Err("E1"), Err("E2")];
    println!("---");
    println!("{:?}", results[0].and(results[1])); //=> Ok && Ok   = Ok(2)
    println!("{:?}", results[0].and(results[2])); //=> Ok && Err  = Err("E1")
    println!("{:?}", results[2].and(results[1])); //=> Err && Ok  = Err("E1")
    println!("{:?}", results[2].and(results[3])); //=> Err && Err = Err("E1")
    println!("{:?}", results[0].or(results[1])); //=> Ok || Ok   = Ok(1)
    println!("{:?}", results[0].or(results[2])); //=> Ok || Err  = Ok(1)
    println!("{:?}", results[2].or(results[1])); //=> Err || Ok  = Ok(2)
    println!("{:?}", results[2].or(results[3])); //=> Err || Err = Err("E2")
}


