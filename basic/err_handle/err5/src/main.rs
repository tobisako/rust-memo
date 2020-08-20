// https://qiita.com/nirasan/items/321e7cc42e0e0f238254

// ? 演算子

// ? 演算子が便利なのは Result を受け取ったときにエラーならすぐに処理を中止して return したいというときで、
// よくある処理を少ない記述量で手軽に実装することができます。

// Result を返す double 関数のなかで同じく Result を返す odd が呼び出され ? 演算子を使ってエラーがハンドリングされています。
fn main() {
    let odd = |n| -> Result<i32, String> {
        if n % 2 == 1 {
            Ok(n)
        } else {
            Err(format!("{} is not odd", n))
        }
    };

    let double = |n| -> Result<i32, String> {
        println!("A");
        let n = odd(n)?; // odd が Err ならすぐに return する
        println!("B");
        return Ok(n * 2);
    };

    for n in 0 .. 4 {
        println!("number: {}", n);
        println!("double result: {:?}\n", double(n));
    }

    // Optionでも。
    println!("-----");
    let double = |n: Option<i32>| -> Option<i32> {
        println!("Option: {:?}", n);
        let m = n?;
        println!("C");
        return Some(m * 2);
    };
    println!("{:?}\n", double(Some(100)));
    println!("{:?}", double(None));
}
