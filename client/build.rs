// 编译proto文件，将.proto文件，转换成.rs文件
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true) // 是否编译生成用于客户端的代码
        // .build_server(true) // 是否编译生成用于服务端的代码
        // src/grpc这个目录一定要存在
        .out_dir("src/grpc/") // 输出的路径，针对于项目根目录下的具体路径
        .compile(&["protos/grpc_scheduler.proto"], &["protos/"])?; // 制定扩展文件路径，或者制定proto文件所在的目录
    Ok(())
}
