// where1
// 参考：https://doc.rust-jp.rs/book/second-edition/ch10-02-traits.html

// トレイト境界(bounds)：ジェネリック型に対して「このトレイトを実装していなければならない」という制約を課すもの

// トレイト
pub trait Tr {
    fn tr(&self) -> String {
        String::from("tr")
    }
}

// トレイト２
pub trait Tr2 {
    fn tr2(&self) -> String {
        String::from("tr2")
    }
}

// 構造体
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// トレイト境界を定義。
impl Tr for Tweet {}

// トレイト境界を定義。
impl Tr2 for Tweet {}


/////////////////////////////////////////////////////

// ◆その１：whereを使わない形
// トレイト境界をジェネリックな型引数宣言とともにコロンの後、山カッコ内に配置しています。
fn fn1<T: Tr>(item: T) {
    println!("Breaking news! {}", item.tr());
}
    // fn fn1<T: Tr>(item: T) {
    //           ↑この「Tr」が、トレイト境界。

// ◆その２：その１をwhere化（「その１」と「その２」は全く同じ）
fn fn2<T>(item: T) -> () where T: Tr {
//fn fn2<T>(item: T) where T: Tr {
    println!("Breaking news! {}", item.tr());
}
    // where句でトレイト境界指定をうしろに纏める表記もある。

// ◆その３：その２は、こんなふうにも書ける（全く同じ）
// fn fn3<T>(item: T) -> () where T: Tr {
    // ()は、戻り値が無いから。

// ◆その４：「+」記法で、トレイト境界を複数指定できる。
fn fn4<T>(item: T) -> () where T: Tr+Tr2 {
    println!("Breaking news! {}", item.tr());
    println!("Breaking news! {}", item.tr2());
}


fn main() {

    // ◆その１
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    fn1(tweet);

    // ◆その２
    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    fn2(tweet2);

    // ◆その４
    let tweet4 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    fn4(tweet4);
}
