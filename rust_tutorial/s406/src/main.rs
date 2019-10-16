fn main() {

  // while
  let mut x = 5; // mut x: i32
  let mut done = false; // mut done: bool
  while !done {
    x += x - 3;
    println!("{}", x);
    if x % 5 == 0 {
      done = true;
    }
  }

  // for
  //let mut x2;
  //for (x2 = 0; x2 < 10; x2++) {
  //  printf( "%d\n", x2 );
  //}

  //for var in expression {
  //  code
  //}
  for x in 0..10 {
    println!("{}", x); // x: i32
  }

  // 列挙
  for (i,j) in (5..10).enumerate() {
    println!("i = {} and j = {}", i, j);
  }

  // イテレーターを対象に:
  //for (linenumber, line) in lines.enumerate() {
  //  println!("{}: {}", linenumber, line);
  //}

  // ループラベル
  'outer: for x in 0..10 {
    'inner: for y in 0..10 {
      if x % 2 == 0 { continue 'outer; } // x のループを継続
      if y % 2 == 0 { continue 'inner; } // y のループを継続
      println!("x: {}, y: {}", x, y);
    }
  }

  // 無限
  //loop {
    println!("Loop forever!");
  //}
}
