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
mod vectors {
    use crate::bytes::create_empty_bytes;
    use crate::vectors::{Vector, VectorHandler};

    #[test]
    fn modify_test() {
        let x: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10];
        let start = 3;
        let y: Vec<u8> = vec![0x20, 0x21, 0x22, 0x23, 0x24];
        let z = Vector::modify(x.clone(), y, start);
        println!("x = {:#?}\ny = {:#?}", x, z)
    }

    #[test]
    fn sub_test() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("sub = {:#?}", Vector::sub(vec, 2, 5));

        let x: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10];
        println!("sub = {:#?}", Vector::sub(x.clone(), 2, 5));
        println!("x = {:#?}", x);
    }

    #[test]
    fn find_last_eq_bytes_test() {
        let mut a: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let mut b = create_empty_bytes(8);
        let mut c = vec![0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10];
        let mut d = create_empty_bytes(8);
        let mut e = vec![0x03, 0x04, 0x05, 0x06, 0x01, 0x02, 0x08, 0x10];
        let mut f = create_empty_bytes(8);
        a.append(&mut b);
        a.append(&mut c);
        a.append(&mut d);
        a.append(&mut e);
        a.append(&mut f);
        println!("a = {:#?}", a);
        let g = Vector::find_last_eq_bytes(a, 8);
        println!("g = {:#?}", g);
    }

    #[test]
    fn find_eq_vec_bytes_test() {
        let mut a: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let mut b = create_empty_bytes(8);
        let mut c = vec![0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10];
        let mut d = create_empty_bytes(8);
        let mut e = vec![0x03, 0x04, 0x05, 0x06, 0x01, 0x02, 0x08, 0x10];
        let mut f = create_empty_bytes(8);
        a.append(&mut b);
        a.append(&mut c);
        a.append(&mut d);
        a.append(&mut e);
        a.append(&mut f);
        println!("a = {:#?}", a);
        let g = Vector::find_eq_vec_bytes(a, 8);
        println!("g = {:#?}", g);
    }
}
