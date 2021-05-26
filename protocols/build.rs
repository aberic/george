use std::env;

use protoc_rust::Customize;

fn openssl() {
    if let Ok(v) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();
        if version >= 0x1_01_01_00_0 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }
}

fn proto() {
    let mut ps_comm: Vec<&str> = vec![];
    ps_comm.push("protocols/src/protos/timestamp.proto");
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/protocols")
        .inputs(ps_comm)
        .rust_protobuf(true)
        .rust_protobuf_customize(Customize {
            serde_derive: Some(true),
            ..Default::default()
        })
        .run()
        .expect("protoc-rust-grpc");

    let mut ps_block: Vec<&str> = vec![];
    ps_block.push("protocols/src/protos/chain/block.proto");
    ps_block.push("protocols/src/protos/chain/contract.proto");
    ps_block.push("protocols/src/protos/chain/data.proto");
    ps_block.push("protocols/src/protos/chain/genesis.proto");
    ps_block.push("protocols/src/protos/chain/ledger.proto");
    ps_block.push("protocols/src/protos/chain/organization.proto");
    ps_block.push("protocols/src/protos/chain/peer.proto");
    ps_block.push("protocols/src/protos/chain/policy.proto");
    ps_block.push("protocols/src/protos/chain/rwset.proto");
    ps_block.push("protocols/src/protos/chain/sign.proto");
    ps_block.push("protocols/src/protos/chain/transaction.proto");
    protoc_rust_grpc::Codegen::new()
        .out_dir("protocols/src/impls/chain")
        .inputs(ps_block)
        .rust_protobuf(true)
        .rust_protobuf_customize(Customize {
            serde_derive: Some(true),
            ..Default::default()
        })
        .run()
        .expect("protoc-rust-grpc");
}

fn main() {
    openssl();
    proto();
}
