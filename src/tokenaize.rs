pub fn tokenize_text(text: &str) -> Vec<String> {
    let filtered_text: String = text.chars().filter(|&c| c > '\u{0040}').collect();
    if filtered_text.is_empty() {
        return vec!["NULL".to_string()];
    }
    let tokenizer = lindera::tokenizer::Tokenizer::new(
        lindera::core::mode::Mode::Normal,
        lindera::dictionary::DictionaryLoader::load_dictionary_from_config(
            lindera::dictionary::DictionaryConfig {
                kind: Some(lindera::dictionary::DictionaryKind::IPADIC),
                path: None,
            },
        )
        .unwrap(),
        None,
    );
    let tokens = tokenizer.tokenize(&filtered_text).unwrap();
    tokens.iter().map(|token| token.text.to_string()).collect()
}

//名詞取得
pub fn tokenize_word(text: &str) -> Vec<String> {
    // トークンを格納するベクタを作成
    let mut tokens_vec = Vec::new();
    // 0x40以下の文字を削除する
    let filtered_text: String = text
        .chars()
        .filter(|&c| c.is_ascii_alphabetic() || !c.is_ascii())
        .collect();
    // 辞書の設定を行う
    let dictionary_config = lindera::dictionary::DictionaryConfig {
        kind: Some(lindera::dictionary::DictionaryKind::IPADIC),
        path: None,
    };

    let dictionary =
        lindera::dictionary::DictionaryLoader::load_dictionary_from_config(dictionary_config)
            .unwrap();

    // トークナイザーを作成
    let tokenizer =
        lindera::tokenizer::Tokenizer::new(lindera::core::mode::Mode::Normal, dictionary, None);

    // テキストをトークン化する
    let tokens = tokenizer.tokenize(&filtered_text).unwrap();

    // 各トークンに対して接続詞、助詞、句読点かどうかをチェックして、該当しなければ追加する
    for mut token in tokens {
        let details = token.details();
        if details.is_empty() {
            tokens_vec.push(token.text.to_string());
            continue;
        }

        match details[0] {
            "接続詞" | "助詞" | "助動詞" | "記号" => continue,
            "動詞"
                if details[1] != "自立"
                    || (details[5] != "基本形" && !details[4].starts_with("五段")) =>
            {
                continue
            }
            _ => tokens_vec.push(token.text.to_string()),
        }
    }
    // トークンを返す
    tokens_vec
}
