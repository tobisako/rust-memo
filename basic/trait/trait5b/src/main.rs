// https://qiita.com/deta-mamoru/items/b9bc953b54d8eea605d5#%E7%B6%99%E6%89%BF%E3%82%B5%E3%83%96%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88
// 親子継承を作ってみる

pub trait Super {
    fn print_super(&self) {
        println!("SUPER");
    }
}

trait Sub: Super {
    fn print_sub(&self) {
        println!("SUB");
    }
}

struct Unit;
impl Super for Unit {}  // 親がimplされてないとエラー。
impl Sub for Unit {}

fn main() {
    let u = Unit;
    u.print_sub();

    println!("Hello, world!");
}
