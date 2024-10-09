pub fn tokenize_text(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec!["NULL".to_string()];
    }
    let tokenizer = lindera::tokenizer::Tokenizer::new(
        lindera::core::mode::Mode::Normal,
        lindera::dictionary::DictionaryLoader::load_dictionary_from_config(
            lindera::dictionary::DictionaryConfig {
                kind: Some(lindera::dictionary::DictionaryKind::UniDic),
                path: None,
            },
        )
        .unwrap(),
        None,
    );
    let tokens = tokenizer.tokenize(text).unwrap();
    tokens.iter().map(|token| token.text.to_string()).collect()
}