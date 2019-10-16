use std::cell::Cell;

struct Point {
    x: i32,
    y: Cell<i32>,
}

fn main() {

  // ミュータブル参照
  let mut x = 5;
  let y = &mut x;

  // 内側 vs. 外側のミュータビリティ

  // フィールド・レベルのミュータビリティ
    // let mut a = Point { x: 5, y: 6 };
    // a.x = 10;
    // let b = Point { x: 5, y: 6};
    // b.x = 10; // エラー: イミュータブルなフィールド `b.x` へ代入できない
  // Cell<T> を使えば、フィールド・レベルのミュータビリティをエミュレートできます。
  let point = Point { x: 5, y: Cell::new(6) };
  point.y.set(7);
  println!("y: {:?}", point.y);
}
