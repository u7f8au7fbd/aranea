use mods::lazy_str::{unique_strings_with_count, VecToStr};
use scraper::{Html, Selector};
use std::fs;
mod mods;

fn main() {
    let path = format!("./db/{}.html", 1);
    let links = get_links(&path);
    let links_str: Vec<&str> = links.vec_to_str();
    println!("{:?}", unique_strings_with_count(links_str, 5));
}

fn get_links(path: &str) -> Vec<String> {
    let mut links = Vec::new();
    if let Ok(html_content) = fs::read_to_string(path) {
        let document = Html::parse_document(&html_content);
        let selector = Selector::parse("a").unwrap();
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if href.starts_with("http") {
                    links.push(href.to_string());
                }
            }
        }
    }
    links
}
