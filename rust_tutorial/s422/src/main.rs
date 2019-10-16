// トレイトオブジェクト
  // コードがポリモーフィズムを伴う場合、
  // 実際に実行するバージョンを決定するメカニズムが必要です。
  // これは「ディスパッチ」(dispatch)と呼ばれます。
  // ディスパッチには主に静的ディスパッチと動的ディスパッチという2つの形態があります。
  // Rustは静的ディスパッチを支持している一方で、
  // 「トレイトオブジェクト」(trait objects)と呼ばれるメカニズムにより
  // 動的ディスパッチもサポートしています。

trait Foo {
  fn method(&self) -> String;
}
impl Foo for u8 {
  fn method(&self) -> String { format!("u8: {}", *self) }
}
impl Foo for String {
  fn method(&self) -> String { format!("string: {}", *self) }
}

fn do_something<T: Foo>(x: T) {
  x.method();
}

fn do_something_u8(x: u8) {
  x.method();
}
fn do_something_string(x: String) {
  x.method();
}

fn main() {

  // 静的ディスパッチ
    // トレイト境界を使ってこのトレイトで静的ディスパッチが出来ます。
  let x = 5u8;
  let y = "Hello".to_string();
  do_something(x);
  do_something(y);
    // これはRustが u8 と String それぞれ専用の do_something() を作成し、
    // それら特殊化された関数を宛てがうように呼び出しの部分を書き換えるという意味です。

  let x = 5u8;
  let y = "Hello".to_string();
  do_something_u8(x);
  do_something_string(y);
    // これは素晴らしい利点です。呼び出される関数はコンパイル時に分かっているため、
    // 静的ディスパッチは関数呼び出しをインライン化できます。インライン化は優れた最適化の鍵です。    
    // 標準ライブラリは可能な限り静的ディスパッチで実装するよう心がけています。
  main2();
}

fn do_something2(x: &Foo) {
  x.method();
}

fn main2() {
  // 動的ディスパッチ
    // Rustは「トレイトオブジェクト」と呼ばれる機能によって動的ディスパッチを提供しています。
  let x = 5u8;
  do_something2(&x as &Foo);
    // または型強制によって、
  let x2 = "Hello".to_string();
  do_something2(&x2);

  // 何故ポインタなのか？
    // Rustは型によってサイズが違います。

  // トレイトオブジェクトの内部表現
    // トレイトのメソッドはトレイトオブジェクト内にある伝統的に
    // 「vtable」（これはコンパイラによって作成、管理されます）と呼ばれる
    // 特別な関数ポインタのレコードを介して呼び出されます。
    // 単純な例として、トレイトオブジェクトの実行時の表現から見て行きましょう。 
    // std::raw モジュールは複雑なビルドインの型と同じレイアウトの構造体を格納しており、 
    // トレイトオブジェクトも含まれています 。
  mod foo {
    pub struct TraitObject {
      pub data: *mut (),
      pub vtable: *mut (),
    }
  }
    // つまり、 &Foo のようなトレイトオブジェクトは
    // 「data」ポインタと「vtable」ポインタから成るわけです。

    // vtableは本質的には関数ポインタの構造体で、
    // 実装内における各メソッドの具体的な機械語の命令列を指しています。 
    // trait_object.method() のようなメソッド呼び出しを行うとvtableの中から適切なポインタを取り出し
    // 動的に呼び出しを行います。例えば、
//   struct FooVtable {
//     destructor: fn(*mut ()),
//     size: usize,
//     align: usize,
//     method: fn(*const ()) -> String,
//   }

// // u8:
// fn call_method_on_u8(x: *const ()) -> String {
//   // コンパイラは `x` がu8を指しているときにのみこの関数が呼ばれることを保障します
//   let byte: &u8 = unsafe { &*(x as *const u8) };
//   byte.method()
// }

// static Foo_for_u8_vtable: FooVtable = FooVtable {
//   destructor: /* コンパイラマジック */,
//   size: 1,
//   align: 1,
//   // 関数ポインタへキャスト
//   method: call_method_on_u8 as fn(*const ()) -> String,
// };

// // String:
// fn call_method_on_String(x: *const ()) -> String {
//   // コンパイラは `x` がStringを指しているときにのみこの関数が呼ばれることを保障します
//   let string: &String = unsafe { &*(x as *const String) };
//   string.method()
// }

// static Foo_for_String_vtable: FooVtable = FooVtable {
//   destructor: /* コンパイラマジック */,
//   // この値は64bitコンピュータ向けのものです、32bitコンピュータではこの半分にします
//   size: 24,
//   align: 8,
//   method: call_method_on_String as fn(*const ()) -> String,
// };

  // 例えば Foo を実装する値を幾つか得たとします。 
  // Foo トレイトオブジェクトを作る、あるいは使う時のコードを明示的に書いたものは少しだけ似ているでしょう。
  // （型の違いを無視すればですが。どのみちただのポインタになります）
  // let a: String = "foo".to_string();
  // let x: u8 = 1;
  // // let b: &Foo = &a;
  // let b = TraitObject {
  //   // データを保存
  //   data: &a,
  //   // メソッドを保存
  //   vtable: &Foo_for_String_vtable
  // };
  // // let y: &Foo = x;
  // let y = TraitObject {
  //   // データを保存
  //   data: &x,
  //   // メソッドを保存
  //   vtable: &Foo_for_u8_vtable
  // };
  // // b.method();
  // (b.vtable.method)(b.data);
  // // y.method();
  // (y.vtable.method)(y.data);

  // オブジェクトの安全性
    // 全てのトレイトがトレイトオブジェクトとして使えるわけではありません。
    // 例えば、ベクタは Clone を実装していますが、トレイトオブジェクトを作ろうとすると、
  // let v = vec![1, 2, 3];
  // let o = &v as &Clone;
    // エラーが発生します。
    // 「特別な状況を除いて、
    // トレイトのメソッドで Self を使うとオブジェクト安全ではなくなる」と考えるのが良いでしょう。
}

