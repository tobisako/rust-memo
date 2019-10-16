trait Foo {
  fn f(&self);
}
trait Bar {
  fn f(&self);
}
struct Baz;
impl Foo for Baz {
  fn f(&self) { println!("Baz’s impl of Foo"); }
}
impl Bar for Baz {
  fn f(&self) { println!("Baz’s impl of Bar"); }
}

fn main() {
  // 共通の関数呼び出し構文
  let b = Baz;
    // 以下はエラーとなる。
    //b.f();

    // そのようなフィーチャーは 「共通の関数呼び出し構文」と呼ばれ、以下のように書けます:
  Foo::f(&b);
  Bar::f(&b);

  main2();
}

trait Foo2 {
  fn foo2() -> i32;
}
struct Bar2;
impl Bar2 {
  fn foo2() -> i32 {
    20
  }
}
impl Foo2 for Bar2 {
  fn foo2() -> i32 {
    10
  }
}

fn main2() {
  // 山括弧形式
  assert_eq!(10, <Bar2 as Foo2>::foo2());
  assert_eq!(20, Bar2::foo2());
}
