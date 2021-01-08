use std::fs::File;
use std::sync::{Arc, RwLock};

use comm::errors::entrances::GeorgeResult;

use crate::engine::siam::selector::{Condition, Constraint};
use crate::engine::traits::TSeed;
use crate::utils::comm::IndexMold;

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
    ) -> GeorgeResult<()>
    where
        Self: Sized;
    /// 获取数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>>;
    /// 删除数据，返回存储对象<p><p>
    ///
    /// ###Params
    ///
    /// key string
    ///
    /// ###Return
    ///
    /// Seed value信息
    fn remove(&self, key: String) -> GeorgeResult<Vec<u8>>;
    /// 获取最后一条记录数据，返回存储对象
    fn get_last(&self) -> GeorgeResult<Vec<u8>>;
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
    /// total 检索过程中遍历的总条数
    ///
    /// count 检索结果过程中遍历的总条数
    ///
    /// values 检索结果集合
    fn select(
        &self,
        mold: IndexMold,
        left: bool,
        start: u64,
        end: u64,
        constraint: Constraint,
    ) -> GeorgeResult<(u64, u64, Vec<Vec<u8>>)>;
    /// 通过查询约束删除数据集
    ///
    /// ###Params
    ///
    /// left 是否左查询
    ///
    /// constraint 查询约束
    ///
    /// ###Return
    ///
    /// total 检索过程中遍历的总条数
    ///
    /// count 检索结果过程中遍历的总条数
    fn delete(
        &self,
        mold: IndexMold,
        left: bool,
        start: u64,
        end: u64,
        constraint: Constraint,
    ) -> GeorgeResult<(u64, u64)>;
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
    fn put_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u64,
        seed: Arc<RwLock<dyn TSeed>>,
        force: bool,
        root: bool,
        next_node_seek: u64,
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
    fn get_in_node(
        &self,
        node_bytes: Vec<u8>,
        level: u8,
        flexible_key: u64,
    ) -> GeorgeResult<Vec<u8>>;
    /// 获取数据真实操作
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// level 当前操作结点层
    fn get_last_in_node(&self, node_bytes: Vec<u8>, level: u8) -> GeorgeResult<Vec<u8>>;
    /// 通过左查询约束获取数据集
    ///
    /// ###Params
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// conditions 条件集合
    ///
    /// skip 结果集跳过数量
    ///
    /// limit 结果集限制数量
    ///
    /// delete 是否删除检索结果
    ///
    /// ###Return
    ///
    /// total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    ///
    /// count 检索结果过程中遍历的总条数
    ///
    /// skip 检索结果过程中跳过数量
    ///
    /// limit 检索结果过程中限制数量
    ///
    /// values 检索结果集合
    fn left_query(
        &self,
        mold: IndexMold,
        index_file: Arc<RwLock<File>>,
        view_file: Arc<RwLock<File>>,
        node_bytes: Vec<u8>,
        start_key: u64,
        end_key: u64,
        level: u8,
        conditions: Vec<Condition>,
        skip: u64,
        limit: u64,
        delete: bool,
    ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)>;
    /// 通过右查询约束获取数据集
    ///
    /// ###Params
    ///
    /// node_bytes 当前操作结点的字节数组
    ///
    /// conditions 条件集合
    ///
    /// skip 结果集跳过数量
    ///
    /// limit 结果集限制数量
    ///
    /// delete 是否删除检索结果
    ///
    /// ###Return
    ///
    /// total 检索过程中遍历的总条数（也表示文件读取次数，文件描述符次数远小于该数，一般文件描述符数为1，即共用同一文件描述符）
    ///
    /// count 检索结果过程中遍历的总条数
    ///
    /// skip 检索结果过程中跳过数量
    ///
    /// limit 检索结果过程中限制数量
    ///
    /// values 检索结果集合
    fn right_query(
        &self,
        mold: IndexMold,
        index_file: Arc<RwLock<File>>,
        view_file: Arc<RwLock<File>>,
        node_bytes: Vec<u8>,
        start_key: u64,
        end_key: u64,
        level: u8,
        conditions: Vec<Condition>,
        skip: u64,
        limit: u64,
        delete: bool,
    ) -> GeorgeResult<(u64, u64, u64, u64, Vec<Vec<u8>>)>;
}
