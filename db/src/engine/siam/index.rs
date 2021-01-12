/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use chrono::{Duration, Local, NaiveDateTime};

use comm::errors::entrances::err_string;
use comm::errors::entrances::GeorgeResult;
use comm::io::file::create_file;
use comm::trans::{trans_bytes_2_u16, trans_u16_2_bytes};
use comm::vectors;

use crate::engine::siam::selector::{Constraint, Expectation};
use crate::engine::siam::traits::TNode;
use crate::engine::traits::{TDescription, TIndex, TSeed};
use crate::utils::comm::{Category, IndexMold, IndexType, LevelType};
use crate::utils::path;
use crate::utils::store::{
    before_content_bytes_for_index, category, category_u8, head, mold, mold_u8, save,
    store_index_id, FileHeader, Tag,
};
use serde_json::{Error, Value};

/// Siam索引
///
/// 5位key及16位md5后key及5位起始seek和4位持续seek
#[derive(Debug)]
pub struct Index<N: TNode>
where
    N: Debug,
{
    /// 库ID
    database_id: String,
    /// 视图ID
    view_id: String,
    /// 索引唯一ID
    id: String,
    /// 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
    name: String,
    /// 是否主键
    primary: bool,
    /// 结点
    root: Arc<N>,
    /// 类型
    category: Category,
    /// 索引值类型
    mold: IndexMold,
    /// 创建时间
    create_time: Duration,
    /// 除此参数外的描述长度
    description_len: usize,
}

impl<N: TNode + Debug> TDescription for Index<N> {
    fn description(&mut self) -> Vec<u8> {
        let mut des: Vec<u8> = vec![];
        let mut des_front = hex::encode(format!(
            "{}/{}/{}/{}/{}/{}/{}/{}",
            self.database_id,
            self.view_id,
            self.id,
            self.primary,
            self.name,
            category_u8(self.category),
            mold_u8(self.mold),
            self.create_time.num_nanoseconds().unwrap().to_string(),
        ))
        .into_bytes();
        let len = des_front.len();
        self.description_len = len + 2 + 40;
        let mut len_bytes = trans_u16_2_bytes(len as u16);
        des.append(&mut len_bytes);
        des.append(&mut des_front);
        des.append(&mut self.root.clone().node_bytes().read().unwrap().to_vec());
        des
    }

    fn recover(&mut self, mut description: Vec<u8>) -> GeorgeResult<()> {
        let des_len: Vec<u8> = vectors::sub(description.clone(), 0, 2);
        let len = trans_bytes_2_u16(des_len) as usize;
        let des_front = vectors::sub(description.clone(), 2, len + 2);
        let description_len = len + 2;
        let node_bytes = description.split_off(description_len);
        match String::from_utf8(des_front) {
            Ok(description_str) => match hex::decode(description_str) {
                Ok(vu8) => match String::from_utf8(vu8) {
                    Ok(real) => {
                        let mut split = real.split("/");
                        self.database_id = split.next().unwrap().to_string();
                        self.view_id = split.next().unwrap().to_string();
                        self.id = split.next().unwrap().to_string();
                        self.primary = split.next().unwrap().to_string().parse::<bool>().unwrap();
                        self.name = split.next().unwrap().to_string();
                        self.category =
                            category(split.next().unwrap().to_string().parse::<u8>().unwrap());
                        self.mold = mold(split.next().unwrap().to_string().parse::<u8>().unwrap());
                        self.create_time = Duration::nanoseconds(
                            split.next().unwrap().to_string().parse::<i64>().unwrap(),
                        );
                        self.description_len = description_len + 40;
                        self.root.set_node_bytes(node_bytes);
                        Ok(())
                    }
                    Err(err) => Err(err_string(format!(
                        "recovery index from utf8 failed! error is {}",
                        err
                    ))),
                },
                Err(err) => Err(err_string(format!(
                    "recovery index decode failed! error is {}",
                    err
                ))),
            },
            Err(err) => Err(err_string(err.to_string())),
        }
    }
}

