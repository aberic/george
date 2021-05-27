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

fn protos_chain_single() {
    let mut ps_block: Vec<&str> = vec![];
    ps_block.push("src/protos/chain/timestamp.proto");
    ps_block.push("src/protos/chain/contract.proto");
    ps_block.push("src/protos/chain/data.proto");
    ps_block.push("src/protos/chain/peer.proto");
    ps_block.push("src/protos/chain/policy.proto");
    ps_block.push("src/protos/chain/rwset.proto");
    ps_block.push("src/protos/chain/sign.proto");
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

fn protos_chain_depends() {
    let mut ps_block: Vec<&str> = vec![];
    ps_block.push("src/protos/chain/block.proto");
    ps_block.push("src/protos/chain/genesis.proto");
    ps_block.push("src/protos/chain/ledger.proto");
    ps_block.push("src/protos/chain/organization.proto");
    ps_block.push("src/protos/chain/transaction.proto");
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

fn protos_chain_service() {
    let mut ps_block: Vec<&str> = vec![];
    ps_block.push("src/protos/chain/service.proto");
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

// 先执行无依赖proto，完成后导入mod，再执行有依赖proto，完成后导入mod
fn main() {
    // protos_chain_single();
    // protos_chain_depends();
    protos_chain_service();
}
