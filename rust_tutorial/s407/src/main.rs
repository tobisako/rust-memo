fn main() {

  // ムーブセマンティクス
  //let v = vec![1, 2, 3];
  //let v2 = v;
  //println!("v[0] is: {}", v[0]);
    // エラーが発生する：value used here after move
    // 所有権を何か別のものに転送するとき、参照するものを「ムーブした」と言います。 
    // ここでは特別な種類の注釈を必要としません。 それはRustの行うデフォルトの動作です。
    // それは、ヒープ上のベクタの内容へのポインタが2つあることを意味します。 
    // それはデータ競合を持ち込むことでRustの安全性保証に違反するでしょう。 
    // そのため、Rustはムーブを終えた後の v の使用を禁止するのです。

  // Copy型
  let v = 1;
  let v2 = v;
  println!("v is: {}", v);
    // これは当然ＯＫ。コピーが出来る。＝内部でmemalloc()的動作を行っていない為。

  let a = 5;
  let _y = double(a);
  println!("{} {}", a, _y);

  let a2 = true;
  let _y2 = change_truth(a2);
  println!("{} {}", a2, _y2);

}

fn double(x: i32) -> i32 {
  x * 2
}

fn change_truth(x: bool) -> bool {
  !x
}
