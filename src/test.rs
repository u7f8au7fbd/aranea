mod mods;
use charming::component::title;
use mods::extract;
use mods::measures_items;
use mods::statistics;
use mods::statistics::*;
use mods::strfn;
use mods::tokenaize;
use mods::traits::Formatter;
use mods::vec_man;

const FILE_PATH: &str = "./db/100.html";
const FIRST_QUERY: &str = "大阪";
const SECOND_QUERY: &str = "転職";

fn main() {
    let title = extract::extract_element("title", FILE_PATH);
    let h1 = extract::extract_element("h1", FILE_PATH);
    let h2 = extract::extract_element("h2", FILE_PATH);
    let h3 = extract::extract_element("h3", FILE_PATH);
    let p = strfn::vec_format(extract::extract_element("p", FILE_PATH));

    let html_vecs = [title, h1, h2, h3, p];
    let html_vecs = vec_man::gether(html_vecs.to_vec());

    let mut tokens = Vec::new();
    for vec in html_vecs {
        let tokenized = tokenaize::tokenize_word(&vec);
        tokens.extend(tokenized);
    }
    println!("{:#?}", tokens);
    println!("{:?}", statistics::word_distribution(tokens));
}
