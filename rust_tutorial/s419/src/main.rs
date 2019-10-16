
struct Circle {
  x: f64,
  y: f64,
  radius: f64,
}

// トレイト
  // トレイトはある型が提供しなければならない機能をRustのコンパイラに伝える言語機能です。
trait HasArea {
  fn area(&self) -> f64;
}

impl HasArea for Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
}

impl Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
}

// ジェネリック関数におけるトレイト境界

struct Square {
  x: f64,
  y: f64,
  side: f64,
}

impl HasArea for Square {
  fn area(&self) -> f64 {
    self.side * self.side
  }
}

  // これはコンパイルできません。
// fn print_area<T>(shape: T) {
//   println!("This shape has an area of {}", shape.area());
// }
  // T はあらゆる型になれるため、 area メソッドが実装されているか確認できません。

  // ですがジェネリックな T にはトレイト境界を追加でき、境界が実装を保証してくれます。
fn print_area<T: HasArea>(shape: T) {
  println!("This shape has an area of {}", shape.area());
}

fn main() {
  let c = Circle {
    x: 0.0f64,
    y: 0.0f64,
    radius: 1.0f64,
  };
  let s = Square {
    x: 0.0f64,
    y: 0.0f64,
    side: 1.0f64,
  };
  print_area(c);
  print_area(s);
    // 上記の print_area はジェネリックですが、適切な型が渡されることを保証しています。
    // もし不適切な型を渡すと、
    // print_area(5);
      // コンパイル時エラーが発生します。

  main2();
}

// ジェネリック構造体におけるトレイト境界

struct Rectangle<T> {
  x: T,
  y: T,
  width: T,
  height: T,
}

impl<T: PartialEq> Rectangle<T> {
  fn is_square(&self) -> bool {
    self.width == self.height
  }
}

fn main2() {
  let mut r = Rectangle {
    x: 0,
    y: 0,
    width: 47,
    height: 47,
  };
  assert!(r.is_square());
  r.height = 42;
  assert!(!r.is_square());
    // impl<T: PartialEq> Rectangle<T> { ... }

  // トレイト実装のルール
  trait HasArea {
    fn area(&self) -> f64;
  }
  impl HasArea for i32 {
    fn area(&self) -> f64 {
      println!("this is silly");
      *self as f64
    }
  }
  5.area();
    // しかし例え可能であったとしても、
    // そのようなプリミティブ型のメソッドを実装するのは適切でない手法だと考えられています。

    // これはエラー無しでコンパイルされます。
    // (foo.txtが無いと実行はエラーになる)
  // use std::io::Write;
  // let mut f = std::fs::File::open("foo.txt").expect("Couldn’t open foo.txt");
  // let buf = b"whatever";
  // let result = f.write(buf);

  // Where 節
  foo("Hello", "world");
  bar("Hello", "world");

  main3();
}

// 複数のトレイト境界
// use std::fmt::Debug;
// fn foo<T: Clone + Debug>(x: T) {
//   x.clone();
//   println!("{:?}", x);
// }
  // この T 型は Clone と Debug 両方が必要です。

// Where 節
use std::fmt::Debug;
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
  x.clone();
  y.clone();
  println!("{:?}", y);
}
fn bar<T, K>(x: T, y: K)
  where T: Clone,
        K: Clone + Debug {
  x.clone();
  y.clone();
  println!("{:?}", y);
}

// where は基本の構文よりも強力です。
trait ConvertTo<Output> {
  fn convert(&self) -> Output;
}
impl ConvertTo<i64> for i32 {
  fn convert(&self) -> i64 { *self as i64 }
}
// T == i32の時に呼び出せる
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
  x.convert()
}
// T == i64の時に呼び出せる
fn inverse<T>() -> T
    // これは「ConvertTo<i64>」であるかのようにConvertToを用いている
    where i32: ConvertTo<T> {
  42.convert()
}

// デフォルトメソッド
  // トレイトの定義にデフォルトメソッドを加えることができます。
  // 例えば、以下の is_invalid() は is_valid() の反対として定義されます。
trait Foo {
  fn is_valid(&self) -> bool;
  fn is_invalid(&self) -> bool { !self.is_valid() }
}

// Foo トレイトの実装者は is_valid() を実装する必要がありますが、
// デフォルトの動作が加えられている is_invalid() には必要ありません。
struct UseDefault;
impl Foo for UseDefault {
  fn is_valid(&self) -> bool {
    println!("Called UseDefault.is_valid.");
    true
  }
}
struct OverrideDefault;
impl Foo for OverrideDefault {
  fn is_valid(&self) -> bool {
    println!("Called OverrideDefault.is_valid.");
    true
  }
  fn is_invalid(&self) -> bool {
    println!("Called OverrideDefault.is_invalid!");
    true // 予期されるis_invalid()の値をオーバーライドする
  }
}

fn main3() {
  // Foo トレイトの実装者は is_valid() を実装する必要がありますが、
  // デフォルトの動作が加えられている is_invalid() には必要ありません。
  let default = UseDefault;
  assert!(!default.is_invalid()); // 「Called UseDefault.is_valid.」を表示
  let over = OverrideDefault;
  assert!(over.is_invalid()); // 「Called OverrideDefault.is_invalid!」を表示

  // 継承
  trait Foo {
    fn foo(&self);
  }
  trait FooBar : Foo {
    fn foobar(&self);
  }
  struct Baz;
  impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
  }
  impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
  }

  // Derive
  #[derive(Debug)]
  struct Fooz;
  println!("{:?}", Fooz);
    // 繰り返しDebug や Default のようなトレイトを実装するのは非常にうんざりさせられます。
    // そのような理由から、Rustは自動的にトレイトを実装するための アトリビュート を提供しています。
    // ただし、deriveは以下の特定のトレイトに制限されています。
      // Clone
      // Copy
      // Debug
      // Default
      // Eq
      // Hash
      // Ord
      // PartialEq
      // PartialOrd
}
