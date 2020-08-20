// https://doc.rust-jp.rs/book/second-edition/ch19-03-advanced-traits.html

// 明確化のためのフルパス記法: 同じ名前のメソッドを呼ぶ

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        // キャプテンのお言葉
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        // 上がれ！
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        // *激しく腕を振る*
        println!("*waving arms furiously*");
    }
}

fn main() {
    // 明示的なトレイト呼び出し
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
