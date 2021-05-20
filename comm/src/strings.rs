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

use crate::errors::entrances::{Errs, GeorgeResult};

pub trait StringHandler {
    /// 字符串截取
    fn sub(comment: &str, begin: usize, end: usize) -> String;
    /// 字符串截取
    fn subs(comment: String, begin: usize, end: usize) -> String;
    /// 字符串左边补齐字符，长度为len
    ///
    /// comment 待补齐字符串
    ///
    /// ch 补齐字符
    ///
    /// len 期望补齐后的总长度
    fn left_fit(comment: &str, ch: char, len: usize) -> String;
    /// 字符串左边补齐字符，长度为len
    ///
    /// comment 待补齐字符串
    ///
    /// ch 补齐字符
    ///
    /// len 期望补齐后的总长度
    fn left_fits(comment: String, ch: char, len: usize) -> String;
    /// 字符串左边删除字符
    ///
    /// comment 待操作字符串
    ///
    /// ch 待删除字符
    fn left_un_fit(comment: &str, ch: char) -> String;
    /// 字符串左边删除字符
    ///
    /// comment 待操作字符串
    ///
    /// ch 待删除字符
    fn left_un_fits(comment: String, ch: char) -> String;
    /// 字符串右边补齐字符，长度为len
    ///
    /// comment 待补齐字符串
    ///
    /// ch 补齐字符
    ///
    /// len 期望补齐后的总长度
    fn right_fit(comment: &str, ch: char, len: usize) -> String;
    /// 字符串右边补齐字符，长度为len
    ///
    /// comment 待补齐字符串
    ///
    /// ch 补齐字符
    ///
    /// len 期望补齐后的总长度
    fn right_fits(comment: String, ch: char, len: usize) -> String;
    /// 获取重复len次repeated的字符串
    fn repeater(repeated: &str, len: usize) -> String;
    /// 获取重复len次repeated的字符串
    fn repeaters(repeated: String, len: usize) -> String;
    fn from_utf8(data: Vec<u8>) -> GeorgeResult<String>;
}

pub struct Strings {}

impl StringHandler for Strings {
    fn sub(comment: &str, begin: usize, end: usize) -> String {
        sub_string(comment.to_string(), begin, end)
    }
    fn subs(comment: String, begin: usize, end: usize) -> String {
        sub_string(comment, begin, end)
    }
    fn left_fit(comment: &str, ch: char, len: usize) -> String {
        left_fit_string(comment.to_string(), ch, len)
    }
    fn left_fits(comment: String, ch: char, len: usize) -> String {
        left_fit_string(comment, ch, len)
    }
    fn left_un_fit(comment: &str, ch: char) -> String {
        left_un_fit_string(comment.to_string(), ch)
    }
    fn left_un_fits(comment: String, ch: char) -> String {
        left_un_fit_string(comment, ch)
    }
    fn right_fit(comment: &str, ch: char, len: usize) -> String {
        right_fit_string(comment.to_string(), ch, len)
    }
    fn right_fits(comment: String, ch: char, len: usize) -> String {
        right_fit_string(comment, ch, len)
    }
    fn repeater(repeated: &str, len: usize) -> String {
        repeated_string(repeated, len)
    }
    fn repeaters(repeated: String, len: usize) -> String {
        repeated_string(repeated.as_str(), len)
    }
    fn from_utf8(data: Vec<u8>) -> GeorgeResult<String> {
        from_utf8(data)
    }
}

fn sub_string(comment: String, begin: usize, end: usize) -> String {
    let mut s = String::new();
    let mut position: usize = 0;
    let chs = comment.chars();
    for ch in chs.into_iter() {
        if position >= begin && position < end {
            s.push(ch)
        }
        position += 1
    }
    s
}

/// 字符串左边补齐字符，长度为len
fn left_fit_string(mut comment: String, ch: char, len: usize) -> String {
    let mut comment_len = comment.len();
    if comment_len < len {
        while comment_len < len {
            comment = format!("{}{}", ch, comment);
            comment_len += 1
        }
    }
    comment
}

/// 字符串左边删除字符
fn left_un_fit_string(comment: String, ch: char) -> String {
    let mut s = String::new();
    let mut end = false;
    let chs = comment.chars();
    for cha in chs.into_iter() {
        if end {
            s.push(cha)
        } else {
            if cha.eq(&ch) {
                continue;
            }
            end = true;
            s.push(cha)
        }
    }
    s
}

/// 字符串右边补齐字符，长度为len
fn right_fit_string(mut comment: String, ch: char, len: usize) -> String {
    let mut comment_len = comment.len();
    if comment_len < len {
        while comment_len < len {
            comment.push(ch);
            comment_len += 1
        }
    }
    comment
}

/// 获取重复len次repeated的字符串
fn repeated_string(repeated: &str, len: usize) -> String {
    let mut res = String::new();
    let mut position = 0;
    while position < len {
        res.push_str(repeated);
        position += 1
    }
    res
}

fn from_utf8(data: Vec<u8>) -> GeorgeResult<String> {
    match String::from_utf8(data) {
        Ok(res) => Ok(res),
        Err(err) => Err(Errs::strs("string from utf8", err)),
    }
}
