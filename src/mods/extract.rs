use crate::mods::traits::Formatter;
fn extract_tag_content(file_path: &str, tag: &str) -> Vec<String> {
    let html_content = std::fs::read_to_string(file_path).expect("Failed to read file");
    let document = scraper::Html::parse_document(&html_content);
    let selector = scraper::Selector::parse(tag).expect("Failed to parse selector");

    document
        .select(&selector)
        .map(|element| {
            let content = Formatter::replace_ws(&element.inner_html());
            Formatter::format_ws(&content)
        })
        .collect()
}

pub fn extract_element(element: &str, file_path: &str) -> Vec<String> {
    extract_tag_content(file_path, element)
}
