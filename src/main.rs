#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use jieba_rs::Jieba;
use rocket::http::RawStr;
use rocket::State;

struct MyConfig {
    jieba: Jieba
}

// test for example with http://localhost:8000/我们中出了一个叛徒
#[get("/<text>")]
fn index(state: State<MyConfig>, text: &RawStr) -> String {
    let text = match text.percent_decode(){
        Ok(decoded) => decoded,
        Err(_) => return String::from("Can't decode string")
    };
    let words = state.jieba.cut(text.as_ref(), false);
    String::from(words.join(","))
}

fn main() {
    let config = MyConfig{
        jieba: Jieba::new()
    };

    rocket::ignite().
        mount("/", routes![index]).
        manage(config).
        launch();
}
