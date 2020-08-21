// tomlと混ぜてlazyで処理してみるテスト
// https://qiita.com/sunjin110/items/c9dfdd12f433ccaf5270

// config.toml を、static グローバル変数で読み込んで使う。

use serde::de::DeserializeOwned;
//use serde::Deserialize;
use serde_derive::{Deserialize};
use std::fs;
use std::io;
use toml;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Deserialize)]
struct MyConfig {
    pub meta: Meta,
    pub metajp: MetaJp,
}

#[derive(Debug, Deserialize)]
struct Meta {
    pub foo: String,
    pub bar: String,
}

#[derive(Debug, Deserialize)]
struct MetaJp {
    pub hoge: i32,
    pub piyo: i32,
    pub huga: i32,
}

lazy_static! {
    static ref CONF: MyConfig = {
        println!("[lazy_static!]LOADING");
        deserial_toml_file::<MyConfig>("./config.toml").unwrap()
    };
}

fn main() {
    // 1 ローカルで読み込むテスト
    let config: MyConfig = deserial_toml_file::<MyConfig>("./config.toml").unwrap();
    println!("----------");
    println!("config {:?}", config);
    println!("config meta.foo is {:?}", config.meta.foo);
    println!("config metajp.piyo is {:?}", config.metajp.piyo);

    // 2 グローバル・スタティック変数で読込テスト
    println!("----------");
    println!("CONF meta.foo is {:?}", CONF.meta.foo);
    println!("CONF metajp.piyo is {:?}", CONF.metajp.piyo);
    func1();
}

fn func1() {
    // CONF だけはグローバルでいいんちゃう？
    println!("CONF metajp.hoge is {:?}", CONF.metajp.hoge);
}

// deserial_toml_file tomlのfileをstructにdeserialize
pub fn deserial_toml_file<T>(path: &str) -> io::Result<T>
where
    T: DeserializeOwned,
{
    let file_str = fs::read_to_string(path)?;
    let obj = toml::from_str::<T>(&file_str)?;
    Ok(obj)
}
