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
mod errors_index_test {
    use std::error::Error;

    use crate::errors::{Errs, GeorgeResult};
    use crate::io::dir::DirHandler;
    use crate::io::Dir;

    fn index_ok() -> GeorgeResult<u32> {
        Ok(500)
    }

    fn index_exist_err() -> GeorgeResult<u32> {
        Err(Errs::data_exist_error())
    }

    fn index_no_exist_err() -> GeorgeResult<u32> {
        Err(Errs::data_no_exist_error())
    }

    fn index_test1() -> GeorgeResult<u32> {
        let v = index_ok()?;
        Ok(v)
    }

    fn index_test2() -> GeorgeResult<u32> {
        // let x = index_exist_err()?;
        Ok(100)
    }

    fn index_test3() -> GeorgeResult<u32> {
        // let x = index_no_exist_err()?;
        Ok(100)
    }

    fn index_test4() -> GeorgeResult<u32> {
        // let x = index_no_exist_err()?;
        match Dir::exist("src/cryptos/mod.rs") {
            Ok(_) => Ok(1),
            Err(err) => Err(Errs::strs("test4", err)),
        }
    }

    // fn index_test5() -> GeorgeResult<u32> {
    //     // let x = index_no_exist_err()?;
    //     match Dir::exist("src/cryptos/mod.rs") {
    //         Ok(_) => Ok(1),
    //         Err(err) => Err(GeorgeError::string("test5".to_string(), err)),
    //     }
    // }

    fn matches(ir: GeorgeResult<u32>) {
        match ir {
            Ok(u) => println!("u is {}", u),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
    }

    #[test]
    fn index() {
        let res = index_ok();
        matches(res)
    }

    #[test]
    fn index_exist() {
        let res = index_exist_err();
        matches(res)
    }

    #[test]
    fn index_no_exist() {
        let res = index_no_exist_err();
        matches(res)
    }

    #[test]
    fn index1() {
        let res = index_test1();
        matches(res)
    }

    #[test]
    fn index2() {
        let res = index_test2();
        matches(res)
    }

    #[test]
    fn index3() {
        let res = index_test3();
        matches(res)
    }

    #[test]
    fn index4() {
        let res = index_test4();
        matches(res)
    }

    #[test]
    fn index5() {
        let res = index_test4();
        matches(res)
    }
}
