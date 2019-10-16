enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {

  // マッチ
  let x = 5;
  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
  }
    // 「網羅性検査」が実施されます。 
    // 上のコードで、最後のアンダースコア( _ )を用いている腕があるのがわかりますか？ 
    // もし、その腕を削除した場合、Rustは以下の様なエラーを発生させます:

  let number = match x {
    1 => "one",
    2 => "two",
    3 => "three",
    4 => "four",
    5 => "five",
    _ => "something else",
  };

  // 列挙型に対するマッチ

}

fn quit() { /* ... */ }
fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
fn move_cursor(x: i32, y: i32) { /* ... */ }

fn process_message(msg: Message) {
  match msg {
    Message::Quit => quit(),
    Message::ChangeColor(r, g, b) => change_color(r, g, b),
    Message::Move { x: x, y: y } => move_cursor(x, y),
    Message::Write(s) => println!("{}", s),
  };
}
