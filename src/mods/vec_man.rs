pub fn gether(vecs: Vec<Vec<String>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for vec in vecs {
        for item in vec {
            result.push(item);
        }
    }
    result
}
