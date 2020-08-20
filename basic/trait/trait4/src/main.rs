// https://doc.rust-jp.rs/book/second-edition/ch19-03-advanced-traits.html
// リスト19-27: 関連関数のあるトレイトとそのトレイトも実装し、同じ名前の関連関数がある型
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        // スポット(Wikipediaによると、飼い主の事故死後もその人の帰りを待つ忠犬の名前の模様)
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        // 子犬
        String::from("puppy")
    }
}

fn main() {
    // 赤ちゃん犬は{}と呼ばれる：結果は「Spot」
    println!("A baby dog is called a {}", Dog::baby_name());

    // これではエラーになる
    // println!("A baby dog is called a {}", Animal::baby_name());
        // Animal::baby_nameはメソッドではなく関連関数であり、故にself引数がないので、
        // どのAnimal::baby_nameが欲しいのか、 コンパイラには推論できません。

    // <Dog as Animal>なら指定できる
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
        // 一般的に、フルパス記法は、以下のように定義されています:
        // <Type as Trait>::function(receiver_if_method, next_arg, ...);

}
