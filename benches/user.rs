// Copyright 2026 Lordseriouspig
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::{criterion_group, criterion_main, Criterion};
use std::process::Command;
use std::hint::black_box;
use tokio::runtime::Runtime;
use flavorcli::commands::user::{get::UserGet, list::UserList};

#[allow(unused)]

fn cmd_user_get(c: &mut Criterion) {
    c.bench_function("cmd_user_get", |b| {
        b.iter(|| {
            black_box(
                Command::new("./target/release/flavor")
                    .args(["user", "get", "1"])
                    .status()
                    .expect("Failed to execute command")
            );
        })
    });
}

fn cmd_user_list(c: &mut Criterion) {
    c.bench_function("cmd_user_list", |b| {
        b.iter(|| {
            black_box(
                Command::new("./target/release/flavor")
                    .args(["user", "list"])
                    .status()
                    .expect("Failed to execute command")
            );
        })
    });
}

fn fn_user_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("fn_user_get", |b| {
        b.iter(|| {
            let cmd = UserGet {
                user_id: Some(1),
                json: false,
                resolve: 0,
            };
            black_box(rt.block_on(cmd.execute()));
        })
    });
}

fn fn_user_list(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("fn_user_list", |b| {
        b.iter(|| {
            let cmd = UserList {
                json: false,
                page: None,
                fields: Vec::new(),
                query: None,
            };
            black_box(rt.block_on(cmd.execute()));
        })
    });
}

criterion_group!(cmd_user_benches, cmd_user_get, cmd_user_list);
criterion_group!(fn_user_benches, fn_user_get, fn_user_list);
criterion_main!(cmd_user_benches, fn_user_benches);