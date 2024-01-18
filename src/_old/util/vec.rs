// pub fn take_last<T: std::clone::Clone>(source: &Vec<T>, len: usize) -> Vec<T> {
//     let mut res = vec![];
//     for i in 0..len {
//         res.push(source[(source.len() - 1) - i].clone())
//     }
//     return  res;
// }

pub fn reverse_vec<T>(source: &Vec<T>) -> Vec<&T> {
    let mut result: Vec<&T> = vec![];
    for i in source.len()..0 {
        result.push(source[i]);
    }

    return result;
}

pub fn difference(source: &Vec<f64>, lag: usize) -> Vec<f64> {
    if lag == 0 {
        return source.clone();
    } else {
        let mut result: Vec<f64> = vec![];
        for i in lag..source.len() {
            result.push(source[i] - source[i - lag]);
        }

        return result;
    }
}
