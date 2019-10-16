struct Point {
    x: i32,
    y: i32,
}

// use std::result::Result;

fn main() {
  // パターン
    // パターンには一つ落とし穴があります。
    // 新しい束縛を導入する他の構文と同様、パターンはシャドーイングをします。例えば：
  let x = 'x';
  let c = 'c';
  match c {
    x => println!("x: {} c: {}", x, c), // 元のxがシャドーイングされて、別のxとして動作している。
  }
  println!("x: {}", x);
    // x => は値をパターンにマッチさせ、マッチの腕内で有効な x という名前の束縛を導入します。
    // 既に x という束縛が存在していたので、新たに導入した x は、その古い x をシャドーイングします。

  // 複式パターン
  let x2 = 1;
  match x2 {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // 分配束縛
  let origin = Point { x: 0, y: 0 };
  match origin {
    Point { x, y } => println!("({},{})", x, y),
  }

    // 値に別の名前を付けたいときは、 : を使うことができます。
  let origin2 = Point { x: 0, y: 0 };
  match origin2 {
    Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
  }

    // 値の一部にだけ興味がある場合は、値のすべてに名前を付ける必要はありません。
  let origin = Point { x: 0, y: 0 };
  match origin {
    Point { x, .. } => println!("x is {}", x),
  }

    // 最初のメンバだけでなく、どのメンバに対してもこの種のマッチを行うことができます。
  let origin = Point { x: 0, y: 0 };
  match origin {
    Point { y, .. } => println!("y is {}", y),
  }

  // 束縛の無視
  let some_value: Result<i32, &'static str> = Err("There was an error");
  match some_value {
    Ok(value) => println!("got a value: {}", value),
    Err(_) => println!("an error occurred"),
  }

    // ここでは、タプルの最初と最後の要素に x と z を束縛します。
  fn coordinate() -> (i32, i32, i32) {
    // generate and return some sort of triple tuple
    // 3要素のタプルを生成して返す
    (1, 2, 3)
  }
  let (x, _, z) = coordinate();

    // 同様に .. でパターン内の複数の値を無視することができます。
  enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
  }
  let x = OptionalTuple::Value(5, -2, 3);
  match x {
    OptionalTuple::Value(..) => println!("Got a tuple!"),
    OptionalTuple::Missing => println!("No such luck."),
  }

  // ref と ref mut
    // 参照 を取得したいときは ref キーワードを使いましょう。
  let x3 = 5;
  match x3 {
    ref r => println!("Got a reference to {}", r),
  }
    // ミュータブルな参照が必要な場合は、同様に ref mut を使います。
  let mut x = 5;
  match x {
    ref mut mr => {
      *mr += 1;
      println!("Got a mutable reference to {}", mr);
    },
  }
  println!("x = {}", x);  // 値が書き換わっている。

  // 範囲
  let x = 1;
  match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything"),
  }
    // 範囲は多くの場合、整数か char 型で使われます：
  let x = '💅';
  match x {
    'a' ... 'j' => println!("early letter"),
    'k' ... 'z' => println!("late letter"),
    _ => println!("something else"),
  }

  // 束縛
  let x = 1;
  match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
  }

    // 内側の name の値への参照に a を束縛します。
  #[derive(Debug)]
  struct Person {
    name: Option<String>,
  }
  let name = "Steve".to_string();
  let mut x: Option<Person> = Some(Person { name: Some(name) });
  match x {
    Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
    _ => {}
  }

    // @ を | と組み合わせて使う場合は、それぞれのパターンで同じ名前が束縛されるようにする必要があります：
  let x = 5;
  match x {
    e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
    _ => println!("anything"),
  }

  // ガード
    // if を使うことでマッチガードを導入することができます：
  enum OptionalInt {
    Value(i32),
    Missing,
  }
  let x = OptionalInt::Value(5);
  match x {
    OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
    OptionalInt::Value(..) => println!("Got an int!"),
    OptionalInt::Missing => println!("No such luck."),
  }

    // 複式パターンで if を使うと、 if は | の両側に適用されます：
  let x = 4;
  let y = false;
  match x {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
  }
    // イメージ： (4 | 5) if y => ...

  // 混ぜてマッチ
    // やりたいことに応じて、それらを混ぜてマッチさせることもできます：
  // match x {
  //   Foo { x: Some(ref name), y: None } => { println!("foo"); },
  // }
    // パターンはとても強力です。上手に使いましょう。
}
