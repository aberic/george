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

use protoc_rust::Customize;

// 先执行无依赖proto，完成后导入mod，再执行有依赖proto，完成后导入mod
fn init_chain() {
    let mut ps_block: Vec<&str> = vec![];
    ps_block.push("chain/contract.proto");
    ps_block.push("chain/data.proto");
    ps_block.push("chain/peer.proto");
    ps_block.push("chain/policy.proto");
    ps_block.push("chain/rwset.proto");
    ps_block.push("chain/sign.proto");
    ps_block.push("chain/block.proto");
    ps_block.push("chain/genesis.proto");
    ps_block.push("chain/ledger.proto");
    ps_block.push("chain/organization.proto");
    ps_block.push("chain/transaction.proto");
    ps_block.push("chain/service.proto");
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/impls/chain")
        .inputs(ps_block)
        .rust_protobuf(true)
        .rust_protobuf_customize(Customize {
            serde_derive: Some(true),
            ..Default::default()
        })
        .run()
        .expect("protoc-rust-grpc");
}

fn init_db() {
    let mut ps_block: Vec<&str> = vec![];
    ps_block.push("db/user.proto");
    ps_block.push("db/database.proto");
    ps_block.push("db/index.proto");
    ps_block.push("db/master.proto");
    ps_block.push("db/page.proto");
    ps_block.push("db/service.proto");
    ps_block.push("db/view.proto");
    ps_block.push("db/disk.proto");
    ps_block.push("db/memory.proto");
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/impls/db")
        .inputs(ps_block)
        .rust_protobuf(true)
        .rust_protobuf_customize(Customize {
            serde_derive: Some(true),
            ..Default::default()
        })
        .run()
        .expect("protoc-rust-grpc");
}

// 先执行无依赖proto，完成后导入mod，再执行有依赖proto，完成后导入mod
fn main() {
    init_chain();
    init_db();
}
