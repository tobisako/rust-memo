  // Rustが標準ライブラリで提供している型 Option<T> はジェネリックです。
  // <T> の部分は、前に少し見たことがあると思いますが、
  // これがジェネリックなデータ型であることを示しています。 
  // enum の宣言内であれば、どこでも T を使うことができ、
  // 宣言内に登場する同じ型をジェネリック内で T 型に置き換えています。

enum Option<T> {
  Some(T),
  None,
}
  // 慣習としては、「Type」から第1ジェネリックパラメータは T であるべきですし、
  // 「Error」から E を用いるのですが、
  // Rustは気にしません。
// enum Result<A, Z> {
//     Ok(A),
//     Err(Z),
// }
enum Result<T, E> {
  Ok(T),
  Err(E),
}

  // struct 内にジェネリックな型の値を保存することもできます。
struct Point<T> {
  x: T,
  y: T,
}

  // ジェネリックな struct に実装を追加したい場合、 impl の後に型パラメータを宣言するだけです。
impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

fn main() {

  // ジェネリクス
    // 型注釈を用いたOption<T>の使用例が以下になります。
  //let x: Option<i32> = Some(5);
  //let x: Option<f64> = Some(5);
  //let x: Option<i32> = Some(5);
  //let y: Option<f64> = Some(5.0f64);
    // ?ビルド出来ない

  // ジェネリック構造体
  let int_origin = Point { x: 0, y: 0 };
  let float_origin = Point { x: 0.0, y: 0.0 };

}

// ジェネリック関数
fn takes_anything<T>(x: T) {
  // xで何か行う
}
fn takes_two_of_the_same_things<T>(x: T, y: T) {
  // ...
}
fn takes_two_things<T, U>(x: T, y: U) {
  // ...
}

