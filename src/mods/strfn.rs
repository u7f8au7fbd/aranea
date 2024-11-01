use crate::mods::traits;

pub fn contains_word(word: &str, sentence: &str) -> bool {
    sentence.contains(word)
}

pub fn vec_format(vecs: Vec<String>) -> Vec<String> {
    vecs.iter().map(traits::Formatter::format_html).collect()
}
