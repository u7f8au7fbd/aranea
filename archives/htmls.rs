mod get_html;

use scraper::{ElementRef, Html, Selector};

#[macro_use]
mod macros;

#[tokio::main]
async fn main() {
    cmd!(clear);
    // ファイルの内容を読み取る
    let body = reqwest::get("https://www.rust-lang.org/ja")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    //println!("{}", mold_html!(&body));

    // HTMLドキュメントを解析する
    let document = Html::parse_document(&body);

    // 対象とするタグのセレクタを作成する
    let selectors = vec![
        ("h1", Selector::parse("h1").unwrap()),
        ("h2", Selector::parse("h2").unwrap()),
        ("h3", Selector::parse("h3").unwrap()),
        ("p", Selector::parse("p").unwrap()),
        ("a", Selector::parse("a").unwrap()),
    ];

    // 要素を保持するためのベクタを初期化する
    let mut elements: Vec<(String, String)> = Vec::new();

    // ノードのツリー構造を利用して全ての要素を順番に走査
    for node in document.tree.nodes() {
        if let Some(element) = ElementRef::wrap(node) {
            for (tag_name, _) in &selectors {
                if element.value().name() == *tag_name {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        elements.push((tag_name.to_string(), text));
                    }
                }
            }
        }
    }

    let mut objects = Tag {
        p: vec![],
        h1: vec![],
        h2: vec![],
        h3: vec![],
        a: vec![],
    };

    // 取得した要素を表示する
    for (tag, text) in elements {
        if tag == "p" {
            println!("{}:{}", tag, text);
            objects.p.push(text);
        } else if tag == "h1" {
            println!("{}:{}", yellow!(tag), yellow!(text));
            objects.h1.push(text);
        } else if tag == "h2" {
            println!("{}:{}", cyan!(tag), cyan!(text));
            objects.h2.push(text);
        } else if tag == "h3" {
            println!("{}:{}", magenta!(tag), magenta!(text));
            objects.h3.push(text);
        } else if tag == "a" {
            println!("{}:{}", blue!(tag), blue!(text));
            objects.a.push(text);
        }
    }

    println!("{:#?}", objects);
}

#[derive(Debug)]
struct Tag {
    p: Vec<String>,
    h1: Vec<String>,
    h2: Vec<String>,
    h3: Vec<String>,
    a: Vec<String>,
}
