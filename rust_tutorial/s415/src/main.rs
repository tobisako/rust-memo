struct Circle {
  x: f64,
  y: f64,
  radius: f64,
}

// impl Circle {
//   fn area(&self) -> f64 {
//     std::f64::consts::PI * (self.radius * self.radius)
//   }
// }

// メソッドに渡す特別な第1引数として、 self 、 &self 、 &mut self という3つの変形があります。
impl Circle {
    fn reference(&self) {
       println!("taking self by reference!");
    }

    fn mutable_reference(&mut self) {
       println!("taking self by mutable reference!");
    }

    fn takes_ownership(self) {
       println!("taking ownership of self!");
    }
}
  // 好きな数だけ impl ブロックを使用することができます。前述の例は以下のように書くこともできるでしょう。

impl Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
    // 単に Circle を返しているだけです。このメソッドにより、私たちは新しい Circle を任意の大きさに拡大することができます。
  fn grow(&self, increment: f64) -> Circle {
    Circle { x: self.x, y: self.y, radius: self.radius + increment }
  }
}

impl Circle {
  // あなたは self を引数に取らない関連関数を定義することもできます。
    // 以下のパターンはRustのコードにおいて非常にありふれた物です。
  fn new(x: f64, y: f64, radius: f64) -> Circle {
    Circle {
      x: x,
      y: y,
      radius: radius,
    }
  }
}

struct CircleBuilder {
  x: f64,
  y: f64,
  radius: f64,
}

impl CircleBuilder {
  fn new() -> CircleBuilder {
    CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
  }

  fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
    self.x = coordinate;
    self
  }

  fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
    self.y = coordinate;
    self
  }

  fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
    self.radius = radius;
    self
  }
    // finalizeで、Circleを返している。
  fn finalize(&self) -> Circle {
    Circle { x: self.x, y: self.y, radius: self.radius }
  }
}

fn main() {
  // メソッド構文
    // baz(bar(foo));   
    // foo.bar().baz();   // 可能。

  // メソッド呼び出し
  let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
  println!("{}", c.area());
    // 円を表す struct

  // メソッドチェーン
  let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
  println!("{}", c.area());
  let d = c.grow(2.0).area();
  println!("{}", d);

  // 関連関数
  let c = Circle::new(0.0, 0.0, 2.0);

  // Builderパターン
  let c = CircleBuilder::new()
    .x(1.0)
    .y(2.0)
    .radius(2.0)
    .finalize();
  println!("area: {}", c.area());
  println!("x: {}", c.x);
  println!("y: {}", c.y);

}
