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

#[cfg(test)]
mod md5 {
    use crate::cryptos::hash::{HashCRCHandler, HashCRCTypeHandler, HashMD5Handler};
    use crate::cryptos::Hash;

    #[test]
    fn md5_test() {
        let str = "test".to_string();
        let md5_1 = Hash::md5(str.clone());
        let md5_2 = Hash::md5(str.clone());
        println!("test md5 1 = {}", md5_1);
        println!("test md5 2 = {}", md5_2);
        let md516_1 = Hash::md516(str.clone());
        let md516_2 = Hash::md516(str.clone());
        println!("test md516 1 = {}", md516_1);
        println!("test md516 2 = {}", md516_2);
    }

    #[test]
    fn sm3_test() {
        let str = "test".to_string();
        let str_u8s = "test".as_bytes();
        let str_v8s = "test".as_bytes().to_vec();

        let sm3_1 = Hash::sm3(str.clone());
        let sm3_2 = Hash::sm3(str.clone());
        let sm3_3 = Hash::sm3(str_u8s);
        let sm3_4 = Hash::sm3(str_v8s.clone());
        println!("test sm3 1 = {}", sm3_1);
        println!("test sm3 2 = {}", sm3_2);
        println!("test sm3 3 = {}", sm3_3);
        println!("test sm3 4 = {}", sm3_4);
        println!("test sm3 5 = {}", sm3_4);
    }

    #[test]
    fn sha1_test() {
        let str = "test".to_string();
        let str_u8s = "test".as_bytes();
        let str_v8s = "test".as_bytes().to_vec();

        let sm3_1 = Hash::sha1(str.clone());
        let sm3_2 = Hash::sha1(str.clone());
        let sm3_3 = Hash::sha1(str_u8s);
        let sm3_4 = Hash::sha1(str_v8s.clone());
        println!("test sm3 1 = {}", sm3_1);
        println!("test sm3 2 = {}", sm3_2);
        println!("test sm3 3 = {}", sm3_3);
        println!("test sm3 4 = {}", sm3_4);
        println!("test sm3 5 = {}", sm3_4);
    }

    #[test]
    fn sha256_test() {
        let str = "test".to_string();
        let str_u8s = "test".as_bytes();
        let str_v8s = "test".as_bytes().to_vec();

        let sm3_1 = Hash::sha256(str.clone());
        let sm3_2 = Hash::sha256(str.clone());
        let sm3_3 = Hash::sha256(str_u8s);
        let sm3_4 = Hash::sha256(str_v8s.clone());
        println!("test sm3 1 = {}", sm3_1);
        println!("test sm3 2 = {}", sm3_2);
        println!("test sm3 3 = {}", sm3_3);
        println!("test sm3 4 = {}", sm3_4);
        println!("test sm3 5 = {}", sm3_4);
    }

    #[test]
    fn hashcode32_test1() {
        let bytes1 = "test1".as_bytes();
        let bytes2 = "test2".as_bytes();

        println!("bytes1 = {}", Hash::crc32(bytes1));
        println!("bytes1u64 = {}", Hash::crc32(bytes1) as u64);
        println!("bytes2 = {}", Hash::crc32(bytes2));
        println!("bytes2u64 = {}", Hash::crc32(bytes2) as u64);
    }

    #[test]
    fn hashcode32_test2() {
        let x1: i32 = -1;
        let x2: i32 = -2;
        let x3: i32 = -3;
        let x4: i32 = 301047507;
        let x5: i32 = 1;
        let x6: i64 = 1;
        let x1b = x1.to_be_bytes();
        let x2b = x2.to_be_bytes();
        let x3b = x3.to_be_bytes();
        let x4b = x4.to_be_bytes();
        let x5b = x5.to_be_bytes();
        let x6b = x6.to_be_bytes();

        println!(
            "x1b = {}, bytes = {:#?}",
            Hash::crc32(x1b.to_vec().as_slice()),
            x1b
        );
        println!(
            "x2b = {}, bytes = {:#?}",
            Hash::crc32(x2b.to_vec().as_slice()),
            x2b
        );
        println!(
            "x3b = {}, bytes = {:#?}",
            Hash::crc32(x3b.to_vec().as_slice()),
            x3b
        );
        println!(
            "x4b = {}, bytes = {:#?}",
            Hash::crc32(x4b.to_vec().as_slice()),
            x4b
        );
        println!(
            "x5b = {}, bytes = {:#?}",
            Hash::crc32(x5b.to_vec().as_slice()),
            x5b
        );
        println!(
            "x6b = {}, bytes = {:#?}",
            Hash::crc32(x6b.to_vec().as_slice()),
            x6b
        );

        println!("x1b {} | x1 {}", Hash::crc32(x1b.to_vec().as_slice()), x1);
        println!("x2b {} | x2 {}", Hash::crc32(x2b.to_vec().as_slice()), x2);
        println!("x3b {} | x3 {}", Hash::crc32(x3b.to_vec().as_slice()), x3);
        println!("x4b {} | x4 {}", Hash::crc32(x4b.to_vec().as_slice()), x4);
        println!("x5b {} | x5 {}", Hash::crc32(x5b.to_vec().as_slice()), x5);
        println!("x6b {} | x6 {}", Hash::crc32(x6b.to_vec().as_slice()), x6);
    }

