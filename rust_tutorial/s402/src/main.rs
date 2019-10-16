fn main() {
  let ans = print_sum(5, 6);
  println!("sum is: {}", ans);

  // 関数ポインタ
  let f: fn(i32) -> i32;
  f = plus_one;

  let six = f(5);
  println!("six is: {}", six);
}

fn print_sum(x: i32, y: i32) -> i32 {
  x + y
}
// セミコロン不要

fn plus_one(i: i32) -> i32 {
  i + 1
}
