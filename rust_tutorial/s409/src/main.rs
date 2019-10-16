// 見てのとおり、 struct もライフタイムを持つことができます。 これは関数と同じ方法です。
struct Foo<'a> {
    x: &'a i32,
}

// implブロック
impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
  // ライフタイム
    // 1 私はある種のリソースへのハンドルを取得する
    // 2 私はあなたにリソースへの参照を貸し付ける
    // 3 私はリソースを使い終わり、それを解放することを決めるが、あなたはそれに対する参照をまだ持っている
    // 4 あなたはリソースを使うことを決める

  // structの中
  let y = &5; // これは`let _y = 5; let y = &_y;`と同じ
  let f = Foo { x: y };
  println!("{}", f.x);

  // implブロック
  let y = &5; // これは`let _y = 5; let y = &_y;`と同じ
  let f = Foo { x: y };
  println!("x is: {}", f.x());

  // 'static
    // 「static」と名付けられたライフタイムは特別なライフタイムです。
  let x: &'static str = "Hello, world.";
}

// 黙示的に　※ほぼコチラを使う。
fn foo(x: &i32) {
}

// 明示的に
fn bar<'a>(x: &'a i32) {
}
  // 関数は <> の間に「ジェネリックパラメータ」を持つことができ、ライフタイムはその一種です。 

// ライフタイムの省略
// 例
/*
fn print(s: &str); // 省略された形
fn print<'a>(s: &'a str); // 展開した形

fn debug(lvl: u32, s: &str); // 省略された形
fn debug<'a>(lvl: u32, s: &'a str); // 展開された形

// 前述の例では`lvl`はライフタイムを必要としません。なぜなら、それは参照（`&`）
// ではないからです。（参照を含む`struct`のような）参照に関係するものだけがライ
// フタイムを必要とします。

fn substr(s: &str, until: u32) -> &str; // 省略された形
fn substr<'a>(s: &'a str, until: u32) -> &'a str; // 展開された形

fn get_str() -> &str; // 不正。入力がない

fn frob(s: &str, t: &str) -> &str; // 不正。入力が2つある
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // 展開された形。出力ライフタイムが決まらない

fn get_mut(&mut self) -> &mut T; // 省略された形
fn get_mut<'a>(&'a mut self) -> &'a mut T; // 展開された形

fn args<T:ToCStr>(&mut self, args: &[T]) -> &mut Command; // 省略された形
fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // 展開された形

fn new(buf: &mut [u8]) -> BufWriter; // 省略された形
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // 展開された形
*/