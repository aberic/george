#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Req {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resp {
    #[prost(enumeration = "Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub msg_err: ::prost::alloc::string::String,
}
/// 返回类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    /// 一切正常
    Ok = 0,
    /// 该操作被取消(通常由调用者)
    Cancelled = 1,
    /// 未知错误。此错误可能返回的位置的示例是如果从其他地址空间接收到的Status值属于在此地址空间中未知的错误空间。
    /// 也由没有返回足够错误信息的api引发的错误可能转换为此错误
    Unknown = 2,
    /// 客户端指定了一个无效参数。注意这是不同的从FAILED_PRECONDITION。INVALID_ARGUMENT显示参数不管系统的状态如何，这些都是有问题的
    /// (例如，格式错误的文件名)。
    Argument = 3,
    /// 在操作完成之前，截止日期已经过期。操作更改系统状态时，可能会返回此错误即使操作已经成功完成。
    /// 例如,一个服务器的成功响应可能会延迟很长时间足够让最后期限到期。
    DeadlineExceeded = 4,
    /// 一些请求的实体(例如，文件或目录)没有找到。
    NotFound = 5,
    /// 我们试图创建的实体(例如，文件或目录)，但已经存在。
    AlreadyExists = 6,
    /// 用者没有权限执行指定的操作
    PermissionDenied = 7,
    /// 对象的请求没有有效的身份验证凭证操作。
    Unauthenticated = 8,
    /// 某些资源已经耗尽，可能是每个用户配额，或可能是整个文件系统空间不足。
    ResourceExhausted = 9,
    /// 操作被拒绝，因为系统没有处于执行操作所需的状态。例如，要删除的目录可能是非空的，对非目录应用rmdir操作等
    FailedPrecondition = 10,
    /// 操作被中止，通常是由于并发问题，如序列检查失败，事务中止等
    Aborted = 11,
    /// 操作试图超过有效范围。例如，查找或读取超出文件末尾的内容
    OutOfRange = 12,
    /// 此服务不支持或不支持操作
    Unimplemented = 13,
    /// 内部错误。意味着基础系统所期望的一些不变量被打破了。如果你看到这些错误中的一个，说明有些东西坏得很厉害。
    Internal = 14,
    /// 当前服务不可用
    Unavailable = 15,
    /// 不可恢复的数据丢失或损坏
    DataLoss = 16,
    /// 非定义的自由错误类型
    Custom = 17,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
