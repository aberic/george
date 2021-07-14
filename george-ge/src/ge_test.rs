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
mod ge_lib {
    use crate::factory::Bootstrap;
    use crate::utils::enums::Tag;
    use crate::{Ge, GeFactory};

    #[test]
    fn create() {
        let b = Bootstrap::new("src/test/factory/tmp/none.ge", "test".as_bytes().to_vec()).unwrap();
        println!("filepath = {}", b.filepath());
    }

    #[test]
    fn recovery() {
        let b = Bootstrap::recovery("src/test/factory/tmp/none.ge").unwrap();
        println!("filepath = {}", b.filepath());
        println!("metadata = {:#?}", b.metadata());
        println!("header = {:#?}", b.metadata().header());
        println!("description = {:#?}", b.metadata().description());
        println!("digest = {:#?}", b.metadata().header().digest());
    }

    #[test]
    fn factory_create() {
        let f = GeFactory;
        let b_ge = f
            .create(
                Tag::Bootstrap,
                "src/test/factory/impl/none.ge",
                Some("test".as_bytes().to_vec()),
            )
            .unwrap();
        println!("filepath = {}", b_ge.filepath());
    }

    #[test]
    fn factory_recovery() {
        let f = GeFactory;
        let b_ge = f
            .recovery(Tag::Bootstrap, "src/test/factory/impl/none.ge")
            .unwrap();
        println!("filepath = {}", b_ge.filepath());
        println!("metadata = {:#?}", b_ge.metadata());
        println!("header = {:#?}", b_ge.metadata().header());
        println!("description = {:#?}", b_ge.metadata().description());
        println!("digest = {:#?}", b_ge.metadata().header().digest());
    }
}

#[cfg(test)]
mod ge {
    use crate::utils::enums::Tag;
    use crate::{Ge, GeImpl};

    #[test]
    fn fmt() {
        let ge = GeImpl::new(
            "src/test/fmt/none.ge",
            Tag::None,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = GeImpl::new(
            "src/test/fmt/bootstrap.ge",
            Tag::Bootstrap,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = GeImpl::new(
            "src/test/fmt/page.ge",
            Tag::Page,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = GeImpl::new(
            "src/test/fmt/view.ge",
            Tag::View,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = GeImpl::new(
            "src/test/fmt/index.ge",
            Tag::Index,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge = GeImpl::new(
            "src/test/fmt/ledger.ge",
            Tag::Ledger,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
    }

    #[test]
    fn recovery() {
        let ge = GeImpl::mock_new(
            "src/test/recovery/none.ge",
            Tag::None,
            "test".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge_recovery = GeImpl::recovery("src/test/recovery/none.ge").unwrap();
        println!("ge_recovery = {:#?}", ge_recovery);
    }

    #[test]
    fn modify_history() {
        let ge = GeImpl::mock_new(
            "src/test/modify/none.ge",
            Tag::None,
            "hello".as_bytes().to_vec(),
        )
        .unwrap();
        println!("ge = {:#?}", ge);
        let ge_recovery = GeImpl::recovery("src/test/modify/none.ge").unwrap();
        println!("ge_recovery = {:#?}", ge_recovery);
        ge.modify("world 1".as_bytes().to_vec()).unwrap();
        println!("ge = {:#?}", ge);
        let ge_modify1 = GeImpl::recovery("src/test/modify/none.ge").unwrap();
        println!("ge_modify1 = {:#?}", ge_modify1);
        ge.modify("world 2".as_bytes().to_vec()).unwrap();
        println!("ge = {:#?}", ge);
        let ge_modify2 = GeImpl::recovery("src/test/modify/none.ge").unwrap();
        println!("ge_modify2 = {:#?}", ge_modify2);
        ge.modify("world 3".as_bytes().to_vec()).unwrap();
        println!("ge = {:#?}", ge);
        let ge_modify3 = GeImpl::recovery("src/test/modify/none.ge").unwrap();
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
        let ge = GeImpl::mock_new(
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
        let ge_recovery = GeImpl::recovery("src/test/rebuild/build.ge").unwrap();
        println!("ge_recovery = {:#?}", ge_recovery);
    }
}
