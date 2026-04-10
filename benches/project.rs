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
use flavorcli::commands::project::{get::ProjectGet, list::ProjectList};

#[allow(unused)]

fn cmd_project_get(c: &mut Criterion) {
    c.bench_function("cmd_project_get", |b| {
        b.iter(|| {
            black_box(
                Command::new("./target/release/flavor")
                    .args(["project", "get", "1"])
                    .status()
                    .expect("Failed to execute command")
            );
        })
    });
}

fn cmd_project_list(c: &mut Criterion) {
    c.bench_function("cmd_project_list", |b| {
        b.iter(|| {
            black_box(
                Command::new("./target/release/flavor")
                    .args(["project", "list"])
                    .status()
                    .expect("Failed to execute command")
            );
        })
    });
}

fn fn_project_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("fn_project_get", |b| {
        b.iter(|| {
            let cmd = ProjectGet {
                project_id: 1,
                json: false,
                resolve: false,
            };
            black_box(rt.block_on(cmd.execute()));
        })
    });
}

fn fn_project_list(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("fn_project_list", |b| {
        b.iter(|| {
            let cmd = ProjectList {
                json: false,
                page: None,
                fields: Vec::new(),
                query: None,
            };
            black_box(rt.block_on(cmd.execute()));
        })
    });
}

criterion_group!(cmd_project_benches, cmd_project_get, cmd_project_list);
criterion_group!(fn_project_benches, fn_project_get, fn_project_list);
criterion_main!(cmd_project_benches, fn_project_benches);