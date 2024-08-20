use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
// use std::path::PathBuf;

// CommandLine结构体，derive属性（宏）提供了Parser和Debug特性
#[derive(Parser, Debug)]
pub struct CommandLine {
    /// 服务绑定的IP地址及端口号
    #[arg(short = 's', long = "address", value_parser = parse_ipaddr, default_value = "")]
    pub address: SocketAddr,
    /// 类库的fuzz语言
    #[arg(short = 'l', long = "fuzzing language", default_value = "C")]
    pub language: String,
    /// 类库的引擎
    #[arg(short = 'e', long = "fuzzing engine", default_value = "xlibfuzzer")]
    pub engine: String,
    /// 持续fuzz
    #[arg(short = 'p', long = "persistent", action = clap::ArgAction::Count)]
    pub persistent: u8,
    /// fuzzer的执行命令
    #[arg(short = 'a', long = "args", default_value = "Fuzzer [args]")]
    pub args: String,
}

/// 解析地址，支持IP及IP:port格式，默认port为3000
fn parse_ipaddr(s: &str) -> Result<SocketAddr, String> {
    // 如果输入字符串为空,则返回0.0.0.0:0
    if s.is_empty() {
        return Ok(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0));
    }

    // 尝试解析SocketAddr类型
    match s.parse::<SocketAddr>() {
        Ok(ip) => Ok(ip),
        // 尝试解析IpAddr类型
        Err(_) => match s.parse::<IpAddr>() {
            Ok(ip_addr) => Ok(SocketAddr::new(ip_addr, 3000)),
            Err(_) => Err(format!("非法IP及端口: {}", s)),
        },
    }
}
