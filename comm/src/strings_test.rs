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
    use crate::strings::{left_fit, left_un_fit, repeated_string, right_zero, sub_string};

    #[test]
    fn sub_string_test() {
        let s = String::from("hello world, 你好，中国！");
        println!("{:#?}", s.chars());
        println!("{}", sub_string(s.clone(), 0, 1));
        println!("{}", sub_string(s.clone(), 1, 2));
        println!("{}", sub_string(s.clone(), 2, 3));
        println!("{}", sub_string(s.clone(), 3, 4));
        println!("{}", sub_string(s.clone(), 5, 10));
        println!("{}", sub_string(s.clone(), 13, 15));
        println!("{}", sub_string(s.clone(), 16, 18));
    }

    #[test]
    fn zero_test() {
        let x = "hello".to_string();
        let x1 = left_fit(x.clone(), "0".parse().unwrap(), 6);
        let x2 = left_fit(x.clone(), "#".parse().unwrap(), 10);
        let x3 = left_fit(x.clone(), "@".parse().unwrap(), 11);
        let x4 = left_fit(x.clone(), "%".parse().unwrap(), 12);
        let x5 = left_fit(x.clone(), "*".parse().unwrap(), 30);
        println!("1 = {}", x1);
        println!("2 = {}", x2);
        println!("3 = {}", x3);
        println!("4 = {}", x4);
        println!("5 = {}", x5);

        println!();

        println!("1 = {}", left_un_fit(x1.clone(), "0".parse().unwrap()));
        println!("2 = {}", left_un_fit(x2.clone(), "#".parse().unwrap()));
        println!("3 = {}", left_un_fit(x3.clone(), "@".parse().unwrap()));
        println!("4 = {}", left_un_fit(x4.clone(), "%".parse().unwrap()));
        println!("5 = {}", left_un_fit(x5.clone(), "*".parse().unwrap()));

        println!();

        println!("1 = {}", right_zero(x.clone(), 6));
        println!("2 = {}", right_zero(x.clone(), 10));
        println!("3 = {}", right_zero(x.clone(), 11));
        println!("4 = {}", right_zero(x.clone(), 12));
        println!("5 = {}", right_zero(x.clone(), 13));
    }

    #[test]
    fn repeated_string_test() {
        let repeated1 = "hello";
        let repeated_string1 = repeated_string(repeated1, 10);
        println!("repeated_string1 = {}", repeated_string1);
        println!();
        let repeated2 = "0";
        let repeated_string2 = repeated_string(repeated2, 1537);
        println!("repeated_string2 = {}", repeated_string2);
    }
}
