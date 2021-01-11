use serde::{Deserialize, Serialize};

use comm::errors::entrances::{err_string, GeorgeResult};
use comm::io::writer::write_seek_u8s;
use comm::trans::{trans_bytes_2_u64, trans_u64_2_bytes};

use crate::engine::traits::TSeed;
use crate::utils::store::{store_view_id, Tag};
use crate::utils::writer::GLOBAL_WRITER;

/// B+Tree索引叶子结点内防hash碰撞数组结构中单体结构
///
/// 搭配Index使用
///
/// 叶子节点下真实存储数据的集合单体结构
#[derive(Debug)]
pub struct Seed {
    database_id: String,
    view_id: String,
    value: Vec<u8>,
    idxes: Vec<Idx>,
    invalid: bool,
    error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Idx {
    index_file_path: String,
    next_node_seek: u64,
    start: u64,
}

fn trans(v8s: Vec<u8>) -> GeorgeResult<Idx> {
    match serde_json::from_slice(v8s.as_slice()) {
        Ok(t) => Ok(t),
        Err(err) => Err(err_string(err.to_string())),
    }
}

fn seed_bytes(value: Vec<u8>) -> Vec<u8> {
    let mut seed_bytes = value.clone();
    let mut seed_bytes_len_bytes = trans_u64_2_bytes(seed_bytes.len() as u64);
    seed_bytes_len_bytes.append(&mut seed_bytes);
    seed_bytes_len_bytes
}

/// 封装方法函数
impl Seed {
    /// 新建seed
    pub fn create(database_id: String, view_id: String, value: Vec<u8>) -> Seed {
        return Seed {
            database_id,
            view_id,
            value,
            idxes: Vec::new(),
            invalid: false,
            error: "".to_string(),
        };
    }
    fn database_id(&self) -> String {
        self.database_id.clone()
    }
    fn view_id(&self) -> String {
        self.view_id.clone()
    }
    pub fn u8s(index_file_path: String, next_node_seek: u64, start: u64) -> GeorgeResult<Vec<u8>> {
        let trans = Idx {
            index_file_path,
            next_node_seek,
            start,
        };
        match serde_json::to_vec(&trans) {
            Ok(v8s) => Ok(v8s),
            Err(err) => Err(err_string(err.to_string())),
        }
    }

    pub fn seek_value(mut seek_bytes: Vec<u8>) -> (u64, Vec<u8>) {
        let sequence_id_bytes = seek_bytes.as_slice()[0..8].to_vec();
        let sequence_id = trans_bytes_2_u64(sequence_id_bytes);
        (sequence_id, seek_bytes.split_off(8))
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
    fn modify(&mut self, value: Vec<u8>) {
        if self.invalid {
            return;
        }
        match trans(value) {
            Ok(trans) => self.idxes.push(trans),
            Err(err) => {
                self.invalid = true;
                self.error = err.to_string()
            }
        }
    }
    fn save(&mut self, value: Vec<u8>) -> GeorgeResult<()> {
        if self.idxes.len() == 0 {
            return Ok(());
        }
        let seed_bytes = seed_bytes(value);
        // 将数据存入view，返回数据在view中的坐标
        let seek = GLOBAL_WRITER.write_append_bytes(
            Tag::View,
            store_view_id(self.database_id(), self.view_id()),
            seed_bytes.clone(),
        )?;
        let seek_v = trans_u64_2_bytes(seek);
        // 将在数据在view中的坐标存入各个index
        for idx in self.idxes.to_vec() {
            write_seek_u8s(
                idx.index_file_path,
                idx.start + idx.next_node_seek,
                seek_v.clone().as_slice(),
            )?;
        }
        Ok(())
    }

    fn remove(&mut self) -> GeorgeResult<()> {
        if self.idxes.len() == 0 {
            return Ok(());
        }
        // 将在数据在view中的坐标存入各个index
        for idx in self.idxes.to_vec() {
            write_seek_u8s(
                idx.index_file_path,
                idx.start + idx.next_node_seek,
                &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            )?;
        }
        Ok(())
    }
}
