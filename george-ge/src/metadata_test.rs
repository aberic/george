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
mod metadata {
    use crate::header::Digest;
    use crate::metadata::{Description, Header};
    use crate::utils::enums::Tag;
    use crate::Metadata;
    use std::sync::{Arc, RwLock};

    #[test]
    fn fmt() {
        let digest = Arc::new(RwLock::new(Digest {
            tag: Tag::View,
            version: [0x01, 0x02],
            sequence: [0x03, 0x04],
        }));
        let description = Arc::new(RwLock::new(Description {
            start: 100,
            len: 1000,
            modify: 10000,
        }));
        let header = Header { digest };
        let md = Metadata {
            header,
            description,
        };
        println!("md = {:#?}", md);
    }
}
