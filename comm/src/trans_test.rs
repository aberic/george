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
mod strings {
    use crate::strings::{StringHandler, Strings};
    use crate::trans::{
        trans_bytes_2_u16, trans_bytes_2_u32, trans_bytes_2_u64, trans_string64_2_u32,
        trans_string64_2_u64, trans_u16_2_bytes, trans_u32_2_bytes, trans_u32_2_string64,
        trans_u32_2_string64_fit, trans_u64_2_bytes, trans_u64_2_string64,
    };

    #[test]
    fn trans_test1() {
        let x = 18446744073709551615;
        let xs = trans_u64_2_string64(x);
        println!("xs = {}", xs);
        let xs2u = trans_string64_2_u64(xs);
        println!("xs2u = {}", xs2u);

        println!();

        let y = 100;
        let ys = trans_u64_2_string64(y);
        println!("ys = {}", ys);
        let ys2u = trans_string64_2_u64(ys);
        println!("ys2u = {}", ys2u);

        let z = "000000".to_string();
        let z2u = trans_string64_2_u64(z);
        println!("z2u = {}", z2u);
        let z2us = trans_u64_2_string64(z2u);
        println!("z2us = {}", z2us);
    }

    #[test]
    fn trans_test2() {
        let x = 4294967295;
        let xs = trans_u32_2_string64(x);
        println!("xs = {}", xs);
        let xs2u = trans_string64_2_u32(xs);
        println!("xs2u = {}", xs2u);

        println!();

        let y = 100;
        let ys = trans_u32_2_string64(y);
        println!("ys = {}", ys);
        let ys2u = trans_string64_2_u32(ys);
        println!("ys2u = {}", ys2u);

        println!();

        let m = 4294967295;
        let ms = trans_u32_2_string64_fit(m);
        println!("ms = {}", ms);
        let ms2u = trans_string64_2_u32(ms);
        println!("ms2u = {}", ms2u);

        println!();

        let n = 100;
        let ns = trans_u32_2_string64_fit(n);
        println!("ns = {}", ns);
        let ns_un = Strings::left_un_fits(ns, "*".parse().unwrap());
        let ns2u = trans_string64_2_u32(ns_un);
        println!("ns2u = {}", ns2u);
    }

    #[test]
    fn trans_test3() {
        let x: u8 = 250;
        let x_str = x.to_string();
        println!("x = {}, x_str = {}", x, x_str);
        let y = x_str.parse::<u8>().unwrap();
        println!("y = {}", y);
        println!("x == y = {}", x == y);
    }

    #[test]
    fn trans_test4() {
        let a1: [u8; 8] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let a1u64 = trans_bytes_2_u64(a1.to_vec());
        let a1u64bs = hex::encode(trans_u64_2_bytes(a1u64));
        println!("a1u64 = {}, a1u64bs = {}", a1u64, a1u64bs);
        println!();

        let a2: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff];
        let a2u64 = trans_bytes_2_u64(a2.to_vec());
        let a2u64bs = hex::encode(trans_u64_2_bytes(a2u64));
        println!("a2u64 = {}, a2u64bs = {}", a2u64, a2u64bs);
        println!();

        let a3: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff];
        let a3u64 = trans_bytes_2_u64(a3.to_vec());
        let a3u64bs = hex::encode(trans_u64_2_bytes(a3u64));
        println!("a3u64 = {}, a3u64bs = {}", a3u64, a3u64bs);
        let a3u64bs2bs = hex::decode(a3u64bs).unwrap();
        println!("a3u64bs2bs = {:#?}", a3u64bs2bs);
        let a3u64_2 = trans_bytes_2_u64(a3u64bs2bs);
        println!("a3u64_2 = {}", a3u64_2);
        println!();

        let a4: [u8; 4] = [0x00, 0x00, 0x1f, 0xff];
        let a4u32 = trans_bytes_2_u32(a4.to_vec());
        let a4u32bs = hex::encode(trans_u32_2_bytes(a4u32));
        println!("a4u32 = {}, a4u32bs = {}", a4u32, a4u32bs);
        let a4u32bs2bs = hex::decode(a4u32bs).unwrap();
        println!("a4u32bs2bs = {:#?}", a4u32bs2bs);
        let a4u32_2 = trans_bytes_2_u32(a4u32bs2bs);
        println!("a4u32_2 = {}", a4u32_2);
        println!();

        let a5: [u8; 2] = [0xff, 0xff];
        let a5u16 = trans_bytes_2_u16(a5.to_vec());
        let a5u16bs = hex::encode(trans_u16_2_bytes(a5u16));
        println!("a5u16 = {}, a5u16bs = {}", a5u16, a5u16bs);
        let a5u16bs2bs = hex::decode(a5u16bs).unwrap();
        println!("a5u16bs2bs = {:#?}", a5u16bs2bs);
        let a5u16_2 = trans_bytes_2_u16(a5u16bs2bs);
        println!("a5u16_2 = {}", a5u16_2);
    }
}
