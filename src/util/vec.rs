pub fn take_last<T: std::clone::Clone>(source: Vec<T>, len: usize) -> Vec<T> {
    return source.as_slice()[source.len() - len..].to_vec();
}