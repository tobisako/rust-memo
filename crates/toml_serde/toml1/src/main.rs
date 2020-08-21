// https://smile-jsp.hateblo.jp/entry/2020/04/20/170000
// https://cipepser.hatenablog.com/entry/rust-toml
// https://qiita.com/sunjin110/items/c9dfdd12f433ccaf5270

#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::path::Path;
use std::fs;
use std::io::{BufReader, Read};

#[derive(Debug, Deserialize)]
struct MyConfig {
    meta: Option<Meta>,
    metajp: Option<MetaJp>,
}

#[derive(Debug, Deserialize)]
struct Meta {
    foo: Option<String>,
    bar: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MetaJp {
    hoge: Option<i32>,
    piyo: Option<i32>,
    huga: Option<i32>,
}

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

fn main() {
    let s = match read_file("./config.toml".to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    let mut CONF: Result<MyConfig, toml::de::Error> = toml::from_str(&s);

    // 内容確認
    match CONF {
        Ok(p) => println!("{:#?}", p),
        Err(e) => panic!("fail to parse toml: {}", e),
    };

    // 使い方
    //println!("[metajp] hoge={}", &CONF.unwrap().metajp.unwrap().hoge.unwrap());
    //println!("[metajp] piyo={}", &CONF.unwrap().metajp.unwrap().piyo.unwrap());
}

// let cur_path_str = env::current_exe().unwrap().clone();
// let cur_path = Path::new(&cur_path_str);
// let cur_dir = cur_path.parent().unwrap().display();
// println!("{}", cur_dir);
// println!("Hello, world!");
