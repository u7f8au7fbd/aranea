fn extract_tag_content(file_path: &str, tag: &str) -> Vec<String> {
    let html_content = std::fs::read_to_string(file_path).expect("Failed to read file");
    let document = scraper::Html::parse_document(&html_content);
    let selector = scraper::Selector::parse(tag).expect("Failed to parse selector");

    document
        .select(&selector)
        .map(|element| remove_whitespace(&element.inner_html()))
        .collect()
}

pub fn extract_title(file_path: &str) -> String {
    extract_tag_content(file_path, "title")
        .into_iter()
        .next()
        .unwrap_or_default()
}

pub fn extract_h1(file_path: &str) -> String {
    extract_tag_content(file_path, "h1").join("")
}

pub fn extract_h2(file_path: &str) -> Vec<String> {
    extract_tag_content(file_path, "h2")
}

fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}