    #[test]
    fn hashcode64_test1() {
        let bytes1 = "test1".as_bytes();
        let bytes2 = "test2".as_bytes();
        let bytes3 = &[0x00];
        let bytes4 = &0_i32.to_be_bytes();
        let bytes41 = &1_i32.to_be_bytes();
        let i: i32 = -1;
        let bytes42 = &i.to_be_bytes();
        let bytes5 = &0_u64.to_be_bytes();
        let bytes6 = &[0x01];
        let bytes7 = &1_u32.to_be_bytes();
        let bytes8 = &1_i64.to_be_bytes();
        let bytes9 = &[0xff];
        let bytes10 = &16_u64.to_be_bytes();
        let bytes11 = &16_f64.to_be_bytes();

        println!("res1 = {}", Hash::crc64(bytes1));
        println!("res2 = {}", Hash::crc64(bytes2));
        println!("res3 = {}", Hash::crc64(bytes3.to_vec().as_slice()));
        println!("res4 = {}", Hash::crc64(bytes4.to_vec().as_slice()));
        println!("res41 = {}", Hash::crc64(bytes41.to_vec().as_slice()));
        println!("res42 = {}", Hash::crc64(bytes42.to_vec().as_slice()));
        println!("res5 = {}", Hash::crc64(bytes5.to_vec().as_slice()));
        println!("res6 = {}", Hash::crc64(bytes6.to_vec().as_slice()));
        println!("res7 = {}", Hash::crc64(bytes7.to_vec().as_slice()));
        println!("res8 = {}", Hash::crc64(bytes8.to_vec().as_slice()));
        println!("res9 = {}", Hash::crc64(bytes9.to_vec().as_slice()));
        println!("res10 = {}", Hash::crc64(bytes10.to_vec().as_slice()));
        println!("res11 = {}", Hash::crc64(bytes11.to_vec().as_slice()));
    }

    #[test]
    fn hashcode64_test2() {
        let x1: i64 = -1;
        let x2: i64 = -2;
        let x3: i64 = -3;
        let x4: i64 = 301047507;
        let x5: i64 = 1;
        let x1b = x1.to_be_bytes();
        let x2b = x2.to_be_bytes();
        let x3b = x3.to_be_bytes();
        let x4b = x4.to_be_bytes();
        let x5b = x5.to_be_bytes();

        println!(
            "x1b = {}, bytes = {:#?}",
            Hash::crc64(x1b.to_vec().as_slice()),
            x1b
        );
        println!(
            "x2b = {}, bytes = {:#?}",
            Hash::crc64(x2b.to_vec().as_slice()),
            x2b
        );
        println!(
            "x3b = {}, bytes = {:#?}",
            Hash::crc64(x3b.to_vec().as_slice()),
            x3b
        );
        println!(
            "x4b = {}, bytes = {:#?}",
            Hash::crc64(x4b.to_vec().as_slice()),
            x4b
        );
        println!(
            "x5b = {}, bytes = {:#?}",
            Hash::crc64(x5b.to_vec().as_slice()),
            x5b
        );
    }

