/// 索引
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 存储引擎类型
    #[prost(enumeration = "Engine", tag = "2")]
    pub engine: i32,
    /// 是否主键，主键也是唯一索引，即默认列表依赖索引
    #[prost(bool, tag = "3")]
    pub primary: bool,
    /// 是否唯一索引
    #[prost(bool, tag = "4")]
    pub unique: bool,
    /// 是否允许为空
    #[prost(bool, tag = "5")]
    pub null: bool,
    /// 索引值类型
    #[prost(enumeration = "KeyType", tag = "6")]
    pub key_type: i32,
    /// 创建时间
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<super::utils::Timestamp>,
}
/// 索引集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseIndexList {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub indexes: ::prost::alloc::vec::Vec<Index>,
}
/// 请求索引集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestIndexList {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
}
/// 请求新建视图
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestIndexCreate {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// 存储引擎类型
    #[prost(enumeration = "Engine", tag = "4")]
    pub engine: i32,
    /// 是否主键，主键也是唯一索引，即默认列表依赖索引
    #[prost(bool, tag = "5")]
    pub primary: bool,
    /// 是否唯一索引
    #[prost(bool, tag = "6")]
    pub unique: bool,
    /// 是否允许为空
    #[prost(bool, tag = "7")]
    pub null: bool,
    /// 索引值类型
    #[prost(enumeration = "KeyType", tag = "8")]
    pub key_type: i32,
}
/// 请求视图信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestIndexInfo {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// 请求视图信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseIndexInfo {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub index: ::core::option::Option<Index>,
}
/// 存储引擎类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Engine {
    /// 占位
    None = 0,
    /// 卷宗存储引擎(单文件索引存储-64位)，最合适用于自增
    Increment = 1,
    /// 卷宗存储引擎(单文件索引存储-64位)，最合适用于不重复u64
    Sequence = 2,
    /// 卷宗存储引擎(单文件索引存储-32位)
    Disk = 3,
    /// 块存储引擎(区块链索引存储-32位)
    Block = 4,
}
//// 索引值类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    /// 字符串索引
    String = 0,
    /// 无符号64位整型
    UInt = 1,
    /// 有符号64位整型
    Int = 2,
    /// 有符号64位浮点类型
    Float = 3,
    /// bool类型
    Bool = 4,
    /// 不支持类型
    Nonsupport = 5,
}
/// 视图
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct View {
    /// 名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    /// 创建时间
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<super::utils::Timestamp>,
    /// 索引集合
    #[prost(message, repeated, tag = "4")]
    pub indexes: ::prost::alloc::vec::Vec<Index>,
    /// 文件地址
    #[prost(string, tag = "5")]
    pub filepath: ::prost::alloc::string::String,
    /// 版本号
    #[prost(uint32, tag = "6")]
    pub version: u32,
}
/// 视图集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseViewList {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub views: ::prost::alloc::vec::Vec<View>,
}
/// 请求视图集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestViewList {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
}
/// 请求新建视图
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestViewCreate {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
    /// 是否带自增ID
    #[prost(bool, tag = "4")]
    pub with_increment: bool,
}
/// 请求变更视图
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestViewModify {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 新名称
    #[prost(string, tag = "3")]
    pub name_new: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "4")]
    pub comment: ::prost::alloc::string::String,
}
/// 请求视图信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestViewInfo {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// 请求视图信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseViewInfo {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub view: ::core::option::Option<View>,
}
/// 请求视图信息删除
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestViewRemove {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// 整理归档
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestViewArchive {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 归档路径
    #[prost(string, tag = "3")]
    pub archive_file_path: ::prost::alloc::string::String,
}
/// 读取指定归档版本信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestViewRecord {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 版本号
    #[prost(uint32, tag = "3")]
    pub version: u32,
}
/// 读取指定归档版本信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseViewRecord {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    /// 归档版本信息
    #[prost(message, optional, tag = "3")]
    pub record: ::core::option::Option<ViewRecord>,
}
/// 读取指定归档版本信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestViewRecords {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// 读取指定归档版本信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseViewRecords {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    /// 归档版本信息
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<ViewRecord>,
}
/// 指定归档版本信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewRecord {
    /// 当前归档版本文件所处路径
    #[prost(string, tag = "1")]
    pub filepath: ::prost::alloc::string::String,
    /// 归档时间
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<super::utils::Timestamp>,
    /// 版本号
    #[prost(uint32, tag = "3")]
    pub version: u32,
}
/// 数据库
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// 名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    /// 创建时间
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<super::utils::Timestamp>,
    /// 视图集合
    #[prost(message, repeated, tag = "4")]
    pub views: ::prost::alloc::vec::Vec<View>,
}
/// 数据库集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDatabaseList {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub databases: ::prost::alloc::vec::Vec<Database>,
}
/// 请求新建数据库
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDatabaseCreate {
    /// 名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
}
/// 请求变更数据库
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDatabaseModify {
    /// 名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 新名称
    #[prost(string, tag = "2")]
    pub name_new: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "3")]
    pub comment: ::prost::alloc::string::String,
}
/// 请求数据库信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDatabaseInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// 请求数据库信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDatabaseInfo {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub database: ::core::option::Option<Database>,
}
/// 请求数据库信息删除
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDatabaseRemove {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// 请求插入数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDiskInto {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// 请求获取数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDiskOut {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
/// 请求获取数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDiskIOut {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
    /// 索引名称
    #[prost(string, tag = "3")]
    pub index_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
}
/// 返回获取数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDiskOut {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// 请求删除数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDiskRemove {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
/// 请求查询数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDiskSelect {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
    /// 选择器字节数组，自定义转换策略
    #[prost(bytes = "vec", tag = "3")]
    pub constraint_json_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// 返回查询数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDiskSelect {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    /// 查询数据
    #[prost(message, optional, tag = "3")]
    pub selected: ::core::option::Option<DiskSelected>,
}
/// 返回查询数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskSelected {
    /// total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    #[prost(uint64, tag = "1")]
    pub total: u64,
    /// 检索结果过程中遍历的总条数
    #[prost(uint64, tag = "2")]
    pub count: u64,
    ///  使用到的索引名称，如果没用上则为空
    #[prost(string, tag = "3")]
    pub index_name: ::prost::alloc::string::String,
    /// 索引是否顺序
    #[prost(bool, tag = "4")]
    pub asc: bool,
    /// values 检索结果集合
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 请求删除数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDiskDelete {
    /// 数据库名称
    #[prost(string, tag = "1")]
    pub database_name: ::prost::alloc::string::String,
    /// 视图名称
    #[prost(string, tag = "2")]
    pub view_name: ::prost::alloc::string::String,
    /// 选择器字节数组，自定义转换策略
    #[prost(bytes = "vec", tag = "3")]
    pub constraint_json_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// 返回查询数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDiskDelete {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    /// 删除数据
    #[prost(message, optional, tag = "3")]
    pub deleted: ::core::option::Option<DiskDeleted>,
}
/// 返回删除数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskDeleted {
    /// total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    #[prost(uint64, tag = "1")]
    pub total: u64,
    /// 检索结果过程中遍历的总条数
    #[prost(uint64, tag = "2")]
    pub count: u64,
    ///  使用到的索引名称，如果没用上则为空
    #[prost(string, tag = "3")]
    pub index_name: ::prost::alloc::string::String,
    /// 索引是否顺序
    #[prost(bool, tag = "4")]
    pub asc: bool,
    /// values 检索结果集合
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 缓存页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// 名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    /// 可使用内存大小(单位：Mb，0：不限制大小)
    #[prost(uint64, tag = "3")]
    pub size: u64,
    /// 默认有效期(单位：秒)，如无设置，默认维300(0：永久有效)
    #[prost(uint32, tag = "4")]
    pub period: u32,
    /// 创建时间
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<super::utils::Timestamp>,
}
/// 缓存页集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePageList {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
}
/// 请求新建缓存页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPageCreate {
    /// 名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 描述
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    /// 可使用内存大小(单位：Mb，0：不限制大小)
    #[prost(uint64, tag = "3")]
    pub size: u64,
    /// 默认有效期(单位：秒)，如无设置，默认维300(0：永久有效)
    #[prost(uint32, tag = "4")]
    pub period: u32,
}
/// 请求变更缓存页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPageModify {
    /// 名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 新名称
    #[prost(string, tag = "2")]
    pub name_new: ::prost::alloc::string::String,
}
/// 请求缓存页信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPageInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// 请求缓存页信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePageInfo {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub page: ::core::option::Option<Page>,
}
/// 请求缓存页信息删除
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPageRemove {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// 主管员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Master {
    /// 默认缓存页名称
    #[prost(string, tag = "1")]
    pub default_page_name: ::prost::alloc::string::String,
    /// 缓存页集合
    #[prost(message, repeated, tag = "2")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
    /// 库集合
    #[prost(message, repeated, tag = "3")]
    pub databases: ::prost::alloc::vec::Vec<Database>,
    /// 创建时间
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<super::utils::Timestamp>,
}
/// 请求插入数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMemoryInto {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// 请求获取数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMemoryOut {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// 返回获取数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMemoryOut {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// 请求删除数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMemoryRemove {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// 请求插入数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMemoryPInto {
    /// 缓存页名称
    #[prost(string, tag = "1")]
    pub page_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// 请求获取数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMemoryPOut {
    /// 缓存页名称
    #[prost(string, tag = "1")]
    pub page_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// 返回获取数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMemoryPOut {
    #[prost(enumeration = "super::utils::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// 请求删除数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMemoryPRemove {
    /// 缓存页名称
    #[prost(string, tag = "1")]
    pub page_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// 请求登录数据库
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLogin {
    /// 名称
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 密码
    #[prost(string, tag = "2")]
    pub pass: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod user_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct UserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UserServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " 数据库用户登录"]
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestLogin>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.UserService/login");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UserServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UserServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UserServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod page_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct PageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PageServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PageServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " 缓存页集合"]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::super::utils::Req>,
        ) -> Result<tonic::Response<super::ResponsePageList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.PageService/list");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 创建缓存页"]
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestPageCreate>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.PageService/create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 修改缓存页"]
        pub async fn modify(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestPageModify>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.PageService/modify");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取缓存页详情"]
        pub async fn info(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestPageInfo>,
        ) -> Result<tonic::Response<super::ResponsePageInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.PageService/info");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除缓存页"]
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestPageRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.PageService/remove");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PageServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PageServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PageServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod database_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct DatabaseServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatabaseServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatabaseServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " 数据库集合"]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::super::utils::Req>,
        ) -> Result<tonic::Response<super::ResponseDatabaseList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DatabaseService/list");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 创建数据库"]
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDatabaseCreate>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DatabaseService/create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 修改数据库"]
        pub async fn modify(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDatabaseModify>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DatabaseService/modify");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取数据库详情"]
        pub async fn info(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDatabaseInfo>,
        ) -> Result<tonic::Response<super::ResponseDatabaseInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DatabaseService/info");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除数据库"]
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDatabaseRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DatabaseService/remove");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DatabaseServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DatabaseServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DatabaseServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ViewServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ViewServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " 视图集合"]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestViewList>,
        ) -> Result<tonic::Response<super::ResponseViewList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.ViewService/list");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 创建视图"]
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestViewCreate>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.ViewService/create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 修改视图"]
        pub async fn modify(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestViewModify>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.ViewService/modify");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取视图详情"]
        pub async fn info(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestViewInfo>,
        ) -> Result<tonic::Response<super::ResponseViewInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.ViewService/info");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除视图"]
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestViewRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.ViewService/remove");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 整理归档"]
        pub async fn archive(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestViewArchive>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.ViewService/archive");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 读取指定归档版本信息"]
        pub async fn record(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestViewRecord>,
        ) -> Result<tonic::Response<super::ResponseViewRecord>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.ViewService/record");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 读取所有归档版本信息"]
        pub async fn records(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestViewRecords>,
        ) -> Result<tonic::Response<super::ResponseViewRecords>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.ViewService/records");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ViewServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod index_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct IndexServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IndexServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IndexServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " 索引集合"]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestIndexList>,
        ) -> Result<tonic::Response<super::ResponseIndexList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.IndexService/list");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 创建索引"]
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestIndexCreate>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.IndexService/create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取索引详情"]
        pub async fn info(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestIndexInfo>,
        ) -> Result<tonic::Response<super::ResponseIndexInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.IndexService/info");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for IndexServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for IndexServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "IndexServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod disk_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct DiskServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DiskServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DiskServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " 插入数据，如果存在则返回已存在"]
        pub async fn put(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDiskInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DiskService/put");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 插入数据，无论存在与否都会插入或更新数据"]
        pub async fn set(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDiskInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DiskService/set");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取数据，返回存储对象"]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDiskOut>,
        ) -> Result<tonic::Response<super::ResponseDiskOut>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DiskService/get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 根据指定索引名称获取数据，返回存储对象"]
        pub async fn get_by_index(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDiskIOut>,
        ) -> Result<tonic::Response<super::ResponseDiskOut>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DiskService/get_by_index");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除数据"]
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDiskRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DiskService/remove");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 条件检索"]
        pub async fn select(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDiskSelect>,
        ) -> Result<tonic::Response<super::ResponseDiskSelect>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DiskService/select");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 条件删除"]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDiskDelete>,
        ) -> Result<tonic::Response<super::ResponseDiskDelete>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.DiskService/delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DiskServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DiskServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DiskServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod memory_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct MemoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MemoryServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MemoryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " 插入数据，如果存在则返回已存在"]
        pub async fn put(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestMemoryInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.MemoryService/put");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 插入数据，无论存在与否都会插入或更新数据"]
        pub async fn set(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestMemoryInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.MemoryService/set");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 获取数据，返回存储对象"]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestMemoryOut>,
        ) -> Result<tonic::Response<super::ResponseMemoryOut>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.MemoryService/get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 删除数据"]
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestMemoryRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.MemoryService/remove");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 在指定缓存页中插入数据，如果存在则返回已存在"]
        pub async fn put_by_page(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestMemoryPInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.MemoryService/put_by_page");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 在指定缓存页中插入数据，无论存在与否都会插入或更新数据"]
        pub async fn set_by_page(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestMemoryPInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.MemoryService/set_by_page");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 在指定缓存页中获取数据，返回存储对象"]
        pub async fn get_by_page(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestMemoryPOut>,
        ) -> Result<tonic::Response<super::ResponseMemoryPOut>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.MemoryService/get_by_page");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 在指定缓存页中删除数据"]
        pub async fn remove_by_page(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestMemoryPRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/db.MemoryService/remove_by_page");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MemoryServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MemoryServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MemoryServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod user_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UserServiceServer."]
    #[async_trait]
    pub trait UserService: Send + Sync + 'static {
        #[doc = " 数据库用户登录"]
        async fn login(
            &self,
            request: tonic::Request<super::RequestLogin>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UserServiceServer<T: UserService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: UserService> UserServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for UserServiceServer<T>
    where
        T: UserService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/db.UserService/login" => {
                    #[allow(non_camel_case_types)]
                    struct loginSvc<T: UserService>(pub Arc<T>);
                    impl<T: UserService> tonic::server::UnaryService<super::RequestLogin> for loginSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestLogin>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).login(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = loginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: UserService> Clone for UserServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: UserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserService> tonic::transport::NamedService for UserServiceServer<T> {
        const NAME: &'static str = "db.UserService";
    }
}
#[doc = r" Generated server implementations."]
pub mod page_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PageServiceServer."]
    #[async_trait]
    pub trait PageService: Send + Sync + 'static {
        #[doc = " 缓存页集合"]
        async fn list(
            &self,
            request: tonic::Request<super::super::utils::Req>,
        ) -> Result<tonic::Response<super::ResponsePageList>, tonic::Status>;
        #[doc = " 创建缓存页"]
        async fn create(
            &self,
            request: tonic::Request<super::RequestPageCreate>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 修改缓存页"]
        async fn modify(
            &self,
            request: tonic::Request<super::RequestPageModify>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 获取缓存页详情"]
        async fn info(
            &self,
            request: tonic::Request<super::RequestPageInfo>,
        ) -> Result<tonic::Response<super::ResponsePageInfo>, tonic::Status>;
        #[doc = " 删除缓存页"]
        async fn remove(
            &self,
            request: tonic::Request<super::RequestPageRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PageServiceServer<T: PageService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PageService> PageServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for PageServiceServer<T>
    where
        T: PageService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/db.PageService/list" => {
                    #[allow(non_camel_case_types)]
                    struct listSvc<T: PageService>(pub Arc<T>);
                    impl<T: PageService> tonic::server::UnaryService<super::super::utils::Req> for listSvc<T> {
                        type Response = super::ResponsePageList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::utils::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = listSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.PageService/create" => {
                    #[allow(non_camel_case_types)]
                    struct createSvc<T: PageService>(pub Arc<T>);
                    impl<T: PageService> tonic::server::UnaryService<super::RequestPageCreate> for createSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestPageCreate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.PageService/modify" => {
                    #[allow(non_camel_case_types)]
                    struct modifySvc<T: PageService>(pub Arc<T>);
                    impl<T: PageService> tonic::server::UnaryService<super::RequestPageModify> for modifySvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestPageModify>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = modifySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.PageService/info" => {
                    #[allow(non_camel_case_types)]
                    struct infoSvc<T: PageService>(pub Arc<T>);
                    impl<T: PageService> tonic::server::UnaryService<super::RequestPageInfo> for infoSvc<T> {
                        type Response = super::ResponsePageInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestPageInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.PageService/remove" => {
                    #[allow(non_camel_case_types)]
                    struct removeSvc<T: PageService>(pub Arc<T>);
                    impl<T: PageService> tonic::server::UnaryService<super::RequestPageRemove> for removeSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestPageRemove>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = removeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: PageService> Clone for PageServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PageService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PageService> tonic::transport::NamedService for PageServiceServer<T> {
        const NAME: &'static str = "db.PageService";
    }
}
#[doc = r" Generated server implementations."]
pub mod database_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DatabaseServiceServer."]
    #[async_trait]
    pub trait DatabaseService: Send + Sync + 'static {
        #[doc = " 数据库集合"]
        async fn list(
            &self,
            request: tonic::Request<super::super::utils::Req>,
        ) -> Result<tonic::Response<super::ResponseDatabaseList>, tonic::Status>;
        #[doc = " 创建数据库"]
        async fn create(
            &self,
            request: tonic::Request<super::RequestDatabaseCreate>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 修改数据库"]
        async fn modify(
            &self,
            request: tonic::Request<super::RequestDatabaseModify>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 获取数据库详情"]
        async fn info(
            &self,
            request: tonic::Request<super::RequestDatabaseInfo>,
        ) -> Result<tonic::Response<super::ResponseDatabaseInfo>, tonic::Status>;
        #[doc = " 删除数据库"]
        async fn remove(
            &self,
            request: tonic::Request<super::RequestDatabaseRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DatabaseServiceServer<T: DatabaseService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DatabaseService> DatabaseServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for DatabaseServiceServer<T>
    where
        T: DatabaseService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/db.DatabaseService/list" => {
                    #[allow(non_camel_case_types)]
                    struct listSvc<T: DatabaseService>(pub Arc<T>);
                    impl<T: DatabaseService> tonic::server::UnaryService<super::super::utils::Req> for listSvc<T> {
                        type Response = super::ResponseDatabaseList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::utils::Req>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = listSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DatabaseService/create" => {
                    #[allow(non_camel_case_types)]
                    struct createSvc<T: DatabaseService>(pub Arc<T>);
                    impl<T: DatabaseService>
                        tonic::server::UnaryService<super::RequestDatabaseCreate> for createSvc<T>
                    {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDatabaseCreate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DatabaseService/modify" => {
                    #[allow(non_camel_case_types)]
                    struct modifySvc<T: DatabaseService>(pub Arc<T>);
                    impl<T: DatabaseService>
                        tonic::server::UnaryService<super::RequestDatabaseModify> for modifySvc<T>
                    {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDatabaseModify>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = modifySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DatabaseService/info" => {
                    #[allow(non_camel_case_types)]
                    struct infoSvc<T: DatabaseService>(pub Arc<T>);
                    impl<T: DatabaseService> tonic::server::UnaryService<super::RequestDatabaseInfo> for infoSvc<T> {
                        type Response = super::ResponseDatabaseInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDatabaseInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DatabaseService/remove" => {
                    #[allow(non_camel_case_types)]
                    struct removeSvc<T: DatabaseService>(pub Arc<T>);
                    impl<T: DatabaseService>
                        tonic::server::UnaryService<super::RequestDatabaseRemove> for removeSvc<T>
                    {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDatabaseRemove>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = removeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DatabaseService> Clone for DatabaseServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DatabaseService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DatabaseService> tonic::transport::NamedService for DatabaseServiceServer<T> {
        const NAME: &'static str = "db.DatabaseService";
    }
}
#[doc = r" Generated server implementations."]
pub mod view_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ViewServiceServer."]
    #[async_trait]
    pub trait ViewService: Send + Sync + 'static {
        #[doc = " 视图集合"]
        async fn list(
            &self,
            request: tonic::Request<super::RequestViewList>,
        ) -> Result<tonic::Response<super::ResponseViewList>, tonic::Status>;
        #[doc = " 创建视图"]
        async fn create(
            &self,
            request: tonic::Request<super::RequestViewCreate>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 修改视图"]
        async fn modify(
            &self,
            request: tonic::Request<super::RequestViewModify>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 获取视图详情"]
        async fn info(
            &self,
            request: tonic::Request<super::RequestViewInfo>,
        ) -> Result<tonic::Response<super::ResponseViewInfo>, tonic::Status>;
        #[doc = " 删除视图"]
        async fn remove(
            &self,
            request: tonic::Request<super::RequestViewRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 整理归档"]
        async fn archive(
            &self,
            request: tonic::Request<super::RequestViewArchive>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 读取指定归档版本信息"]
        async fn record(
            &self,
            request: tonic::Request<super::RequestViewRecord>,
        ) -> Result<tonic::Response<super::ResponseViewRecord>, tonic::Status>;
        #[doc = " 读取所有归档版本信息"]
        async fn records(
            &self,
            request: tonic::Request<super::RequestViewRecords>,
        ) -> Result<tonic::Response<super::ResponseViewRecords>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ViewServiceServer<T: ViewService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ViewService> ViewServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ViewServiceServer<T>
    where
        T: ViewService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/db.ViewService/list" => {
                    #[allow(non_camel_case_types)]
                    struct listSvc<T: ViewService>(pub Arc<T>);
                    impl<T: ViewService> tonic::server::UnaryService<super::RequestViewList> for listSvc<T> {
                        type Response = super::ResponseViewList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestViewList>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = listSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.ViewService/create" => {
                    #[allow(non_camel_case_types)]
                    struct createSvc<T: ViewService>(pub Arc<T>);
                    impl<T: ViewService> tonic::server::UnaryService<super::RequestViewCreate> for createSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestViewCreate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.ViewService/modify" => {
                    #[allow(non_camel_case_types)]
                    struct modifySvc<T: ViewService>(pub Arc<T>);
                    impl<T: ViewService> tonic::server::UnaryService<super::RequestViewModify> for modifySvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestViewModify>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = modifySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.ViewService/info" => {
                    #[allow(non_camel_case_types)]
                    struct infoSvc<T: ViewService>(pub Arc<T>);
                    impl<T: ViewService> tonic::server::UnaryService<super::RequestViewInfo> for infoSvc<T> {
                        type Response = super::ResponseViewInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestViewInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.ViewService/remove" => {
                    #[allow(non_camel_case_types)]
                    struct removeSvc<T: ViewService>(pub Arc<T>);
                    impl<T: ViewService> tonic::server::UnaryService<super::RequestViewRemove> for removeSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestViewRemove>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = removeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.ViewService/archive" => {
                    #[allow(non_camel_case_types)]
                    struct archiveSvc<T: ViewService>(pub Arc<T>);
                    impl<T: ViewService> tonic::server::UnaryService<super::RequestViewArchive> for archiveSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestViewArchive>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).archive(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = archiveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.ViewService/record" => {
                    #[allow(non_camel_case_types)]
                    struct recordSvc<T: ViewService>(pub Arc<T>);
                    impl<T: ViewService> tonic::server::UnaryService<super::RequestViewRecord> for recordSvc<T> {
                        type Response = super::ResponseViewRecord;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestViewRecord>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).record(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = recordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.ViewService/records" => {
                    #[allow(non_camel_case_types)]
                    struct recordsSvc<T: ViewService>(pub Arc<T>);
                    impl<T: ViewService> tonic::server::UnaryService<super::RequestViewRecords> for recordsSvc<T> {
                        type Response = super::ResponseViewRecords;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestViewRecords>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).records(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = recordsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ViewService> Clone for ViewServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ViewService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ViewService> tonic::transport::NamedService for ViewServiceServer<T> {
        const NAME: &'static str = "db.ViewService";
    }
}
#[doc = r" Generated server implementations."]
pub mod index_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with IndexServiceServer."]
    #[async_trait]
    pub trait IndexService: Send + Sync + 'static {
        #[doc = " 索引集合"]
        async fn list(
            &self,
            request: tonic::Request<super::RequestIndexList>,
        ) -> Result<tonic::Response<super::ResponseIndexList>, tonic::Status>;
        #[doc = " 创建索引"]
        async fn create(
            &self,
            request: tonic::Request<super::RequestIndexCreate>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 获取索引详情"]
        async fn info(
            &self,
            request: tonic::Request<super::RequestIndexInfo>,
        ) -> Result<tonic::Response<super::ResponseIndexInfo>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct IndexServiceServer<T: IndexService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: IndexService> IndexServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for IndexServiceServer<T>
    where
        T: IndexService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/db.IndexService/list" => {
                    #[allow(non_camel_case_types)]
                    struct listSvc<T: IndexService>(pub Arc<T>);
                    impl<T: IndexService> tonic::server::UnaryService<super::RequestIndexList> for listSvc<T> {
                        type Response = super::ResponseIndexList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestIndexList>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = listSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.IndexService/create" => {
                    #[allow(non_camel_case_types)]
                    struct createSvc<T: IndexService>(pub Arc<T>);
                    impl<T: IndexService> tonic::server::UnaryService<super::RequestIndexCreate> for createSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestIndexCreate>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.IndexService/info" => {
                    #[allow(non_camel_case_types)]
                    struct infoSvc<T: IndexService>(pub Arc<T>);
                    impl<T: IndexService> tonic::server::UnaryService<super::RequestIndexInfo> for infoSvc<T> {
                        type Response = super::ResponseIndexInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestIndexInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = infoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: IndexService> Clone for IndexServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: IndexService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: IndexService> tonic::transport::NamedService for IndexServiceServer<T> {
        const NAME: &'static str = "db.IndexService";
    }
}
#[doc = r" Generated server implementations."]
pub mod disk_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DiskServiceServer."]
    #[async_trait]
    pub trait DiskService: Send + Sync + 'static {
        #[doc = " 插入数据，如果存在则返回已存在"]
        async fn put(
            &self,
            request: tonic::Request<super::RequestDiskInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 插入数据，无论存在与否都会插入或更新数据"]
        async fn set(
            &self,
            request: tonic::Request<super::RequestDiskInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 获取数据，返回存储对象"]
        async fn get(
            &self,
            request: tonic::Request<super::RequestDiskOut>,
        ) -> Result<tonic::Response<super::ResponseDiskOut>, tonic::Status>;
        #[doc = " 根据指定索引名称获取数据，返回存储对象"]
        async fn get_by_index(
            &self,
            request: tonic::Request<super::RequestDiskIOut>,
        ) -> Result<tonic::Response<super::ResponseDiskOut>, tonic::Status>;
        #[doc = " 删除数据"]
        async fn remove(
            &self,
            request: tonic::Request<super::RequestDiskRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 条件检索"]
        async fn select(
            &self,
            request: tonic::Request<super::RequestDiskSelect>,
        ) -> Result<tonic::Response<super::ResponseDiskSelect>, tonic::Status>;
        #[doc = " 条件删除"]
        async fn delete(
            &self,
            request: tonic::Request<super::RequestDiskDelete>,
        ) -> Result<tonic::Response<super::ResponseDiskDelete>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DiskServiceServer<T: DiskService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DiskService> DiskServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for DiskServiceServer<T>
    where
        T: DiskService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/db.DiskService/put" => {
                    #[allow(non_camel_case_types)]
                    struct putSvc<T: DiskService>(pub Arc<T>);
                    impl<T: DiskService> tonic::server::UnaryService<super::RequestDiskInto> for putSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDiskInto>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).put(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = putSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DiskService/set" => {
                    #[allow(non_camel_case_types)]
                    struct setSvc<T: DiskService>(pub Arc<T>);
                    impl<T: DiskService> tonic::server::UnaryService<super::RequestDiskInto> for setSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDiskInto>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = setSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DiskService/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: DiskService>(pub Arc<T>);
                    impl<T: DiskService> tonic::server::UnaryService<super::RequestDiskOut> for getSvc<T> {
                        type Response = super::ResponseDiskOut;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDiskOut>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DiskService/get_by_index" => {
                    #[allow(non_camel_case_types)]
                    struct get_by_indexSvc<T: DiskService>(pub Arc<T>);
                    impl<T: DiskService> tonic::server::UnaryService<super::RequestDiskIOut> for get_by_indexSvc<T> {
                        type Response = super::ResponseDiskOut;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDiskIOut>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_by_index(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = get_by_indexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DiskService/remove" => {
                    #[allow(non_camel_case_types)]
                    struct removeSvc<T: DiskService>(pub Arc<T>);
                    impl<T: DiskService> tonic::server::UnaryService<super::RequestDiskRemove> for removeSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDiskRemove>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = removeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DiskService/select" => {
                    #[allow(non_camel_case_types)]
                    struct selectSvc<T: DiskService>(pub Arc<T>);
                    impl<T: DiskService> tonic::server::UnaryService<super::RequestDiskSelect> for selectSvc<T> {
                        type Response = super::ResponseDiskSelect;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDiskSelect>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).select(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = selectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.DiskService/delete" => {
                    #[allow(non_camel_case_types)]
                    struct deleteSvc<T: DiskService>(pub Arc<T>);
                    impl<T: DiskService> tonic::server::UnaryService<super::RequestDiskDelete> for deleteSvc<T> {
                        type Response = super::ResponseDiskDelete;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDiskDelete>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = deleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DiskService> Clone for DiskServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DiskService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DiskService> tonic::transport::NamedService for DiskServiceServer<T> {
        const NAME: &'static str = "db.DiskService";
    }
}
#[doc = r" Generated server implementations."]
pub mod memory_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MemoryServiceServer."]
    #[async_trait]
    pub trait MemoryService: Send + Sync + 'static {
        #[doc = " 插入数据，如果存在则返回已存在"]
        async fn put(
            &self,
            request: tonic::Request<super::RequestMemoryInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 插入数据，无论存在与否都会插入或更新数据"]
        async fn set(
            &self,
            request: tonic::Request<super::RequestMemoryInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 获取数据，返回存储对象"]
        async fn get(
            &self,
            request: tonic::Request<super::RequestMemoryOut>,
        ) -> Result<tonic::Response<super::ResponseMemoryOut>, tonic::Status>;
        #[doc = " 删除数据"]
        async fn remove(
            &self,
            request: tonic::Request<super::RequestMemoryRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 在指定缓存页中插入数据，如果存在则返回已存在"]
        async fn put_by_page(
            &self,
            request: tonic::Request<super::RequestMemoryPInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 在指定缓存页中插入数据，无论存在与否都会插入或更新数据"]
        async fn set_by_page(
            &self,
            request: tonic::Request<super::RequestMemoryPInto>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        #[doc = " 在指定缓存页中获取数据，返回存储对象"]
        async fn get_by_page(
            &self,
            request: tonic::Request<super::RequestMemoryPOut>,
        ) -> Result<tonic::Response<super::ResponseMemoryPOut>, tonic::Status>;
        #[doc = " 在指定缓存页中删除数据"]
        async fn remove_by_page(
            &self,
            request: tonic::Request<super::RequestMemoryPRemove>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MemoryServiceServer<T: MemoryService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: MemoryService> MemoryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for MemoryServiceServer<T>
    where
        T: MemoryService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/db.MemoryService/put" => {
                    #[allow(non_camel_case_types)]
                    struct putSvc<T: MemoryService>(pub Arc<T>);
                    impl<T: MemoryService> tonic::server::UnaryService<super::RequestMemoryInto> for putSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestMemoryInto>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).put(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = putSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.MemoryService/set" => {
                    #[allow(non_camel_case_types)]
                    struct setSvc<T: MemoryService>(pub Arc<T>);
                    impl<T: MemoryService> tonic::server::UnaryService<super::RequestMemoryInto> for setSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestMemoryInto>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = setSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.MemoryService/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: MemoryService>(pub Arc<T>);
                    impl<T: MemoryService> tonic::server::UnaryService<super::RequestMemoryOut> for getSvc<T> {
                        type Response = super::ResponseMemoryOut;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestMemoryOut>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.MemoryService/remove" => {
                    #[allow(non_camel_case_types)]
                    struct removeSvc<T: MemoryService>(pub Arc<T>);
                    impl<T: MemoryService> tonic::server::UnaryService<super::RequestMemoryRemove> for removeSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestMemoryRemove>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = removeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.MemoryService/put_by_page" => {
                    #[allow(non_camel_case_types)]
                    struct put_by_pageSvc<T: MemoryService>(pub Arc<T>);
                    impl<T: MemoryService> tonic::server::UnaryService<super::RequestMemoryPInto>
                        for put_by_pageSvc<T>
                    {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestMemoryPInto>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).put_by_page(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = put_by_pageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.MemoryService/set_by_page" => {
                    #[allow(non_camel_case_types)]
                    struct set_by_pageSvc<T: MemoryService>(pub Arc<T>);
                    impl<T: MemoryService> tonic::server::UnaryService<super::RequestMemoryPInto>
                        for set_by_pageSvc<T>
                    {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestMemoryPInto>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_by_page(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = set_by_pageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.MemoryService/get_by_page" => {
                    #[allow(non_camel_case_types)]
                    struct get_by_pageSvc<T: MemoryService>(pub Arc<T>);
                    impl<T: MemoryService> tonic::server::UnaryService<super::RequestMemoryPOut> for get_by_pageSvc<T> {
                        type Response = super::ResponseMemoryPOut;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestMemoryPOut>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_by_page(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = get_by_pageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/db.MemoryService/remove_by_page" => {
                    #[allow(non_camel_case_types)]
                    struct remove_by_pageSvc<T: MemoryService>(pub Arc<T>);
                    impl<T: MemoryService> tonic::server::UnaryService<super::RequestMemoryPRemove>
                        for remove_by_pageSvc<T>
                    {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestMemoryPRemove>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_by_page(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = remove_by_pageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: MemoryService> Clone for MemoryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: MemoryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MemoryService> tonic::transport::NamedService for MemoryServiceServer<T> {
        const NAME: &'static str = "db.MemoryService";
    }
}
