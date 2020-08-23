struct MainState {
    s: String
}

impl MainState {
    fn new() -> MainState {
        let ss = String::from("abc");
        MainState {
            s: ss,
        }
    }

    // ◆０：無問題
    fn exec0(&mut self) {
        self.print0();
    }
    fn print0(&mut self) {
        println!("print0!");    // MainStateのメンバへアクセスしていないのでOK
    }

    // // ◆１：エラーになるケースa
    // fn exec1a(&mut self) {
    //     self.print1a(&self.s);
    // }
    // fn print1a(&mut self, s: &String) {
    //     println!("print1a: {}", s);
    // }
        //    |         self.print1a(&self.s);
        //    |         ^^^^^-------^-------^
        //    |         |    |       |
        //    |         |    |       immutable borrow occurs here
        //    |         |    immutable borrow later used by call
        //    |         mutable borrow occurs here

    // // ◆２：エラーになるケースb
    // fn exec1b(&mut self) {
    //     self.print1b(&mut self.s);
    // }
    // fn print1b(&mut self, s: &mut String) {
    //     println!("print1b: {}", s);
    // }
        //    |         self.print1b(&mut self.s);
        //    |         ^^^^^-------^-----------^
        //    |         |    |       |
        //    |         |    |       first mutable borrow occurs here
        //    |         |    first borrow later used by call
        //    |         second mutable borrow occurs here

    // ◆２：解決法・中身を変更しない事を前提に &self レシーバーで渡す
    fn exec2(&mut self) {
        self.print2(&self.s);
    }
    fn print2(&self, s: &String) {
        println!("print2: {}", s);
    }
}

fn main() {

    // ◆０：無問題
    let mut ms = MainState::new();
    ms.exec0();

    // // ◆１：エラーになるケースa
    // let mut ms = MainState::new();
    // ms.exec1a();

    // // ◆２：エラーになるケースb
    // let mut ms = MainState::new();
    // ms.exec1b();

    // ◆２：解決法
    let mut ms = MainState::new();
    ms.exec2();
}
