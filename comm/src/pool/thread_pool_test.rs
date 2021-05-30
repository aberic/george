/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
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
mod thread_pools {

    #[cfg(test)]
    mod thread_pool_1 {
        use crate::errors::{Errs, GeorgeResult};
        use crate::pool::ThreadPool;
        use std::thread;
        use std::time::Duration;
        use tokio::sync::mpsc;
        use tokio::sync::mpsc::Sender;

        struct Test {
            thread_pool: ThreadPool,
        }

        async fn spawn1(tx: Sender<&str>) -> GeorgeResult<()> {
            thread::sleep(Duration::from_secs(1));
            match tx.send("sending from first handle").await {
                Err(err) => Err(Errs::strs("send first", err)),
                _ => Ok(()),
            }
        }

        async fn spawn2(tx: Sender<&str>) -> GeorgeResult<()> {
            thread::sleep(Duration::from_secs(1));
            match tx.send("sending from second handle").await {
                Err(err) => Err(Errs::strs("send second", err)),
                _ => Ok(()),
            }
        }

        impl Test {
            async fn exec(&self) -> String {
                let (tx, mut rx) = mpsc::channel(32);
                let tx2 = tx.clone();
                self.thread_pool.spawn(spawn1(tx));
                self.thread_pool.spawn(spawn2(tx2));

                while let Some(message) = rx.recv().await {
                    println!("GOT = {}", message);
                }
                "done".to_string()
            }
        }

        #[test]
        fn test_1() {
            let thread_pool = ThreadPool::new(10).unwrap();
            let test = Test { thread_pool };
            let f = test.thread_pool.clone().task_block_on(test.exec());
            println!("return is {}", f);
        }
    }
}
