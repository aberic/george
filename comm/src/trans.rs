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

use std::collections::HashMap;

use phf::{phf_map, Map};

use crate::errors::entrances::{err_string, GeorgeResult};
use crate::strings::{StringHandler, Strings};
use std::ops::{Add, Sub};

/// 十进制对应64进制映射
static STRING_2_U64_MAP: Map<&'static str, u64> = phf_map! {
    "0"=>0, "1"=>1, "2"=>2, "3"=>3,
    "4"=>4, "5"=>5, "6"=>6, "7"=>7,
    "8"=>8, "9"=>9, "a"=>10, "b"=>11,
    "c"=>12, "d"=>13, "e"=>14, "f"=>15,
    "g"=>16, "h"=>17, "i"=>18, "j"=>19,
    "k"=>20, "l"=>21, "m"=>22, "n"=>23,
    "o"=>24, "p"=>25, "q"=>26, "r"=>27,
    "s"=>28, "t"=>29, "u"=>30, "v"=>31,
    "w"=>32, "x"=>33, "y"=>34, "z"=>35,
    "A"=>36, "B"=>37, "C"=>38, "D"=>39,
    "E"=>40, "F"=>41, "G"=>42, "H"=>43,
    "I"=>44, "J"=>45, "K"=>46, "L"=>47,
    "M"=>48, "N"=>49, "O"=>50, "P"=>51,
    "Q"=>52, "R"=>53, "S"=>54, "T"=>55,
    "U"=>56, "V"=>57, "W"=>58, "X"=>59,
    "Y"=>60, "Z"=>61, "+"=>62, "-"=>63,
};

// 64进制对应十进制映射
lazy_static! {
    static ref U64_2_STRING_MAP: HashMap<u64, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0, "0");
        map.insert(1, "1");
        map.insert(2, "2");
        map.insert(3, "3");
        map.insert(4, "4");
        map.insert(5, "5");
        map.insert(6, "6");
        map.insert(7, "7");
        map.insert(8, "8");
        map.insert(9, "9");
        map.insert(10, "a");
        map.insert(11, "b");
        map.insert(12, "c");
        map.insert(13, "d");
        map.insert(14, "e");
        map.insert(15, "f");
        map.insert(16, "g");
        map.insert(17, "h");
        map.insert(18, "i");
        map.insert(19, "j");
        map.insert(20, "k");
        map.insert(21, "l");
        map.insert(22, "m");
        map.insert(23, "n");
        map.insert(24, "o");
        map.insert(25, "p");
        map.insert(26, "q");
        map.insert(27, "r");
        map.insert(28, "s");
        map.insert(29, "t");
        map.insert(30, "u");
        map.insert(31, "v");
        map.insert(32, "w");
        map.insert(33, "x");
        map.insert(34, "y");
        map.insert(35, "z");
        map.insert(36, "A");
        map.insert(37, "B");
        map.insert(38, "C");
        map.insert(39, "D");
        map.insert(40, "E");
        map.insert(41, "F");
        map.insert(42, "G");
        map.insert(43, "H");
        map.insert(44, "I");
        map.insert(45, "J");
        map.insert(46, "K");
        map.insert(47, "L");
        map.insert(48, "M");
        map.insert(49, "N");
        map.insert(50, "O");
        map.insert(51, "P");
        map.insert(52, "Q");
        map.insert(53, "R");
        map.insert(54, "S");
        map.insert(55, "T");
        map.insert(56, "U");
        map.insert(57, "V");
        map.insert(58, "W");
        map.insert(59, "X");
        map.insert(60, "Y");
        map.insert(61, "Z");
        map.insert(62, "+");
        map.insert(63, "-");
        map
    };
}

/// u64转64进制字符串<p><p>
///
/// 最大值2^64=18446744073709551616，64进制字符串为11位“-----------”
pub fn trans_u64_2_string64(mut uint64: u64) -> String {
    let mut res = String::new();
    while uint64 > 0 {
        if uint64 >= 64 {
            res.push_str(U64_2_STRING_MAP.get(&(uint64 % 64)).unwrap());
            uint64 /= 64;
        } else if uint64 > 0 && uint64 < 64 {
            res.push_str(U64_2_STRING_MAP.get(&uint64).unwrap());
            break;
        }
    }
    res
}

