// トレイト定義内で処理を書く場合、そのままではselfが解決できない。

trait Action {
    fn trait1(&mut self) {
        println!("trait1()");
        //println!("trait1() {}", self.x);     // ERR! no field `main_unit` on type `&mut Self`
    }
    fn trait2(&mut self);

    fn trait3(&mut self) {
        println!("trait3()");
    }
}

struct MainState {
    x: u32,
}

impl Action for MainState {
    fn trait2(&mut self) {
        println!("trait2() self.x={}", self.x);
        self.trait3();
    }
}

impl MainState {
    fn new() -> MainState {
        MainState {
            x: 3
        }
    }

    pub fn exec(&mut self) {
        self.trait1();
        self.trait2();
    }
}

fn main() {
    let mut ms = MainState::new();
    ms.exec();
}
