use crate::ge::Metadata;
use crate::utils::Filed;

mod description;
mod digest;
mod ge;
mod ge_test;
mod header;
mod metadata;
mod metadata_test;
pub mod utils;

/// 文件元数据信息，长度52字节
pub const METADATA_SIZE: u64 = 52;

/// 当前文件版本号(2字节)
pub const VERSION: [u8; 2] = [0x00, 0x00];
/// 文件版本1(2字节)
pub const VERSION_1: [u8; 2] = [0x00, 0x00];
/// 起始符(2字节)
const FRONT: [u8; 2] = [0x20, 0x19];
/// 截止符(2字节)
const END: [u8; 2] = [0x02, 0x19];

/// `ge`文件对象
#[derive(Clone, Debug)]
pub struct Ge {
    /// `ge`文件地址
    filepath: String,
    /// 文件元数据信息，长度52字节
    metadata: Metadata,
    /// 根据文件路径获取该文件追加写入的写对象
    ///
    /// 需要借助对象包裹，以便更新file，避免self为mut
    filed: Filed,
}
