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
use flavorcli::commands::devlog::{get::DevlogGet, list::DevlogList};

#[allow(unused)]

fn cmd_devlog_get(c: &mut Criterion) {
    c.bench_function("cmd_devlog_get", |b| {
        b.iter(|| {
            black_box(
                Command::new("./target/release/flavor")
                    .args(["devlog", "get", "1"])
                    .status()
                    .expect("Failed to execute command")
            );
        })
    });
}

fn cmd_devlog_list(c: &mut Criterion) {
    c.bench_function("cmd_devlog_list", |b| {
        b.iter(|| {
            black_box(
                Command::new("./target/release/flavor")
                    .args(["devlog", "list"])
                    .status()
                    .expect("Failed to execute command")
            );
        })
    });
}

fn fn_devlog_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("fn_devlog_get", |b| {
        b.iter(|| {
            let cmd = DevlogGet {
                devlog_id: 1,
                json: false,
                short: false,
            };
            black_box(rt.block_on(cmd.execute()));
        })
    });
}

fn fn_devlog_list(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("fn_devlog_list", |b| {
        b.iter(|| {
            let cmd = DevlogList {
                json: false,
                page: None,
                fields: Vec::new(),
                project_id: None,
            };
            black_box(rt.block_on(cmd.execute()));
        })
    });
}

criterion_group!(cmd_devlog_benches, cmd_devlog_get, cmd_devlog_list);
criterion_group!(fn_devlog_benches, fn_devlog_get, fn_devlog_list);
criterion_main!(cmd_devlog_benches, fn_devlog_benches);