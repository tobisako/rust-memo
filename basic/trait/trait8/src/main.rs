
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

    fn print(&mut self, ss: &String) {
        println!("print: {}", ss);
    }

    pub fn exec(&mut self) {
        //println!("{}", self.main_unit.s);
        self.print(&self.main_unit.s);
    }
}

// error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
//   --> src/main.rs:47:9
//    |
// 47 |         self.print(&self.main_unit.s);
//    |         ^^^^^-----^-----------------^
//    |         |    |     |
//    |         |    |     immutable borrow occurs here
//    |         |    immutable borrow later used by call
//    |         mutable borrow occurs here

fn main() {
    let mut ms = MainState::new();
    ms.exec();
}
