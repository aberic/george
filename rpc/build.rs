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

fn comm() -> Result<(), std::io::Error> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .format(true)
        .compile_well_known_types(true)
        .out_dir("src/protos/utils")
        // .extern_path(".comm", "crate::comm::comm")
        .compile(
            &[
                "utils/request.proto",
                "utils/response.proto",
                "utils/timestamp.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}

fn db() -> Result<(), std::io::Error> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .format(true)
        .compile_well_known_types(true)
        .out_dir("src/protos/db")
        // .extern_path(".utils", "super")
        .compile(
            &[
                "db/database.proto",
                "db/disk.proto",
                "db/index.proto",
                "db/master.proto",
                "db/memory.proto",
                "db/page.proto",
                "db/service.proto",
                "db/user.proto",
                "db/view.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}

fn chain() -> Result<(), std::io::Error> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .format(true)
        .compile_well_known_types(true)
        .out_dir("src/protos/chain")
        // .extern_path(".comm", "crate::comm::comm")
        .compile(
            &[
                "chain/block.proto",
                "chain/contract.proto",
                "chain/data.proto",
                "chain/genesis.proto",
                "chain/ledger.proto",
                "chain/organization.proto",
                "chain/peer.proto",
                "chain/policy.proto",
                "chain/rwset.proto",
                "chain/service.proto",
                "chain/sign.proto",
                "chain/transaction.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    comm()?;
    db()?;
    chain()?;
    Ok(())
}
