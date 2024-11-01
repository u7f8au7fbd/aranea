use std::collections::HashMap;

pub fn word_distribution(strings: Vec<String>) -> Vec<(String, usize)> {
    let mut counts = HashMap::new();
    for word in strings {
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}
