use mods::tokenaize::*;
mod mods;
fn main() {
    let text = "私は、日本語の形態素解析を行うライブラリLinderaを使ってみた。";
    let tokens = tokenize_word(text);
    println!("{:?}", tokens);
}
