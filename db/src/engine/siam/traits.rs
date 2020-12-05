use std::sync::{Arc, RwLock};

use comm::errors::entrances::GeorgeResult;

use crate::engine::siam::selector::Constraint;
use crate::engine::traits::TSeed;
use crate::utils::comm::LevelType;

/// 结点通用特性，遵循此特性创建结点可以更方便的针对db进行扩展
///
/// 该特性包含了结点的基本方法，理论上都需要进行实现才能使用
pub trait TNode: Send + Sync {
    /// 当前结点所在集合中的索引下标，该坐标不一定在数组中的正确位置，但一定是逻辑正确的
    fn degree_index(&self) -> u16;
    /// 子结点集合Vec，允许为空Option，多线程共享数据Arc，支持并发操作RWLock，集合内存储指针Box，指针类型为Node
    fn nodes(&self) -> Option<Arc<RwLock<Vec<Arc<Self>>>>>;
    /// 叶子结点下真实存储数据的集合，该集合主要目的在于解决Hash碰撞，允许为空Option，多线程共享数据Arc，
    /// 支持并发操作RWLock，集合内存储指针Box，指针类型为Seed
    fn seeds(&self) -> Option<Arc<RwLock<Vec<Arc<RwLock<dyn TSeed>>>>>>;
    /// 存储结点所属各子结点坐标顺序字符串
    ///
    /// 如果子项是node集合，在node集合中每一个node的默认字节长度是8，数量是256，即一次性读取2048个字节
    ///
    /// 如果子项是seed集合，在seed集合中每一个seed的默认字符长度是6，当前叶子node会存储叶子中首个出现hash碰撞的
    /// seed起始坐标，每一个seed都会存储出现hash碰撞的下一seed起始坐标
    fn node_bytes(&self) -> Arc<RwLock<Vec<u8>>>;
    fn set_node_bytes(&self, bytes: Vec<u8>);
    /// 新建普通结点
    ///
    /// 该结点需要定义层和度
    ///
    /// 该结点需要指定上一结点，该方法不判断上一结点是否为None，但在检索等操作时可能会造成该结果丢失
    ///
    /// 该结点下真实存储数据的集合必然为None
    fn create_node(degree_index: u16) -> Arc<Self>;
    /// 新建叶子结点
    ///
    /// 该结点需要定义层和度
    ///
    /// 该结点需要指定上一结点，该方法不判断上一结点是否为None，但在检索等操作时可能会造成该结果丢失
    ///
    /// 该结点的子结点集合必然为None
    fn create_leaf(degree_index: u16) -> Arc<Self>;
    /// 插入数据<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// force 如果存在原值，是否覆盖原结果
    ///
    /// description_len 描述长度
    ///
    /// ###Return
    ///
    /// EngineResult<()>
    fn put(
        &self,
        key: String,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        description_len: usize,
        level_type: LevelType,
    ) -> GeorgeResult<()>
    where
        Self: Sized;
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// description_len 描述长度
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get(
        &self,
        key: String,
        description_len: usize,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>>;
    /// 获取最后一条记录数据，返回存储对象
    fn get_last(&self, level_type: LevelType) -> GeorgeResult<Vec<u8>>;
    /// 通过查询约束获取数据集
    ///
    /// ###Params
    ///
    /// left 是否左查询
    ///
    /// constraint 查询约束
    ///
    /// ###Return
    ///
    /// count 检索结果过程中遍历的总条数
    ///
    /// values 检索结果集合
    fn select(
        &self,
        left: bool,
        constraint: Constraint,
        level_type: LevelType,
    ) -> GeorgeResult<(u64, Vec<Vec<u8>>)>;
}

/// 存储文件结点通用特性，遵循此特性创建结点可以更方便的针对db进行扩展
///
/// 该特性包含了结点的基本方法，理论上都需要进行实现才能使用
pub trait DiskNode: Send + Sync {
    fn database_id(&self) -> String;
    fn view_id(&self) -> String;
    fn index_id(&self) -> String;
    fn view_file_path(&self) -> String;
    fn index_file_path(&self) -> String;
    fn modify_node_bytes(&self, start: usize, vs: Vec<u8>);
    /// 存储数据真实操作
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// level 当前操作结点层
    ///
    /// hash_key 存储数据hash
    ///
    /// flexible_key 下一级最左最小树所对应真实key
    ///
    /// Seed value信息
    ///
    /// force 如果存在原值，是否覆盖原结果
    ///
    /// root 是否根结点
    ///
    /// node_seek 当前操作结点在文件中的真实起始位置
    fn put_32_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u32,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        root: bool,
        next_node_seek: u64,
        level_type: LevelType,
    ) -> GeorgeResult<()>
    where
        Self: Sized;
    /// 获取数据真实操作
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// level 当前操作结点层
    ///
    /// hash_key 存储数据hash
    ///
    /// flexible_key 下一级最左最小树所对应真实key
    ///
    /// Seed value信息
    ///
    /// force 如果存在原值，是否覆盖原结果
    ///
    /// root 是否根结点
    ///
    /// node_seek 当前操作结点在文件中的真实起始位置
    fn get_32_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u32,
        root: bool,
        node_seek: u64,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>>;
    /// 存储数据真实操作
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// level 当前操作结点层
    ///
    /// hash_key 存储数据hash
    ///
    /// flexible_key 下一级最左最小树所对应真实key
    ///
    /// Seed value信息
    ///
    /// force 如果存在原值，是否覆盖原结果
    ///
    /// root 是否根结点
    ///
    /// node_seek 当前操作结点在文件中的真实起始位置
    fn put_64_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        root: bool,
        next_node_seek: u64,
        level_type: LevelType,
    ) -> GeorgeResult<()>
    where
        Self: Sized;
    /// 获取数据真实操作
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// level 当前操作结点层
    ///
    /// hash_key 存储数据hash
    ///
    /// flexible_key 下一级最左最小树所对应真实key
    ///
    /// Seed value信息
    ///
    /// force 如果存在原值，是否覆盖原结果
    ///
    /// root 是否根结点
    ///
    /// node_seek 当前操作结点在文件中的真实起始位置
    fn get_64_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u64,
        root: bool,
        node_seek: u64,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>>;
    /// 获取数据真实操作
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// level 当前操作结点层
    fn get_last_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        level_type: LevelType,
    ) -> GeorgeResult<Vec<u8>>;
    /// 通过左查询约束获取数据集
    ///
    /// ###Params
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// constraint 查询约束
    ///
    /// ###Return
    ///
    /// count 检索结果过程中遍历的总条数
    ///
    /// values 检索结果集合
    fn left_query(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        level_type: LevelType,
        constraint: Constraint,
    ) -> GeorgeResult<(u64, Vec<Vec<u8>>)>;
    /// 通过右查询约束获取数据集
    ///
    /// ###Params
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// constraint 查询约束
    ///
    /// ###Return
    ///
    /// count 检索结果过程中遍历的总条数
    ///
    /// values 检索结果集合
    fn right_query(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        level_type: LevelType,
        constraint: Constraint,
    ) -> GeorgeResult<(u64, Vec<Vec<u8>>)>;
}
