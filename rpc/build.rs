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
        .build_server(false)
        .build_client(true)
        .out_dir("src/comm")
        // .extern_path(".comm", "crate::comm::comm")
        .compile(
            &[
                "rust/comm/request.proto",
                "rust/comm/response.proto",
                "rust/comm/timestamp.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}

fn db() -> Result<(), std::io::Error> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src/db")
        .extern_path(".comm", "crate::comm::comm")
        .compile(
            &[
                "database.proto",
                "disk.proto",
                "index.proto",
                "master.proto",
                "memory.proto",
                "page.proto",
                "service.proto",
                "user.proto",
                "view.proto",
                "request.proto",
                "response.proto",
                "timestamp.proto",
            ],
            &["../protos/rust/db", "../protos/rust/comm"],
        )?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    comm()?;
    // db()?;
    Ok(())
}
