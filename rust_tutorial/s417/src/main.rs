fn main() {
  // 文字列
  let greeting = "Hello there."; // greeting: &'static str
    // "Hello there." は文字列リテラルで、 &'static str 型を持ちます。 
    // 文字列リテラルは、静的にアロケートされた文字列スライスです。
    // これはつまりコンパイルされたプログラム内に保存されていて、 
    // プログラムの実行中全てにわたって存在しているということです。 
    // greetingの束縛はこのように静的にアロケートされた文字列を参照しています。 
    // 文字列スライスを引数として期待している関数はすべて文字列リテラルを引数に取ることができます。

  let s = "foo
    bar";
  assert_eq!("foo\n    bar", s);          // OK
  // assert_eq!("foo\n            bar", s);  // NG-assert

    // もう一つは \ を使って空白と改行を削る形式です:
  let s = "foo\
    bar";
  assert_eq!("foobar", s);

    // Rustには &str だけでなく、 String というヒープアロケートされる文字列もあります。 
    // この文字列は伸張可能であり、またUTF-8であることも保証されています。 
    // String は一般的に文字列スライスを to_string メソッドで変換することで作成されます。
  let mut s = "Hello".to_string(); // mut s: String
  println!("{}", s);
  s.push_str(", world.");
  println!("{}", s);

    // String は & によって &str に型強制されます。
  let s = "Hello".to_string();
  takes_slice(&s);

    // このような関数には &str は渡せますが、 
    // String は &* を用いて明示的に変換しなければなりません。
  use std::net::TcpStream;
  TcpStream::connect("192.168.0.1:3000"); // 引数として &str を渡す
  let addr_string = "192.168.0.1:3000".to_string();
  TcpStream::connect(&*addr_string); // addr_string を &str に変換して渡す

  // インデクシング
    // 文字列は妥当なUTF-8であるため、文字列はインデクシングをサポートしていません:
  // let s = "hello";
  // println!("The first letter of s is {}", s[0]); // エラー!!!

    // 文字列をバイト列として見るかコードポイント列として見るか選ぶことができます。
  let hachiko = "忠犬ハチ公";
  for b in hachiko.as_bytes() {
    print!("{}, ", b);
  }
  println!("");
  for c in hachiko.chars() {
    print!("{}, ", c);
  }
  println!("");

    // インデクシングするのと近い結果を以下の様にして得ることができます:
  let dog = hachiko.chars().nth(1); // hachiko[1]のような感じで

  // スライシング
  let dog = "hachiko";
  let hachi = &dog[0..5];
    // しかし、注意しなくてはならない点はこれらのオフセットは
    // バイト であって 文字 のオフセットではないという点です。 
    // そのため、以下のコードは実行時に失敗します:
  // let dog = "忠犬ハチ公";
  // let hachi = &dog[0..2];

  // 連結
    // String が存在するとき、 &str を末尾に連結することができます:
  let hello = "Hello ".to_string();
  let world = "world!";
  let hello_world = hello + world;

    // しかし、2つの String を連結するには、 & が必要になります:
  let hello = "Hello ".to_string();
  let world = "world!".to_string();
  let hello_world = hello + &world;
    // これは、 &String が自動的に &str に型強制されるためです。 
    // このフィーチャは 「 Deref による型強制 」と呼ばれています。
}

fn takes_slice(slice: &str) {
  println!("Got: {}", slice);
}
