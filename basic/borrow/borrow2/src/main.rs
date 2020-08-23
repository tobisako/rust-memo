
struct MainUnit {
    s: String,
}

struct SubUnit {
    s: String,
}

struct Base {
    main_unit: MainUnit, 
    sub_unit: SubUnit
}

fn test(m: &mut MainUnit, s: &SubUnit) {
    println!("test: {},{}", m.s, s.s);
}

impl Base {
    fn new() -> Base {
        let mu = MainUnit { s: String::from("main") };
        let su = SubUnit { s: String::from("sub") };
        Base {
            main_unit: mu,
            sub_unit: su
        }
    }

    fn exec(&mut self) {

        // ※このケースでは分解せずとも渡せてしまう。→利用すべきケースが他に在るのでは
        test(&mut self.main_unit, &self.sub_unit);

        // このケースでは分解すると逆にエラー
        // let Base { main_unit, sub_unit } = self;
        // test(&mut main_unit, &sub_unit);
    }
}

fn main() {
    let mut ms = Base::new();
    ms.exec();
}

// fn f(a: &mut Hoge, b: &Huga) {}
// let Struct { a, b, c } = self; 
// f(a, b);
