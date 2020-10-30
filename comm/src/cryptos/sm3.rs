use libsm::sm3::hash::Sm3Hash;

/// 国密消息摘要。可以用MD5作为对比理解。该算法已公开。校验结果为256位
pub(crate) fn hash(comment: String) -> String {
    let string = String::from(comment);
    let mut hash = Sm3Hash::new(string.as_bytes());
    let digest: [u8; 32] = hash.get_hash();
    // println!("digest = {:#?}", digest);
    hex::encode(digest)
}
