use crate::parse::CommandLine;
use crate::config::SERVER_CONFIG;

use anyhow::{anyhow, Context, Result, bail, format_err};
use log::{info, warn, error, debug};
use futures::future::try_join_all;
use std::num::Wrapping;
use std::process::ExitStatus;
use async_trait::async_trait;

// use tracing::event;
// use tracing::metric;
use tracing::Level;

use serde::Deserialize;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    fmt::Debug,
    process::Stdio,
    path::{Path, PathBuf},
    sync::Arc,
};
use tempfile::{tempdir_in, TempDir};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::{mpsc, Notify},
    time::{sleep, Duration, Instant},
};
use rand::thread_rng;
use tempfile::tempdir;
use uuid::Uuid;
use tokio::process::{Child, Command};
use serde::Serialize;
// use std::process::Command;


/// c/c++语言的libfuzzer
pub async fn run_fuzzer(opt: &CommandLine){
    println!("opt: {:?}", opt);
    println!("Scheduler service listening on {}", SERVER_CONFIG.get().unwrap().server_addr);

    let runtime_stats = RuntimeStats {
        worker_id: 0,
        run_id: Uuid::new_v4(),
        count: 0,
        execs_sec: 0.0,
        cmd: SERVER_CONFIG.get().unwrap().args.clone(),

        crashes: 0,
        queuees: 0,

        basicblocks: 0,
        whole_basicblocks: 0,
        functions: 0,
        whole_functions: 0,
        lines: 0,
        whole_lines: 0,
        edges: 0,
        whole_edges: 0,
    };
    let my_instance = LibFuzzerFuzzTask::new(runtime_stats);
    my_instance.run().await;
}

pub struct RuntimeStats {
    // 开始时间， 执行次数、崩溃数量、队列数量、覆盖率（基本块、函数、行、边）
    worker_id: usize,
    run_id: Uuid,

    count: u64,
    execs_sec: f64,
    cmd: String,

    crashes: u64,
    queuees: u64,

    basicblocks: u64,
    whole_basicblocks: u64,
    functions: u64,
    whole_functions: u64,
    lines: u64,
    whole_lines: u64,
    edges: u64,
    whole_edges: u64,
}

pub struct LibFuzzerFuzzTask {
    runtime_stats: RuntimeStats,
}

impl LibFuzzerFuzzTask {
    /// 创建新的 LibFuzzerFuzzTask 实例
    pub fn new(runtime_stats: RuntimeStats,) -> Self {
        LibFuzzerFuzzTask {
            // 初始化字段
            runtime_stats,
        }
    }

    /// 运行 fuzzer 任务
    pub async fn run(&self) -> Result<()> {
        self.run_fuzzers().await
    }

    /// 开启 fuzzer 进程
    async fn run_fuzzers(&self) -> Result<()> {
        let worker_id = 1;
        self.start_fuzzer_monitor(worker_id).await?;
        Ok(())
    }

    /// 持续运行 fuzzer 进程
    async fn start_fuzzer_monitor(&self, worker_id: usize) -> Result<()> {
        // loop {
        //     self.run_fuzzer(worker_id).await?;
        // }
        self.run_fuzzer(worker_id).await?;
        Ok(())
    }

    /// 运行单个 fuzzer 进程
    async fn run_fuzzer(&self, worker_id: usize) -> Result<()> {
        println!("outcome: {:?}", self.runtime_stats.execs_sec);
        let mut running = self.fuzz_cmd(self.runtime_stats.cmd.as_str()).await;

        println!("child is: {:?}", running);

        // 实现具体的 fuzzer 运行逻辑
        Ok(())
    }

    async fn fuzz_cmd(&self, cmd: &str) -> Result<Child> {
        println!("Running command: {:?}", &cmd);

        let child = Command::new(cmd)
            .spawn()
            .with_context(|| format_err!("libfuzzer failed to start."))?;

        Ok(child)
    }

}