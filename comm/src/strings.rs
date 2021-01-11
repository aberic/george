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

pub(crate) fn sub_string(comment: String, begin: usize, end: usize) -> String {
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

/// 字符串左边补齐0，长度为len
pub(crate) fn left_fit(mut comment: String, ch: char, len: usize) -> String {
    let mut comment_len = comment.len();
    if comment_len < len {
        while comment_len < len {
            comment = format!("{}{}", ch, comment);
            comment_len += 1
        }
    }
    comment
}

/// 字符串左边删除0
pub(crate) fn left_un_fit(comment: String, ch: char) -> String {
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

/// 字符串右边补齐0，长度为len
pub(crate) fn right_zero(mut comment: String, len: usize) -> String {
    let mut comment_len = comment.len();
    if comment_len < len {
        while comment_len < len {
            comment.push_str("0");
            comment_len += 1
        }
    }
    comment
}

/// 获取重复len次repeated的字符串
pub fn repeated_string(repeated: &str, len: usize) -> String {
    let mut res = String::new();
    let mut position = 0;
    while position < len {
        res.push_str(repeated);
        position += 1
    }
    res
}
