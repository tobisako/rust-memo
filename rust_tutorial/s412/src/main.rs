
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

fn main() {

  // 列挙型
  let x: Message = Message::Move { x: 3, y: 4 };
    // 各ヴァリアントの名前を使うためには、 :: 構文を使います。 
    //すなわち、ヴァリアントの名前は enum 自体の名前によってスコープ化されています。 
    // これにより、以下のどちらもうまく動きます。
  let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
    // どちらのヴァリアントも Move という名前ですが、
    // 列挙型の名前でスコープ化されているため、衝突することなく使うことができます。

  // 関数としてのコンストラクタ
  let m = Message::Write("Hello, world".to_string());
  let x = foo("Hello, world".to_string());
}

fn foo(x: String) -> Message {
  Message::Write(x)
}