    #[test]
    fn hashcode64_test3() {
        let sbl1 = String::from("true");
        let sbl2 = String::from("false");
        println!(
            "u64 = {}, sbl1 = {}",
            Hash::crc64_bool(sbl1.clone()).unwrap(),
            sbl1.clone()
        );
        println!(
            "u64 = {}, sbl2 = {}",
            Hash::crc64_bool(sbl2.clone()).unwrap(),
            sbl2.clone()
        );

        println!();

        let sf640 = String::from("-123.34523412");
        let sf641 = String::from("-0.1");
        let sf642 = String::from("-0.0");
        let sf643 = String::from("0.0");
        let sf644 = String::from("0.00");
        let sf645 = String::from("0.0000000000000000000000000001");
        let sf646 = String::from("0.000000000000000000000000001");
        let sf647 = String::from("0.00000000000000000000000001");
        let sf648 = String::from("0.0000000000000000000000001");
        let sf649 = String::from("1.0");
        let sf6410 = String::from("1.34523411233211");
        let sf6411 = String::from("12.3452341");
        let sf6412 = String::from("12.34523411");
        let sf6413 = String::from("12.345234115");
        let sf6414 = String::from("12.345234119");
        let sf6415 = String::from("12.34523412");
        let sf6416 = String::from("12.345234123");
        let sf6417 = String::from("123.34523412");
        println!(
            "u64 = {}, 0 = {}",
            Hash::crc64_f64(sf640.clone()).unwrap(),
            sf640.clone()
        );
        println!(
            "u64 = {}, 1 = {}",
            Hash::crc64_f64(sf641.clone()).unwrap(),
            sf641.clone()
        );
        println!(
            "u64 = {}, 2 = {}",
            Hash::crc64_f64(sf642.clone()).unwrap(),
            sf642.clone()
        );
        println!(
            "u64 = {}, 3 = {}",
            Hash::crc64_f64(sf643.clone()).unwrap(),
            sf643.clone()
        );
        println!(
            "u64 = {}, 4 = {}",
            Hash::crc64_f64(sf644.clone()).unwrap(),
            sf644.clone()
        );
        println!(
            "u64 = {}, 5 = {}",
            Hash::crc64_f64(sf645.clone()).unwrap(),
            sf645.clone()
        );
        println!(
            "u64 = {}, 6 = {}",
            Hash::crc64_f64(sf646.clone()).unwrap(),
            sf646.clone()
        );
        println!(
            "u64 = {}, 7 = {}",
            Hash::crc64_f64(sf647.clone()).unwrap(),
            sf647.clone()
        );
        println!(
            "u64 = {}, 8 = {}",
            Hash::crc64_f64(sf648.clone()).unwrap(),
            sf648.clone()
        );
        println!(
            "u64 = {}, 9 = {}",
            Hash::crc64_f64(sf649.clone()).unwrap(),
            sf649.clone()
        );
        println!(
            "u64 = {}, 10 = {}",
            Hash::crc64_f64(sf6410.clone()).unwrap(),
            sf6410.clone()
        );
        println!(
            "u64 = {}, 11 = {}",
            Hash::crc64_f64(sf6411.clone()).unwrap(),
            sf6411.clone()
        );
        println!(
            "u64 = {}, 12 = {}",
            Hash::crc64_f64(sf6412.clone()).unwrap(),
            sf6412.clone()
        );
        println!(
            "u64 = {}, 13 = {}",
            Hash::crc64_f64(sf6413.clone()).unwrap(),
            sf6413.clone()
        );
        println!(
            "u64 = {}, 14 = {}",
            Hash::crc64_f64(sf6414.clone()).unwrap(),
            sf6414.clone()
        );
        println!(
            "u64 = {}, 15 = {}",
            Hash::crc64_f64(sf6415.clone()).unwrap(),
            sf6415.clone()
        );
        println!(
            "u64 = {}, 16 = {}",
            Hash::crc64_f64(sf6416.clone()).unwrap(),
            sf6416.clone()
        );
        println!(
            "u64 = {}, 17 = {}",
            Hash::crc64_f64(sf6417.clone()).unwrap(),
            sf6417.clone()
        );

        println!();

        let i0 = String::from("-9223372036854775808");
        let i1 = String::from("-9223372036854775807");
        let i2 = String::from("-1");
        let i3 = String::from("-0");
        let i4 = String::from("0");
        let i5 = String::from("1");
        let i6 = String::from("9223372036854775805");
        let i7 = String::from("9223372036854775806");
        println!(
            "i64 = {}, i0 = {}",
            Hash::crc64_i64(i0.clone()).unwrap(),
            i0.clone()
        );
        println!(
            "i64 = {}, i1 = {}",
            Hash::crc64_i64(i1.clone()).unwrap(),
            i1.clone()
        );
        println!(
            "i64 = {}, i2 = {}",
            Hash::crc64_i64(i2.clone()).unwrap(),
            i2.clone()
        );
        println!(
            "i64 = {}, i3 = {}",
            Hash::crc64_i64(i3.clone()).unwrap(),
            i3.clone()
        );
        println!(
            "i64 = {}, i4 = {}",
            Hash::crc64_i64(i4.clone()).unwrap(),
            i4.clone()
        );
        println!(
            "i64 = {}, i5 = {}",
            Hash::crc64_i64(i5.clone()).unwrap(),
            i5.clone()
        );
        println!(
            "i64 = {}, i6 = {}",
            Hash::crc64_i64(i6.clone()).unwrap(),
            i6.clone()
        );
        println!(
            "i64 = {}, i7 = {}",
            Hash::crc64_i64(i7.clone()).unwrap(),
            i7.clone()
        );
    }

    #[test]
    fn hashcode64_test() {
        let t1 = String::from("test1");
        let t2 = String::from("test2");
        let bytes1 = "test1".as_bytes();
        let bytes2 = "test2".as_bytes();
        let x1 = String::from("-1");
        let x2 = String::from("-2094967294");
        let x3 = String::from("-8446744073709551615");
        let x4 = String::from("18446744073709551615");

        println!("bytes1 = {}", Hash::crc64(bytes1));
        println!("t1 = {}", Hash::crc64(t1));
        println!("bytes2 = {}", Hash::crc64(bytes2));
        println!("t2 = {}", Hash::crc64(t2));
        println!("x1 = {}", Hash::crc64(x1));
        println!("x2 = {}", Hash::crc64(x2));
        println!("x3 = {}", Hash::crc64(x3));
        println!("x4 = {}", Hash::crc64(x4));

        let m: u64 = 1 << 63;
        println!("2^64 = {}", m);

        let uu: u32 = 1988888;
        let uu64 = uu as u64;
        let uu32 = uu64 as u32;
        println!("u1 = {}", uu);
        println!("u2 = {}", uu64);
        println!("u3 = {}", uu32);
    }
}
