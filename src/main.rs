mod tokenaize;
fn main() {
    let text = "今日はいい天気ですね";
    let tokens = tokenaize::tokenize_text(text);
    for token in tokens {
        println!("{}", token);
    }
}
