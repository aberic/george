/// 合约
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractBase {
    /// 合约id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 合约版本
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// 合约
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<ContractBase>,
    /// 合约名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 合约描述
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// 读写集信息属于每笔交易中的属性
///
/// 在任一区块中，来自各个节点交易中都包含了该笔交易提交时本地最大区块编号，该区块编号为本次交易的基准编号
///
/// 即当前交易成立条件是在本次出块交易都小于等于该基准编号为前提
///
/// 假如一次出块中打包了两笔交易T1和T2，T1的基准区块编号为5，T2的基准区块编号为6，则无论T1执行了任何操作都无效
///
/// 假如一次出块中打包了两笔交易T1和T2，T1和T2的基准区块编号都是5，按照交易接收时间排序，接收越早的交易排在前面。假定T1的接受时间小于T2，即无论
/// T1的提交时间是否大于T2都会优先处理T1的数据，提交时间只针对节点自身做校验使用，没有事实依据。T1如果对k1进行了写操作，那么T2及后续的交易中将无
/// 法执行有关读k1的操作，即后续交易中所有与读k1相关的交易都被置为无效
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RwSet {
    /// 基准区块编号，从创世区块起始到当前区块的升序序号，创世区块编号为0
    #[prost(uint64, tag = "1")]
    pub block_number: u64,
}
/// 读key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadKey {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// 写key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteKey {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// 签名印章，签名者信息集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signet {
    #[prost(message, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<Signer>,
}
/// 签名者信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signer {
    /// 签名者id，如o1、p1、u1
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// sm2签名结果
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
}
/// 区块中每笔被提交的交易
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// 交易data集合
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 交易数据信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// 交易hash，sm3
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// 交易接收时间
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<super::utils::Timestamp>,
    /// 交易提交时间
    #[prost(message, optional, tag = "3")]
    pub commit: ::core::option::Option<super::utils::Timestamp>,
    /// 交易所依赖合约
    #[prost(message, optional, tag = "4")]
    pub contract: ::core::option::Option<ContractBase>,
    /// 交易提交的方法名
    #[prost(string, tag = "5")]
    pub function: ::prost::alloc::string::String,
    /// 交易提交的数据
    #[prost(bytes = "vec", tag = "6")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// 读写集
    #[prost(message, optional, tag = "7")]
    pub set: ::core::option::Option<RwSet>,
    /// 对当前交易hash进行sm2签名的信息，签名者为交易首次提交节点，如p1
    #[prost(message, optional, tag = "8")]
    pub signer: ::core::option::Option<Signer>,
}
/// 区块结构，该结构为节点出块、校验、共享以及同步等通用结构，最终会存储在指定的 block file 文件中
///
/// 该结构与一般意义上的区块类似，包括了区块头和区块数据，区块头包括该区块链的一些基本信息，区块数据则是区块同步、校验、共享的状态信息
///
/// 区块数据会被存储在区块文件中，在区块文件中存储信息来自 metadata ，即在区块文件中读取任一区块内容，首先读取该区块的 metadata 信息，在该信息
/// 中定义了待读取区块字节长度，且能够追溯上一区块坐标
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<BlockHeader>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<BlockData>,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<BlockMetadata>,
}
/// 区块头
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    /// 区块编号，从创世区块起始到当前区块的升序序号，创世区块编号为0
    #[prost(uint64, tag = "1")]
    pub number: u64,
    /// 上一区块hash
    #[prost(string, tag = "2")]
    pub pre_hash: ::prost::alloc::string::String,
    /// 当前区块hash，hash值来自区块数据中交易的默克尔树
    #[prost(string, tag = "3")]
    pub hash: ::prost::alloc::string::String,
    /// 区块打包时间
    #[prost(message, optional, tag = "4")]
    pub timestamp: ::core::option::Option<super::utils::Timestamp>,
}
/// 区块体
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockData {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// 区块元数据，记录区块存储信息和上下游数据索引
///
/// 包含自身区块长度4字节+上一区块文件编号4字节+上一区块所属文件下标4字节=12字节
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMetadata {
    /// 区块所在文件中字节数组长度
    #[prost(uint32, tag = "1")]
    pub length: u32,
    /// 上一区块索引，所在文件起始下标，如果该值大于当前文件索引，则表示上一区块在上一区块文件中
    #[prost(message, optional, tag = "2")]
    pub index: ::core::option::Option<PreBlockIndex>,
    /// 对当前区块hash进行sm2签名的信息，签名者为出块共识节点，如o1
    #[prost(message, optional, tag = "3")]
    pub signer: ::core::option::Option<Signer>,
}
/// 上一区块索引，记录上一区块存储数据索引
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreBlockIndex {
    /// 区块所在文件编号
    #[prost(uint32, tag = "1")]
    pub no: u32,
    /// 区块所在文件起始位置，所在文件起始下标
    #[prost(uint32, tag = "2")]
    pub seek: u32,
    /// 区块所在文件持续长度
    #[prost(uint32, tag = "3")]
    pub length: u32,
}
/// 区块数据信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    /// 交易对象
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBlock {
    #[prost(oneof = "request_block::Get", tags = "1, 2, 3")]
    pub get: ::core::option::Option<request_block::Get>,
}
/// Nested message and enum types in `RequestBlock`.
pub mod request_block {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Get {
        #[prost(uint32, tag = "1")]
        Height(u32),
        #[prost(string, tag = "2")]
        Hash(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        TxHash(::prost::alloc::string::String),
    }
}
/// 信息载体，包含授信的签名信息头
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// 数据载体头
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    /// 数据内容，类型由数据载体头中类型定义
    ///
    /// 主要有LedgerConfig、BlockInfo和TransactionInfo
    ///
    /// hex.decode解析
    #[prost(bytes = "vec", tag = "2")]
    pub info: ::prost::alloc::vec::Vec<u8>,
}
/// 数据载体头
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(enumeration = "HeaderType", tag = "1")]
    pub r#type: i32,
    /// 签名头，hex.decode解析
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// 账本头，hex.decode解析
    #[prost(bytes = "vec", tag = "3")]
    pub ledger: ::prost::alloc::vec::Vec<u8>,
}
/// 签名头是对数据载体进行签名的结果，并包含了签名者的证书信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureHeader {
    /// 创建者证书信息，hex.decode解析
    #[prost(bytes = "vec", tag = "1")]
    pub creator: ::prost::alloc::vec::Vec<u8>,
    /// 签名信息，接收方验签
    #[prost(bytes = "vec", tag = "2")]
    pub sign: ::prost::alloc::vec::Vec<u8>,
}
/// 账本头
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerHeader {
    /// 账本id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 账本创世区块版本，创世区块记录账本参与方的信息，允许变更，会产生版本迭代数据
    #[prost(uint32, tag = "2")]
    pub version: u32,
}
/// 数据载体头类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HeaderType {
    /// 未定义数据类型
    Undefined = 0,
    /// 账本数据类型
    Ledger = 1,
    /// 合约数据类型
    Contract = 2,
    /// 区块数据类型
    Block = 3,
    /// 交易数据类型
    Transaction = 4,
}
/// 节点
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    /// 节点id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 节点可被访问的地址，如www.peer.com/192.168.0.1
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
    /// 节点可被访问的地址监听的端口号
    #[prost(uint32, tag = "3")]
    pub port: u32,
    /// 是否允许参与更新创世区块信息
    #[prost(bool, tag = "4")]
    pub genesis: bool,
}
/// 节点策略
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerPolicy {
    /// 节点id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 该节点是否为共识节点，如果组织不参与共识，则该节点必然不会成为共识节点
    #[prost(bool, tag = "2")]
    pub consensus: bool,
    /// 是否允许升级智能合约，允许升级智能合约的组织可以通过upgrade接口将旧版本合约升级到新版本
    #[prost(bool, tag = "3")]
    pub contract: bool,
    /// 是否允许参与更新账本信息
    #[prost(bool, tag = "4")]
    pub ledger: bool,
}
/// 组织机构
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organization {
    /// 组织机构id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 组织机构名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 组织机构签名公钥
    #[prost(string, tag = "3")]
    pub pk: ::prost::alloc::string::String,
    /// 组织机构根证书
    #[prost(string, tag = "4")]
    pub root_cert: ::prost::alloc::string::String,
    /// 组织下所属节点集合
    #[prost(message, repeated, tag = "5")]
    pub peers: ::prost::alloc::vec::Vec<Peer>,
    /// 是否允许参与更新创世区块信息
    #[prost(bool, tag = "6")]
    pub genesis: bool,
}
/// 组织机构策略
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationPolicy {
    /// 组织机构id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 该组织是否参与共识
    #[prost(bool, tag = "2")]
    pub consensus: bool,
    /// 是否允许升级智能合约，允许升级智能合约的组织可以通过upgrade接口将旧版本合约升级到新版本
    #[prost(bool, tag = "3")]
    pub contract: bool,
    /// 是否允许参与更新账本信息
    #[prost(bool, tag = "4")]
    pub ledger: bool,
    /// 组织下所属节点集合
    #[prost(message, repeated, tag = "5")]
    pub peers: ::prost::alloc::vec::Vec<PeerPolicy>,
}
/// 运行依据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Foundation {
    /// 出块超时时间，单位秒
    #[prost(uint32, tag = "1")]
    pub timeout: u32,
    /// 区块上限限制，单位Mb
    #[prost(uint32, tag = "2")]
    pub block_size: u32,
    /// 区块中交易数量上限限制
    #[prost(uint32, tag = "3")]
    pub tx_count: u32,
    /// 交易上限限制，单位Kb
    #[prost(uint32, tag = "4")]
    pub tx_size: u32,
}
/// 更新策略
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradePolicy {
    /// 智能合约部署/更新/停用/删除策略
    #[prost(enumeration = "Policy", tag = "1")]
    pub contract: i32,
    /// 账本更新策略
    #[prost(enumeration = "Policy", tag = "2")]
    pub ledger: i32,
}
/// 执行策略
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Policy {
    /// 任意
    Any = 0,
    /// 半数以上
    Majority = 1,
    /// 全数
    All = 2,
}
/// 创世区块配置对象
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisConfig {
    /// 创世区块
    #[prost(message, optional, tag = "1")]
    pub genesis: ::core::option::Option<Genesis>,
    /// 对当前创世区块信息进行的满足当前创世区块执行策略的签名信息集合
    #[prost(message, optional, tag = "2")]
    pub signet: ::core::option::Option<Signet>,
    /// 对当前创世区块信息进行的满足前一创世区块执行策略的签名信息集合
    #[prost(message, optional, tag = "3")]
    pub signet_pre: ::core::option::Option<Signet>,
}
/// 创世区块
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Genesis {
    /// 整个平台的组织机构集合
    #[prost(message, repeated, tag = "1")]
    pub organizations: ::prost::alloc::vec::Vec<Organization>,
    /// 默认账本运行依据
    #[prost(message, optional, tag = "2")]
    pub foundation: ::core::option::Option<Foundation>,
    /// 平台创世区块更新策略
    #[prost(enumeration = "Policy", tag = "3")]
    pub policy: i32,
}
/// 账本配置对象
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerConfig {
    /// 账本
    #[prost(message, optional, tag = "1")]
    pub ledger: ::core::option::Option<Ledger>,
    /// 对当前账本信息进行的满足当前账本执行策略的签名信息集合
    #[prost(message, optional, tag = "2")]
    pub signet: ::core::option::Option<Signet>,
    /// 对当前账本信息进行的满足前一账本执行策略的签名信息集合
    #[prost(message, optional, tag = "3")]
    pub signet_pre: ::core::option::Option<Signet>,
}
/// 账本
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ledger {
    /// 账本id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// 账本名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 账本描述
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// 允许参与账本的组织机构集合及组织机构策略集合
    #[prost(message, repeated, tag = "4")]
    pub organization_policies: ::prost::alloc::vec::Vec<OrganizationPolicy>,
    /// 账本运行依据
    #[prost(message, optional, tag = "5")]
    pub foundation: ::core::option::Option<Foundation>,
    /// 账本更新策略
    #[prost(enumeration = "Policy", tag = "6")]
    pub policy: i32,
    /// 账本运行子内容更新策略
    #[prost(message, optional, tag = "7")]
    pub upgrade_policy: ::core::option::Option<UpgradePolicy>,
}
#[doc = r" Generated client implementations."]
pub mod blocks_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct BlocksClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BlocksClient<tonic::transport::Channel> {
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
    impl<T> BlocksClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BlocksClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BlocksClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn add(
            &mut self,
            request: impl tonic::IntoRequest<super::Block>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chain.Blocks/add");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestBlock>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/chain.Blocks/get");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod blocks_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BlocksServer."]
    #[async_trait]
    pub trait Blocks: Send + Sync + 'static {
        async fn add(
            &self,
            request: tonic::Request<super::Block>,
        ) -> Result<tonic::Response<super::super::utils::Resp>, tonic::Status>;
        async fn get(
            &self,
            request: tonic::Request<super::RequestBlock>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BlocksServer<T: Blocks> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Blocks> BlocksServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for BlocksServer<T>
    where
        T: Blocks,
        B: Body + Send + Sync + 'static,
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
                "/chain.Blocks/add" => {
                    #[allow(non_camel_case_types)]
                    struct addSvc<T: Blocks>(pub Arc<T>);
                    impl<T: Blocks> tonic::server::UnaryService<super::Block> for addSvc<T> {
                        type Response = super::super::utils::Resp;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Block>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = addSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/chain.Blocks/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: Blocks>(pub Arc<T>);
                    impl<T: Blocks> tonic::server::UnaryService<super::RequestBlock> for getSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestBlock>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
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
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Blocks> Clone for BlocksServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Blocks> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Blocks> tonic::transport::NamedService for BlocksServer<T> {
        const NAME: &'static str = "chain.Blocks";
    }
}
