use comm::errors::entrances::GeorgeResult;
use crate::engine::traits::TSeed;

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub(crate) struct Seed {
    value: Vec<u8>,
}

/// 封装方法函数
impl Seed {
    /// 新建seed
    pub(crate) fn create(value: Vec<u8>) -> Seed {
        return Seed { value };
    }
}

/// 封装方法函数
impl TSeed for Seed {
    fn key(&self) -> String {
        "".to_string()
    }
    fn value(&self) -> Option<Vec<u8>> {
        Some(self.value.clone())
    }
    fn modify(&mut self, _value: Vec<u8>) {}
    fn save(&mut self, _value: Vec<u8>) -> GeorgeResult<()> {
        Ok(())
    }
}
