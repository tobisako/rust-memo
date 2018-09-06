extern crate rand;

use std::io;
use std::cmp::Ordering; // これは enum 定義だよ。
use rand::Rng;  // これで、.gen_range() が使える。

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101); // 上限は含むが下限は含まない。

  println!("The secret number is: {}", secret_number);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    //let guess:u32 = guess.trim().parse()  // トリミング.文字を数字に変換。
    //  .expect("Please type a number!");
    let guess:u32 = match guess.trim().parse() {  // トリミング.文字を数字に変換。
      Ok(num) => num,
      Err(_) => continue,   // loopの最初に戻る。
    };

    println!("You gessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less    => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal   => {
        println!("You win!");
        break;
      }
    }
  }
}

