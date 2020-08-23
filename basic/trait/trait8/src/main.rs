
trait Action {
    fn trait1(&mut self);

    fn trait2(&mut self, xx: u32) {
        println!("trait2() xx={}", xx);
    }

    // fn trait3(&mut self, s: String) {
    //     println!("trait2() s={}", s);
    // }
}

struct MainUnit {
    x: u32,
    s: String
}

impl Action for MainUnit {
    fn trait1(&mut self) {
        self.trait2(self.x);
    }
}

struct MainState {
    main_unit: MainUnit
}

impl MainState {
    fn new() -> MainState {
        let ss = String::from("abc");
        let mu = MainUnit {
            x: 3,
            s: ss
        };
        MainState {
            main_unit: mu,
        }
    }

    // ◆エラーケース：
    pub fn exec(&mut self) {
        println!("{}", self.main_unit.s);
        self.print(&self.main_unit.s);
    }
    fn print(&self, ss: &String) {
        println!("print: {}", ss);
    }

    // ◆解決法１：中身を変更しないので &self レシーバーで渡す
    // pub fn exec2(&mut self) {
    //     println!("exec2() {}", self.main_unit.s);
    //     self.print2(&self.main_unit.s);
    // }
    // fn print2(&self, ss: &String) {
    //     println!("print: {}", ss);
    // }

}

fn main() {

    let mut ms = MainState::new();
    ms.exec();

    // let mut ms2 = MainState::new();
    // ms2.exec2();
}

// これは printが &mut self を要求するのに対して、&self.main_unit.s がもうあるのでRustの借用ルール（&mut があるなら&はとれない）に違反するからです
// 中身を変更しないなら &self レシーバを使う必要があります
// どうしてもmutabilityが必要な場合回避するためのテクニックがいくつかあります

// 典型的には
// fn f(a: &mut Hoge, b: &Huga) {}
// let Struct { a, b, c } = self; 
// f(a, b);
// のようにselfを分解するとか
