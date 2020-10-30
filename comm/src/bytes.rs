/// 创建长度为len且字节均为0x00的字节数组
pub fn create_empty_bytes(len: usize) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let mut position = 0;
    while position < len {
        res.push(0x00);
        position += 1
    }
    res
}
