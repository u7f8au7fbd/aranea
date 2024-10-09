use scraper::{Html, Selector};
use std::fs::read_to_string;

fn main() {
    // HTMLファイルを読み込むのじゃ
    let html_content = read_to_string("./db/0.html").expect("ファイルが読み込めなかった");

    // HTMLパース用のドキュメントを作成するのじゃ
    let document = Html::parse_document(&html_content);

    // タイトルのセレクタを作成するのじゃ
    let title_selector = Selector::parse("title").unwrap();

    // タイトルを抽出して表示するのじゃ
    if let Some(title_element) = document.select(&title_selector).next() {
        let title = title_element.text().collect::<Vec<_>>().concat();
        println!("タイトル: {}", title);
    } else {
        println!("タイトルが見つからなかった");
    }
}
