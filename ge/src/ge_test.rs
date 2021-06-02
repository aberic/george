/*
 * Copyright (c) 2021. Aberic - All Rights Reserved.
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
mod ge {
    use crate::utils::enums::Tag;
    use crate::Ge;

    #[test]
    fn fmt() {
        let ge = Ge::new(
            "src/test/fmt/none.ge",
            Tag::None,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = Ge::new(
            "src/test/fmt/bootstrap.ge",
            Tag::Bootstrap,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = Ge::new(
            "src/test/fmt/page.ge",
            Tag::Page,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = Ge::new(
            "src/test/fmt/view.ge",
            Tag::View,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = Ge::new(
            "src/test/fmt/index.ge",
            Tag::Index,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = Ge::new(
            "src/test/fmt/ledger.ge",
            Tag::Ledger,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
    }

    #[test]
    fn recovery() {
        let ge = Ge::mock_new(
            "src/test/recovery/none.ge",
            Tag::None,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge_recovery = Ge::recovery("src/test/recovery/none.ge").unwrap();
        println!("ge_recovery = {:#?}", ge_recovery);
    }

    #[test]
    fn modify_history() {
        let mut ge = Ge::mock_new(
            "src/test/modify/none.ge",
            Tag::None,
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge_recovery = Ge::recovery("src/test/modify/none.ge").unwrap();
        println!("ge_recovery = {:#?}", ge_recovery);
        ge.modify("world 1".as_bytes().to_vec()).unwrap();
        println!("ge = {:#?}", ge);
        let ge_modify1 = Ge::recovery("src/test/modify/none.ge").unwrap();
        println!("ge_modify1 = {:#?}", ge_modify1);
        ge.modify("world 2".as_bytes().to_vec()).unwrap();
        println!("ge = {:#?}", ge);
        let ge_modify2 = Ge::recovery("src/test/modify/none.ge").unwrap();
        println!("ge_modify2 = {:#?}", ge_modify2);
        ge.modify("world 3".as_bytes().to_vec()).unwrap();
        println!("ge = {:#?}", ge);
        let ge_modify3 = Ge::recovery("src/test/modify/none.ge").unwrap();
        println!("ge_modify3 = {:#?}", ge_modify3);

        let vc = ge.history().unwrap();
        for v in vc {
            println!("v = {}", String::from_utf8(v).unwrap())
        }
        println!(
            "last des = {}",
            String::from_utf8(ge.description_content_bytes().unwrap()).unwrap()
        )
    }

    #[test]
    fn rebuild() {
        let mut ge = Ge::mock_new(
            "src/test/rebuild/none.ge",
            Tag::None,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        ge.modify("world 1".as_bytes().to_vec()).unwrap();
        ge.modify("world 2".as_bytes().to_vec()).unwrap();
        ge.modify("world 3".as_bytes().to_vec()).unwrap();
        ge.modify("world 4".as_bytes().to_vec()).unwrap();
        ge.modify("world 5".as_bytes().to_vec()).unwrap();

        let hb = ge.metadata.header.to_vec().unwrap();
        let dcb = ge.history().unwrap();
        ge.archive("src/test/rebuild/build.ge".to_string()).unwrap();
        ge.rebuild(hb, dcb).unwrap();
        println!("ge = {:#?}", ge);
        let ge_recovery = Ge::recovery("src/test/rebuild/build.ge").unwrap();
        println!("ge_recovery = {:#?}", ge_recovery);
    }
}