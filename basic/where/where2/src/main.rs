// https://doc.rust-jp.rs/book/second-edition/ch10-02-traits.html

// whereパターンその２

// トレイト１
pub trait Tr1 {
    fn show(&self) -> String {
        String::from("tr1show")
    }
}

// トレイト２
pub trait Tr2 {
    fn chk(&self) -> String {
        String::from("tr2chk")
    }
}

pub struct Memo {
    pub author: String,
    pub memo: String,
}

// トレイト境界を定義。
impl Tr1 for Memo {}
impl Tr2 for Memo {}

// トレイト３
pub trait Tr3 {
    fn chk_a(&self) -> String {
        String::from("tr3chk_a")
    }
}
// トレイト４
pub trait Tr4 {
    fn chk_b(&self) -> String {
        String::from("tr3chk_b")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}
// トレイト境界を定義
impl Tr3 for NewsArticle {}
impl Tr4 for NewsArticle {}

// Tr4しかimplされていないダミー構造体＝条件を満たしていないのでエラーになる
pub struct DummyStruct {
    pub headline: String,
    pub author: String,
    pub content: String,
}
impl Tr4 for DummyStruct {}

// ◆その５：where句で複数パラメータに対応する
fn fn5<T, U>(item1: T, item2: U) -> () where T: Tr1, U: Tr2 {
    println!("show! {}", item1.show());
    println!("chk! {}", item2.chk());
}

// ◆その６：where句で複数パラメータと「＋」の混合パターン
fn fn6<T, U>(item1: T, item2: U) -> () where T: Tr1, U: Tr3+Tr4 {
    println!("show! {}", item1.show());
    println!("chk! {}", item2.chk_a());
    println!("chk! {}", item2.chk_b());
}

fn main() {
    // ◆その５
    let memo1 = Memo {
        author: String::from("Iceburgh"),
        memo: String::from("memo1"),
    };
    let memo2 = Memo {
        author: String::from("Iceburgh"),
        memo: String::from("memo2"),
    };
    fn5(memo1, memo2);

    // ◆その６
    let memo3 = Memo {
        author: String::from("Iceburgh"),
        memo: String::from("memo1"),
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from("hoge in the NHL."),
    };
    fn6(memo3, article);

    // ◆その６：エラーのケース
    // let memo4 = Memo {
    //     author: String::from("Iceburgh"),
    //     memo: String::from("memo1"),
    // };
    // let dummy = DummyStruct {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     author: String::from("Iceburgh"),
    //     content: String::from("hoge in the NHL."),
    // };
    // fn6(memo3, dummy);
    //     // dummyが、「Tr3+Tr4」の条件を満たしていないのでエラー。
}
