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

use std::future::Future;
use std::sync::Arc;

use tokio::task::JoinHandle;

use crate::errors::{Errs, GeorgeResult};
use crate::pool::ThreadPool;

impl ThreadPool {
    pub fn new(mut worker_threads: usize) -> GeorgeResult<ThreadPool> {
        if worker_threads > 1000 {
            worker_threads = 1000;
        }
        let mut build = tokio::runtime::Builder::new_multi_thread();
        build.worker_threads(worker_threads);
        build.enable_all();
        match build.build() {
            Ok(runtime) => Ok(ThreadPool {
                runtime: Arc::new(runtime),
            }),
            Err(err) => Err(Errs::strs("runtime new", err)),
        }
    }

    pub fn init(&self) {}

    pub fn spawn<T>(&self, task: T) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        tokio::spawn(task)
    }

    pub fn task_block_on<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }

    pub fn task_spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    pub fn task_spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.runtime.spawn_blocking(func)
    }
}
