pub fn str_decimals(input: &str) -> Vec<i32> {
    input.chars().map(|c| c as i32).collect()
}