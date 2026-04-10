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
use flavorcli::commands::store::{get::StoreGet, list::{StoreList, StoreFields, SortOrder, SortFields}};
use flavorcli::models::session::Session;
use flavorcli::helpers::get_key::get_key;

#[allow(unused)]

fn cmd_store_get(c: &mut Criterion) {
    c.bench_function("cmd_store_get", |b| {
        b.iter(|| {
            black_box(
                Command::new("./target/release/flavor")
                    .args(["store", "get", "1"])
                    .status()
                    .expect("Failed to execute command")
            );
        })
    });
}

fn cmd_store_list(c: &mut Criterion) {
    c.bench_function("cmd_store_list", |b| {
        b.iter(|| {
            black_box(
                Command::new("./target/release/flavor")
                    .args(["store", "list"])
                    .status()
                    .expect("Failed to execute command")
            );
        })
    });
}

fn fn_store_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let session = Session::new();
    let auth = get_key().unwrap();
    c.bench_function("fn_store_get", |b| {
        b.iter(|| {
            let cmd = StoreGet {
                item_id: 1,
                json: false,
                short: false,
                detailed: false,
            };
            black_box(rt.block_on(cmd.execute(&session, &auth)));
        })
    });
}

fn fn_store_list(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let session = Session::new();
    let auth = get_key().unwrap();
    c.bench_function("fn_store_list", |b| {
        b.iter(|| {
            let cmd = StoreList {
                json: false,
                region: None,
                fields: Vec::from([
                    StoreFields::Id,
                    StoreFields::Name,
                    StoreFields::Description,
                    StoreFields::Stock,
                    StoreFields::Regional,
                    StoreFields::Type,
                    StoreFields::AttachedTo,
                ]),
                sort: SortFields::Id,
                sort_region: None,
                sort_order: SortOrder::Asc,
            };
            black_box(rt.block_on(cmd.execute(&session, &auth)));
        })
    });
}

criterion_group!(cmd_store_benches, cmd_store_get, cmd_store_list);
criterion_group!(fn_store_benches, fn_store_get, fn_store_list);
criterion_main!(cmd_store_benches, fn_store_benches);