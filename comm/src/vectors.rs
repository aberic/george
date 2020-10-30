/// 变更数组内容
///
/// source 原始数组
///
/// target 变更内容
///
/// start 其实下标
pub fn modify<T: Clone>(mut source: Vec<T>, target: Vec<T>, mut start: usize) -> Vec<T> {
    let len = target.len();
    let mut position = 0;
    while position < len {
        source.remove(start);
        source.insert(start, target.get(position).unwrap().clone());
        start += 1;
        position += 1
    }
    source
}

/// 截取数组
///
/// source 原始数组
///
/// start 截取起始下标
///
/// end 截取终止下标
pub fn sub<T: Clone>(source: Vec<T>, start: usize, end: usize) -> Vec<T> {
    let mut s1 = source.to_vec();
    let mut s2 = s1.split_off(start);
    let _x = s2.split_off(end - start);
    s2
}