/// 64进制字符串转u64
pub fn trans_string64_2_u64(mut string64: String) -> u64 {
    let str_len = string64.len();
    let mut res: u64 = 0;
    let mut i = 0;
    while i < str_len {
        res += STRING_2_U64_MAP
            .get(string64.pop().unwrap().to_string().as_str())
            .unwrap()
            * 1
            << (6 * (str_len - i - 1));
        i += 1;
    }
    res
}

/// u32转64进制字符串<p><p>
///
/// 最大值2^32=4294967295，64进制字符串为6位“------”
pub fn trans_u32_2_string64(uint32: u32) -> String {
    trans_u64_2_string64(uint32 as u64)
}

/// 64进制字符串转u32
pub fn trans_string64_2_u32(string64: String) -> u32 {
    trans_string64_2_u64(string64) as u32
}

/// u32转64进制字符串<p><p>
///
/// 最大值2^32=4294967295，64进制字符串为6位“------”
///
/// 左侧补齐，保证总长度是6
pub fn trans_u32_2_string64_fit(uint32: u32) -> String {
    Strings::left_fits(trans_u32_2_string64(uint32), "*".parse().unwrap(), 6)
}

/// 64进制字符串转u32
///
/// 删除左侧多余
pub fn trans_string64_2_u32_fit(string64: String) -> u32 {
    trans_string64_2_u64(Strings::left_un_fits(string64, "*".parse().unwrap())) as u32
}

/// u64转16进制数组
pub fn trans_u64_2_bytes(uint64: u64) -> Vec<u8> {
    let mut bs: Vec<u8> = vec![];
    bs.push(((uint64 >> 56) & 0xFF) as u8);
    bs.push(((uint64 >> 48) & 0xFF) as u8);
    bs.push(((uint64 >> 40) & 0xFF) as u8);
    bs.push(((uint64 >> 32) & 0xFF) as u8);
    bs.push(((uint64 >> 24) & 0xFF) as u8);
    bs.push(((uint64 >> 16) & 0xFF) as u8);
    bs.push(((uint64 >> 8) & 0xFF) as u8);
    bs.push((uint64 & 0xFF) as u8);
    bs
}

/// 16进制数组转u64
///
/// 字节数组长度不得超过8，超过将溢出
pub fn trans_bytes_2_u64(bs: Vec<u8>) -> GeorgeResult<u64> {
    let bs_len = bs.len();
    if bs_len > 8 {
        Err(err_string(format!(
            "trans bytes 2 u16 out of bounds, except le 8, but receive {}",
            bs_len
        )))
    } else {
        let mut res: u64 = 0;
        let mut i = 0;
        while i < bs_len {
            res += (bs[i] as u64) * 1 << (8 * (bs_len - i - 1));
            i += 1;
        }
        Ok(res)
    }
}

/// u48转16进制数组
pub fn trans_u48_2_bytes(uint64: u64) -> Vec<u8> {
    let mut bs: Vec<u8> = vec![];
    bs.push(((uint64 >> 40) & 0xFF) as u8);
    bs.push(((uint64 >> 32) & 0xFF) as u8);
    bs.push(((uint64 >> 24) & 0xFF) as u8);
    bs.push(((uint64 >> 16) & 0xFF) as u8);
    bs.push(((uint64 >> 8) & 0xFF) as u8);
    bs.push((uint64 & 0xFF) as u8);
    bs
}

/// 16进制数组转u48
///
/// 字节数组长度不得超过6，超过将溢出
pub fn trans_bytes_2_u48(bs: Vec<u8>) -> GeorgeResult<u64> {
    let bs_len = bs.len();
    if bs_len > 6 {
        Err(err_string(format!(
            "trans bytes 2 u16 out of bounds, except le 6, but receive {}",
            bs_len
        )))
    } else {
        let mut res: u64 = 0;
        let mut i = 0;
        while i < bs_len {
            res += (bs[i] as u64) * 1 << (8 * (bs_len - i - 1));
            i += 1;
        }
        Ok(res)
    }
}

