pub fn tokenize_source_string(source_string: &str) -> Vec<String> {
    source_string
        .split(" ")
        .map(str::to_owned)
        .collect::<Vec<String>>()
}
