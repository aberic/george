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

use crate::protos::db::db::{Index, View};
use crate::tools::{Children, Trans};
use db::task::traits::TForm;
use std::sync::{Arc, RwLock};

impl Children {
    pub(crate) fn indexes(view: Arc<RwLock<db::task::View>>) -> Vec<Index> {
        let view_r = view.read().unwrap();
        let indexes = view_r.index_map();
        let indexes_r = indexes.read().unwrap();
        let mut indexes: Vec<Index> = vec![];
        for (_name, index) in indexes_r.iter() {
            indexes.push(Index {
                name: index.name(),
                engine: Trans::db_2_engine_i32(index.engine()),
                primary: index.primary(),
                unique: index.unique(),
                null: index.null(),
                key_type: Trans::db_2_key_type_i32(index.key_type()),
                create_time: Some(Trans::proto_time_2_grpc_timestamp(index.create_time())),
            })
        }
        indexes
    }

    pub(crate) fn views(database: Arc<RwLock<db::task::Database>>) -> Vec<View> {
        let database_r = database.read().unwrap();
        let views = database_r.view_map();
        let views_r = views.read().unwrap();
        let mut views: Vec<View> = vec![];
        for (_name, view) in views_r.iter() {
            let indexes = Children::indexes(view.clone());
            let view_r = view.read().unwrap();
            views.push(View {
                name: view_r.name(),
                comment: view_r.comment(),
                create_time: Some(Trans::proto_time_2_grpc_timestamp(view_r.create_time())),
                indexes,
                filepath: view_r.filepath(),
                version: view_r.version() as u32,
            })
        }
        views
    }
}
