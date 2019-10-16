struct HasDrop;

  // Trait std::ops::Drop
    // 標準で定義されているトレイト。
  // pub trait Drop {
  //   fn drop(&mut self);
  // }

impl Drop for HasDrop {
  fn drop(&mut self) {
    println!("Dropping!");
  }
}

struct Firework {
  strength: i32,
}

impl Drop for Firework {
  fn drop(&mut self) {
    println!("BOOM times {}!!!", self.strength);
  }
}

fn main() {
  // Drop
  let x = HasDrop;
    // いくつかの処理

  // しかし少しだけ注意すべき点があります。 
  // たとえば、値がドロップされる順序は、それらが定義された順序と反対の順序になります:
  let firecracker = Firework { strength: 1 };
  let tnt = Firework { strength: 100 };
    // このコードは以下の様な出力をします:
      // BOOM times 100!!!
      // BOOM times 1!!!

} // x はここでスコープ外になります

// Drop は何をするのに適しているのでしょうか？
// 一般的に Drop は struct に関連付けられているリソースのクリーンアップに利用されます。 
// たとえば、 Arc<T> 型 は参照カウントを行う型です。 
// Drop が呼ばれると、参照カウントがデクリメントされ、 
// もし参照の合計数が0になっていたら、内包している値がクリーンアップされます。

