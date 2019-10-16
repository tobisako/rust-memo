fn main() {
  // クロージャ
    // しばしば、
    // 関数と 自由変数 を一つにまとめておくことがコードの明確さや再利用に役立つ場合が有ります。 
    // 自由変数は外部のスコープから来て、関数中で使われるときに「閉じ込め」られます。 
    // そのためそのようなまとまりを「クロージャ」と呼び、 
    // Rustはこれから見ていくようにクロージャの非常に良い実装を提供しています。
  // 構文
  let plus_one = |x: i32| x + 1;
  assert_eq!(2, plus_one(1));

    // 複数行のクロージャを作成することも可能です:
  let plus_two = |x| {
    let mut result: i32 = x;
    result += 1;
    result += 1;
    result
  };
  assert_eq!(4, plus_two(2));

    // いくつかクロージャと通常の fn で定義される関数との間の違いに気がつくことでしょう。 
    // 一つ目はクロージャの引数や返り値の型を示す必要が無いことです。 
    // 型を以下のように示すことも可能です:
  let plus_one = |x: i32| -> i32 { x + 1 };
  assert_eq!(2, plus_one(1));

    // 通常の関数との違いの二つ目は、構文が大部分は似ていますがほんの少しだけ違うという点です。 
    // 比較がしやすいようにスペースを適宜補って以下に示します:
  fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
  let plus_one_v2 = |x: i32| -> i32 { x + 1 };
  let plus_one_v3 = |x: i32|          x + 1  ;

  // クロージャとクロージャの環境
  let num = 5;
  let plus_num = |x: i32| x + num;
  assert_eq!(10, plus_num(5));

    // 以下はエラー
  // let mut num = 5;
  // let plus_num = |x: i32| x + num;
  // let y = &mut num;

    // もしクロージャがスコープ外になるようにした場合以下のようにできます:
  let mut num = 5;
  {
    let plus_num = |x: i32| x + num;
  } // plus_numがスコープ外に出て、numの借用が終わります
  let y = &mut num;

    // 以下はエラー
  // let nums = vec![1, 2, 3];
  // let takes_nums = || nums;
  // println!("{:?}", nums);

  // move クロージャ
  let num = 5;
  let owns_num = move |x: i32| x + num;

  let mut num = 5;
  {
    let mut add_num = |x: i32| num += x;
    add_num(5);
  }
  assert_eq!(10, num);
    // このケースでは、クロージャは num の変更可能な参照を取得し、 
    // add_num を呼び出した時、期待通りに num の値を変更します。 
    // またクロージャ add_num はその環境を変更するため mut として宣言する必要があります。

    // もしクロージャを move に変更した場合、結果が異なります:
  let mut num = 5;
  {
    let mut add_num = move |x: i32| num += x;
    add_num(5);
  }
  assert_eq!(5, num);
    // move クロージャを捉えるもう一つの観点は:
    // move クロージャは独自のスタックフレームを持っているという点です。 
    // move クロージャは自己従属していますが、 
    // move でないクロージャはクロージャを作成したスタックフレームと紐付いています。 
    // これは一般的に、move でないクロージャを関数から返すことはできないということを意味しています。

  // クロージャの実装
    // Rustにおけるクロージャの実装は他の言語とは少し異なります。 
    // Rustにおけるクロージャは実質的にトレイトへの糖衣構文です。
  // mod foo {
  //   pub trait Fn<Args> : FnMut<Args> {
  //     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
  //   }
  //   pub trait FnMut<Args> : FnOnce<Args> {
  //     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
  //   }
  //   pub trait FnOnce<Args> {
  //     type Output;
  //     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
  //   }
  // }

  // クロージャを引数に取る
    // クロージャが実際にはトレイトであることを学んだので、 
    // クロージャを引数としたり返り値としたりする方法を既に知っていることになります: 
    // 通常のトレイトと同様に行うのです!
    // これは、静的ディスパッチと動的ディスパッチを選択することができるということも意味しています。 
    // 手始めに呼び出し可能な何かを引数にとり、それを呼び出し、結果を返す関数を書いてみましょう:
  fn call_with_one<F>(some_closure: F) -> i32
    where F : Fn(i32) -> i32 {
    some_closure(1)
  }
  let answer = call_with_one(|x| x + 2);
  assert_eq!(3, answer);

    // call_with_one のシグネチャを詳細に見ていきましょう:
  fn call_with_one2<F>(some_closure2: F) -> i32
  where F : Fn(i32) -> i32 { some_closure2(1) }
    // Fn がトレイトであるために、ジェネリックの境界として Fn を指定することができます。 
    // この場合はクロージャは i32 を引数として取り、 i32 を返します、
    // そのため ジェネリックの境界として Fn(i32) -> i32 を指定します。

    // もちろん、動的ディスパッチを行いたいときは、そうすることもできます。 
    // そのような場合もトレイトオブジェクトが通常どおりに対応します:
  fn call_with_one3(some_closure3: &Fn(i32) -> i32) -> i32 {
    some_closure3(1)
  }
  let answer3 = call_with_one3(&|x| x + 2);
  assert_eq!(3, answer3);

  main2();
}

fn main2() {
  // 関数ポインタとクロージャ
  fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
  }
  fn add_one(i: i32) -> i32 {
    i + 1
  }
  let f = add_one;
  let answer = call_with_one(&f);
  assert_eq!(2, answer);
    // この例では、中間の変数 f が必ずしも必要なわけではありません、
    // 関数名を指定することでもきちんと動作します:
  let answer2 = call_with_one(&add_one);

  // クロージャを返す
    // 関数を用いたスタイルのコードでは、クロージャを返すことは非常によく見られます。 
    // もし、クロージャを返すことを試みた場合、エラーが発生します。
  // fn factory() -> (Fn(i32) -> i32) {
  //   let num = 5;
  //   |x| x + num
  // }
  // let f = factory();
  // let answer = f(1);
  // assert_eq!(6, answer);
    // このコードは以下の長いエラーを発生させます:

    // これもエラー
  // fn factory() -> &(Fn(i32) -> i32) {
  //   let num = 5;
  //   |x| x + num
  // }
  // let f = factory();
  // let answer = f(1);
  // assert_eq!(6, answer);

    // これもエラー。
  // fn factory() -> &'static (Fn(i32) -> i32) {
  //   let num = 5;
  //   |x| x + num
  // }
  // let f = factory();
  // let answer = f(1);
  // assert_eq!(6, answer);

    // これもまだエラー。
  // fn factory() -> Box<Fn(i32) -> i32> {
  //   let num = 5;
  //   Box::new(|x| x + num)
  // }
  // let f = factory();
  // let answer = f(1);
  // assert_eq!(6, answer);

    // これでＯＫ！
  fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
  }
  let f = factory();
  let answer = f(1);
  assert_eq!(6, answer);
}
