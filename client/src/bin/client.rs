
use clap::Parser;
use xfl::parse::CommandLine;


// target/debug/client -s 172.1.1.1:3389 -l c -p --args "afl-fuzz -i in -o out -- ./fuzzgoat @@"

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 需要解析命令行, 然后启动不同的引擎, 在不同的引擎中需要实现对服务端的通信 [ok]
    let opt = CommandLine::parse();

    // 将命令行信息保存到结构体中
    xfl::config::init_config(&opt).await;

    // 传递语言类型，然后选择不同的引擎并执行
    xfl::schedule::select_engine(&opt).await;

    Ok(())
}
