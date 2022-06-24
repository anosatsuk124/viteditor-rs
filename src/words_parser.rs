pub fn parser(str: &str) -> Vec<usize> { 
    // TODO: implement unicode clusters
    str.split_whitespace().map(|w| w.len()).collect()
}
