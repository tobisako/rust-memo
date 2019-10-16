fn main() {
  // ベクタ
  let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    // vec! には初期値の繰り返しを表現するための形式があります:
  let v = vec![0; 10]; // 0が10個

  // 要素へのアクセス
  let v = vec![1, 2, 3, 4, 5];
  println!("The third element of v is {}", v[2]);
    // また、インデックスは usize 型でなければならない点に注意しましょう:
  let v = vec![1, 2, 3, 4, 5];
  let i: usize = 0;
  let j: i32 = 0;
  // これは動作します
  v[i];
  // 一方、こちらは動作しません
  // v[j];

  // イテレーティング
  let mut v = vec![1, 2, 3, 4, 5];
  for i in &v {
    println!("A reference to {}", i);
  }
  for i in &mut v {
    println!("A mutable reference to {}", i);
  }
  for i in v {
    println!("Take ownership of the vector and its element {}", i);
  }
}