/// u40转16进制数组
pub fn trans_u40_2_bytes(uint64: u64) -> Vec<u8> {
    let mut bs: Vec<u8> = vec![];
    bs.push(((uint64 >> 32) & 0xFF) as u8);
    bs.push(((uint64 >> 24) & 0xFF) as u8);
    bs.push(((uint64 >> 16) & 0xFF) as u8);
    bs.push(((uint64 >> 8) & 0xFF) as u8);
    bs.push((uint64 & 0xFF) as u8);
    bs
}

/// 16进制数组转u40
///
/// 字节数组长度不得超过6，超过将溢出
pub fn trans_bytes_2_u40(bs: Vec<u8>) -> GeorgeResult<u64> {
    let bs_len = bs.len();
    if bs_len > 5 {
        Err(err_string(format!(
            "trans bytes 2 u16 out of bounds, except le 5, but receive {}",
            bs_len
        )))
    } else {
        let mut res: u64 = 0;
        let mut i = 0;
        while i < bs_len {
            res += (bs[i] as u64) * 1 << (8 * (bs_len - i - 1));
            i += 1;
        }
        Ok(res)
    }
}

/// u32转16进制数组
pub fn trans_u32_2_bytes(uint32: u32) -> Vec<u8> {
    let mut bs: Vec<u8> = vec![];
    bs.push(((uint32 >> 24) & 0xFF) as u8);
    bs.push(((uint32 >> 16) & 0xFF) as u8);
    bs.push(((uint32 >> 8) & 0xFF) as u8);
    bs.push((uint32 & 0xFF) as u8);
    bs
}

/// 16进制数组转u32
///
/// 字节数组长度不得超过4，超过将溢出
pub fn trans_bytes_2_u32(bs: Vec<u8>) -> GeorgeResult<u32> {
    let bs_len = bs.len();
    if bs_len > 4 {
        Err(err_string(format!(
            "trans bytes 2 u16 out of bounds, except le 4, but receive {}",
            bs_len
        )))
    } else {
        let mut res: u32 = 0;
        let mut i = 0;
        while i < bs_len {
            res += (bs[i] as u32) * 1 << (8 * (bs_len - i - 1));
            i += 1;
        }
        Ok(res)
    }
}

/// 16进制数组转u32，但返回u64
///
/// 字节数组长度不得超过4，超过将溢出
pub fn trans_bytes_2_u32_as_u64(bs: Vec<u8>) -> GeorgeResult<u64> {
    let bs_len = bs.len();
    if bs_len > 4 {
        Err(err_string(format!(
            "trans bytes 2 u16 out of bounds, except le 4, but receive {}",
            bs_len
        )))
    } else {
        let mut res: u64 = 0;
        let mut i = 0;
        while i < bs_len {
            res += (bs[i] as u64) * 1 << (8 * (bs_len - i - 1));
            i += 1;
        }
        Ok(res)
    }
}

/// u16转16进制数组
pub fn trans_u16_2_bytes(uint16: u16) -> Vec<u8> {
    let mut bs: Vec<u8> = vec![];
    bs.push(((uint16 >> 8) & 0xFF) as u8);
    bs.push((uint16 & 0xFF) as u8);
    bs
}

/// 16进制数组转u16
///
/// 字节数组长度不得超过2，超过将溢出
pub fn trans_bytes_2_u16(bs: Vec<u8>) -> GeorgeResult<u16> {
    let bs_len = bs.len();
    if bs_len > 2 {
        Err(err_string(format!(
            "trans bytes 2 u16 out of bounds, except le 2, but receive {}",
            bs_len
        )))
    } else {
        let mut res: u16 = 0;
        let mut i = 0;
        while i < bs_len {
            res += (bs[i] as u16) * 1 << (8 * (bs_len - i - 1));
            i += 1;
        }
        Ok(res)
    }
}

pub fn trans_i64_2_u64(res: i64) -> u64 {
    if res >= 0 {
        (res as u64).add(9223372036854775809)
    } else {
        (res as u64).sub(9223372036854775807)
    }
}

pub fn trans_i32_2_u64(res: i32) -> u64 {
    if res >= 0 {
        (res as u64).add(2147483649)
    } else {
        (res as u64).sub(2147483647)
    }
}
