struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

// ほとんどの場合タプル構造体よりも struct を使ったほうが良いです。 
// Color や Point はこのようにも書けます。
  // struct Color {
  //   red: i32,
  //   blue: i32,
  //   green: i32,
  // }
  // struct Point {
  //   x: i32,
  //   y: i32,
  //   z: i32,
  // }

// // Rustは言語レベルでフィールドのミュータビリティに対応していない
// struct Point {
//     mut x: i32,
//     y: i32,
// }

struct Electron;

fn main() {

  // 構造体
  let mut point = Point { x: 0, y: 0 };
  point.x = 5;
  println!("The point is at ({}, {})", point.x, point.y);

  // 以下の方法で少しの間だけミュータブルな構造体を作ることができます。
  let mut point = Point { x: 0, y: 0 };
  point.x = 5;
  let point = point; // この新しい束縛でここから変更できなくなります
  //point.y = 6; // これはエラーになります

  // アップデート構文
  let mut point3 = Point3d { x: 0, y: 0, z: 0 };
  point3 = Point3d { y: 1, .. point3 };
  let origin3 = Point3d { x: 0, y: 0, z: 0 };
  let point3 = Point3d { z: 1, x: 2, .. origin3 };

  // タプル構造体
    // タプル構造体自体には名前がありますが、そのフィールドには名前がありません。
  struct TColor(i32, i32, i32);
  struct TPoint(i32, i32, i32);
    // これら2つは同じ値を持つ同士であったとしても等しくありません。
  let black = TColor(0, 0, 0);
  let orig = TPoint(0, 0, 0);

  // ただし、タプル構造体が非常に便利な場合も あります。要素が1つだけの場合です。
  struct Inches(i32);
  let length = Inches(10);
  let Inches(integer_length) = length;
  println!("length is {} inches", integer_length);

  // Unit-like 構造体
    // 全くメンバを持たない struct を定義することもできます。
  let x = Electron;
}
