use std::cmp::PartialOrd;

pub fn min<T>(x: T, y: T) -> T
where T: PartialOrd {
    if x <= y { x } else { y }
}

pub fn zip<T: Copy, K: Copy>(a: Vec<T>, b: Vec<K>) -> Vec<(T, K)> {
    let mut v = vec![];
    let len = min(a.len(), b.len());

    for i in 0..len {
        v.push((a[i], b[i]));
    }

    v
}
