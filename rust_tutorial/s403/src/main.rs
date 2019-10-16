fn main() {
  // char=4bytes
  let x = 'x';
  let two_hearts = '💕';
  println!("Hello, world! {} {} ", x, two_hearts);

  // 配列
  let a = [1, 2, 3];
  println!("a has {} elements", a.len());

  let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
  println!("The second name is: {}", names[1]);

  // タプル・タプルのインデックス
  let x: (i32, &str) = (1, "hello");
  println!("x.0: {}" , x.0);
  println!("x.1: {}" , x.1);

  // 関数
  fn foo(x: i32) -> i32 { x * 2 }
  let x: fn(i32) -> i32 = foo;
  println!("x = {}", x(10));
}
