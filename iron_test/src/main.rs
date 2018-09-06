extern crate iron;
extern crate router;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
  let mut router = Router::new();
  router.get("/", hello, "helloworld");
  router.get("/hoge", fuga, "piyo");
  router.post("/", h_prefix, "h_prefix");

  Iron::new(router).http("localhost:3000").unwrap();

  fn hello(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello IRON route World!")))
  }

  fn fuga(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello IRON route HOGE World!")))
  }

  fn h_prefix(req: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    req.body.read_to_string(&mut body)
      .expect("Failed to read line!");

    let res = "Hello ".to_string() + &body + &"!".to_string();
    Ok(Response::with((status::Ok, res)))
  }

  println!("Hello, world!");
  // Iron::new(|_: &mut Request| {
  //   Ok(Response::with((status::Ok, "Hello Iron World!")))
  // }).http("localhost:3000").unwrap();
}

// sample 参考ＵＲＬ
//https://qiita.com/shamisonn/items/24fe203ca4fd610e4a25