/// 新建索引
///
/// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
///
/// ###Params
///
/// index_name 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入
///
/// primary 是否主键
fn new_index<N: TNode + Debug>(
    database_id: String,
    view_id: String,
    id: String,
    index_name: String,
    primary: bool,
    root: Arc<N>,
    category: Category,
    mold: IndexMold,
) -> Index<N> {
    let now: NaiveDateTime = Local::now().naive_local();
    let create_time = Duration::nanoseconds(now.timestamp_nanos());
    return Index {
        database_id,
        view_id,
        id,
        primary,
        name: index_name,
        root,
        category,
        mold,
        create_time,
        description_len: 0,
    };
}

/// 封装方法函数
impl<N: TNode + Debug> TIndex for Index<N> {
    fn database_id(&self) -> String {
        self.database_id.clone()
    }
    fn view_id(&self) -> String {
        self.view_id.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
    fn is_primary(&self) -> bool {
        self.primary.clone()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn category(&self) -> Category {
        self.category
    }
    fn mold(&self) -> IndexMold {
        self.mold
    }
    fn create_time(&self) -> Duration {
        self.create_time
    }
    fn put(&self, key: String, seed: Arc<RwLock<dyn TSeed>>, force: bool) -> GeorgeResult<()> {
        self.root.put(key, seed, force, self.description_len)
    }
    fn get(&self, key: String) -> GeorgeResult<Vec<u8>> {
        self.root.get(key)
    }
    fn select(
        &self,
        left: bool,
        start: u64,
        end: u64,
        constraint: Constraint,
    ) -> GeorgeResult<Expectation> {
        let (total, count, mut values) =
            self.root
                .select(self.mold(), left, start, end, constraint.clone())?;
        match constraint.sort() {
            Some(sort) => {
                if self.name() != sort.param() {
                    values.sort_by(|a, b| match String::from_utf8(a.clone()) {
                        Ok(value_str_a) => match String::from_utf8(b.clone()) {
                            Ok(value_str_b) => {
                                let res_a: Result<Value, Error> =
                                    serde_json::from_str(value_str_a.as_ref());
                                let res_b: Result<Value, Error> =
                                    serde_json::from_str(value_str_b.as_ref());
                                match res_a {
                                    Ok(value_a) => match res_b {
                                        Ok(value_b) => {
                                            if sort.asc() {
                                                if value_a[sort.param()].is_i64()
                                                    && value_b[sort.param()].is_i64()
                                                {
                                                    if value_a[sort.param()].as_i64().unwrap()
                                                        > value_b[sort.param()].as_i64().unwrap()
                                                    {
                                                        a.cmp(b)
                                                    } else {
                                                        b.cmp(a)
                                                    }
                                                } else if value_a[sort.param()].is_u64()
                                                    && value_b[sort.param()].is_u64()
                                                {
                                                    if value_a[sort.param()].as_u64().unwrap()
                                                        > value_b[sort.param()].as_u64().unwrap()
                                                    {
                                                        a.cmp(b)
                                                    } else {
                                                        b.cmp(a)
                                                    }
                                                } else if value_a[sort.param()].is_f64()
                                                    && value_b[sort.param()].is_f64()
                                                {
                                                    if value_a[sort.param()].as_f64().unwrap()
                                                        > value_b[sort.param()].as_f64().unwrap()
                                                    {
                                                        a.cmp(b)
                                                    } else {
                                                        b.cmp(a)
                                                    }
                                                } else {
                                                    panic!(
                                                        "{} can't match each other when sort",
                                                        sort.param()
                                                    )
                                                }
                                            } else {
                                                if value_a[sort.param()].is_i64()
                                                    && value_b[sort.param()].is_i64()
                                                {
                                                    if value_a[sort.param()].as_i64().unwrap()
                                                        < value_b[sort.param()].as_i64().unwrap()
                                                    {
                                                        a.cmp(b)
                                                    } else {
                                                        b.cmp(a)
                                                    }
                                                } else if value_a[sort.param()].is_u64()
                                                    && value_b[sort.param()].is_u64()
                                                {
                                                    if value_a[sort.param()].as_u64().unwrap()
                                                        < value_b[sort.param()].as_u64().unwrap()
                                                    {
                                                        a.cmp(b)
                                                    } else {
                                                        b.cmp(a)
                                                    }
                                                } else if value_a[sort.param()].is_f64()
                                                    && value_b[sort.param()].is_f64()
                                                {
                                                    if value_a[sort.param()].as_f64().unwrap()
                                                        < value_b[sort.param()].as_f64().unwrap()
                                                    {
                                                        a.cmp(b)
                                                    } else {
                                                        b.cmp(a)
                                                    }
                                                } else {
                                                    panic!(
                                                        "{} can't match each other when sort",
                                                        sort.param()
                                                    )
                                                }
                                            }
                                        }
                                        _ => panic!("an unexpected mistake 4"),
                                    },
                                    _ => panic!("an unexpected mistake 3"),
                                }
                            }
                            _ => panic!("an unexpected mistake 2"),
                        },
                        _ => panic!("an unexpected mistake 1"),
                    });
                }
            }
            _ => {}
        }
        Ok(Expectation {
            total,
            count,
            index_name: self.name(),
            asc: left,
            values,
        })
    }
}

/// 封装方法函数
impl<N: TNode + Debug> Index<N> {
    /// 新建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// index_name 索引名称，可以自定义；<p>
    /// siam::Index 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入<p><p>
    ///
    /// primary 是否主键
    ///
    /// level 视图规模/级别
    pub(crate) fn init(
        database_id: String,
        view_id: String,
        id: String,
        index_name: String,
        primary: bool,
        root: Arc<N>,
        category: Category,
        mold: IndexMold,
    ) -> GeorgeResult<Arc<RwLock<Index<N>>>> {
        let mut index = new_index(
            database_id.clone(),
            view_id.clone(),
            id.clone(),
            index_name.clone(),
            primary,
            root,
            category,
            mold,
        );
        let index_file_path =
            path::index_file_path(database_id.clone(), view_id.clone(), id.clone());
        let file = create_file(index_file_path.clone(), true)?;
        let mut head = head(FileHeader::create(
            Tag::Index,
            Category::Document,
            LevelType::Small,
            IndexType::Siam,
            0x00,
        ));
        let mut description = index.description();
        // 初始化为32 + 8，即head长度加正文描述符长度
        let mut before_description = before_content_bytes_for_index(40, description.len() as u32);
        head.append(&mut before_description);
        head.append(&mut description);
        save(
            Tag::Index,
            file,
            head,
            store_index_id(database_id, view_id, id),
            index_file_path,
            index,
        )
    }

    /// 新建索引
    ///
    /// 该索引需要定义ID，此外索引所表达的字段组成内容也是必须的，并通过primary判断索引类型，具体传参参考如下定义：<p><p>
    ///
    /// ###Params
    ///
    /// index_name 索引名称，可以自定义；<p>
    /// siam::Index 索引名，新插入的数据将会尝试将数据对象转成json，并将json中的`index_name`作为索引存入<p><p>
    ///
    /// primary 是否主键
    ///
    /// level 视图规模/级别
    pub(crate) fn create(
        database_id: String,
        view_id: String,
        id: String,
        index_name: String,
        primary: bool,
        root: Arc<N>,
        category: Category,
        index_mold: IndexMold,
    ) -> Index<N> {
        new_index(
            database_id,
            view_id,
            id,
            index_name.clone(),
            primary,
            root,
            category,
            index_mold,
        )
    }

    /// 恢复index数据
    pub(crate) fn regain(
        description: Vec<u8>,
        root: Arc<N>,
    ) -> GeorgeResult<Arc<RwLock<Index<N>>>> {
        let mut index = Index {
            database_id: "".to_string(),
            view_id: "".to_string(),
            id: "".to_string(),
            primary: false,
            name: "".to_string(),
            root,
            category: Category::Memory,
            mold: IndexMold::String,
            create_time: Duration::nanoseconds(1),
            description_len: 0,
        };
        index.recover(description)?;
        log::info!(
            "recovery index {}({}.{}.{})",
            index.name(),
            index.database_id(),
            index.view_id(),
            index.id()
        );
        log::debug!(
            "index [dbID={}, vid={}, id={}, index_name={}, primary={}, category={:#?}, create_time={}]",
            index.database_id(),
            index.view_id(),
            index.id(),
            index.name(),
            index.is_primary(),
            index.category(),
            index.create_time().num_nanoseconds().unwrap().to_string(),
        );
        Ok(Arc::new(RwLock::new(index)))
    }
}
