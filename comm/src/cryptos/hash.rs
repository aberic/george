use crc32fast::Hasher;
use crypto::digest::Digest;
use crypto::md5::Md5;

use crate::strings::sub_string;

pub fn md5(comment: String) -> String {
    let mut md5_handler = Md5::new();
    md5_handler.input_str(comment.as_str());
    md5_handler.result_str()
}

pub fn md516(comment: String) -> String {
    sub_string(md5(comment), 8, 24)
}

pub fn hashcode32(comment: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(comment);
    hasher.finalize()
}

pub fn hashcode32_enhance(comment: String) -> u32 {
    return match comment.parse::<u32>() {
        Ok(su32) => su32,
        Err(_err) => hashcode32(comment.as_bytes()),
    };
}

pub fn hashcode64(comment: &[u8]) -> u64 {
    let mut c = crc64fast::Digest::new();
    c.write(comment);
    c.sum64()
}

pub fn hashcode64_enhance(comment: String) -> u64 {
    return match comment.parse::<u64>() {
        Ok(su64) => su64,
        Err(_err) => hashcode64(comment.as_bytes()),
    };
}

pub fn hashcode_enhance(u32: bool, comment: String) -> (u32, u64) {
    if u32 {
        (hashcode32_enhance(comment), 0)
    } else {
        (0, hashcode64_enhance(comment))
    }
}
