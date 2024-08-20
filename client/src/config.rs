use crate::parse::CommandLine;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
// use std::path::PathBuf;
use tokio::sync::OnceCell;


// 结构体，包括输出目录、服务端地址
pub struct ServerConfig {
    pub server_addr: SocketAddr,
    pub language: String,
    pub engine: String,
    pub persistent: u8,
    pub args: String,
}

// 给定默认ip地址和端口
impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50051),
            language: Default::default(),
            engine: Default::default(),
            persistent: Default::default(),
            args: Default::default(),
        }
    }
}

/// 全局变量，保存服务端配置信息
pub static SERVER_CONFIG: OnceCell<ServerConfig> = OnceCell::const_new();

/// 异步函数，使用命令行参数初始化配置项
pub async fn init_config(opt: &CommandLine) {
    // 从get_or_init获取值，如果尚未初始化，则执行闭包中的逻辑进行初始化，从而确保在并发环境下只初始化一次
    SERVER_CONFIG
        .get_or_init(|| async {
            // 创建实例
            ServerConfig {
                server_addr: opt.address,
                language: opt.language.clone(),
                engine: opt.engine.clone(),
                persistent: opt.persistent,
                args: opt.args.clone(),
            }
        }).await;
}