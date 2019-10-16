
// match option {
//   Some(x) => { foo(x) },
//   None => {},
// }
  //  ↓
// if option.is_some() {
//   let x = option.unwrap();
//   foo(x);
// }
  //  ↓
// if let Some(x) = option {
//   foo(x);
// } else {
//     bar();
// }

fn main() {
  // if let
    // if let によってif と let を一体化して用いることが可能となり、 
    // ある種のパターンマッチに伴うオーバーヘッドを削減することができます。
  let option = Some(5);
  fn foo(x: i32) { }
  fn bar() { }
  if let Some(x) = option {
    foo(x);
  } else {
    bar();
  }

  // while let
  // let mut v = vec![1, 3, 5, 7, 11];
  // loop {
  //   match v.pop() {
  //     Some(x) =>  println!("{}", x),
  //     None => break,
  //   }
  // }
  //  ↓
  let mut v = vec![1, 3, 5, 7, 11];
  while let Some(x) = v.pop() {
    println!("{}", x);
  }
}
