// RustでtomlファイルをstructにDeserialzeする
// https://qiita.com/sunjin110/items/c9dfdd12f433ccaf5270

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mongo_db: MongoDb,
}

// 適当なsturctなのでごめんなさい
#[derive(Debug, Deserialize)]
pub struct MongoDb {
    pub name: String,
    pub age: i32,
}

fn main() -> io::Result<()> {
    let config = deserial_toml_file::<Config>("./config/local.toml")?;
    println!("config is {:?}", config);
    println!("config is {:?}", config.mongo_db);
    Ok(())
}

// deserial_toml_file tomlのfileをstructにdeserializeする
pub fn deserial_toml_file<T>(path: &str) -> io::Result<T>
where
    T: DeserializeOwned,
// pub fn deserial_toml_file<T>(path: &str, file_str: & mut String) -> Result<T>
// where
//     T: for<'a> Deserialize<'a>
{
    let file_str = fs::read_to_string(path)?;
    let obj = toml::from_str::<T>(&file_str)?;
    Ok(obj)
}
