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

use std::sync::Arc;

use grpc::{Result, ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
use protobuf::RepeatedField;

use db::task::traits::{TForm, TMaster};
use db::Task;
use protocols::impls::db::service_grpc::ViewService;
use protocols::impls::db::view::{RequestViewList, View, ViewList};

use crate::utils::Comm;

pub(crate) struct ViewServer {
    pub(crate) task: Arc<Task>,
}

impl ViewService for ViewServer {
    fn views(
        &self,
        _o: ServerHandlerContext,
        req: ServerRequestSingle<RequestViewList>,
        resp: ServerResponseUnarySink<ViewList>,
    ) -> Result<()> {
        let mut list = ViewList::new();
        let mut views: RepeatedField<View> = RepeatedField::new();
        let db_map = self.task.database_map();
        let db_map_r = db_map.read().unwrap();
        let view_map;
        match db_map_r.get(req.message.get_database_name()) {
            Some(db) => {
                view_map = db.read().unwrap().view_map();
            }
            None => return resp.finish(list),
        }
        let view_map_r = view_map.read().unwrap();
        for view in view_map_r.values() {
            let view_r = view.read().unwrap();
            let mut view_item = View::new();
            view_item.set_name(view_r.name());
            view_item.set_comment(view_r.comment());
            view_item.set_create_time(Comm::time_2_grpc_timestamp(view_r.create_time()));
            views.push(view_item);
        }
        list.set_views(views);
        resp.finish(list)
    }
}
