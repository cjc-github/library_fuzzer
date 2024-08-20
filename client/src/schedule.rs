use crate::parse::CommandLine;
use crate::engine::libfuzzer_c;
use crate::engine::libfuzzer_csharp;
use crate::engine::libfuzzer_go;
use crate::engine::libfuzzer_java;
use crate::engine::libfuzzer_rust;


/// 根据不同的语言来选择不同的引擎
pub async fn select_engine(opt: &CommandLine) {
    // 语言支持大小写混写
    match (opt.language.to_lowercase().as_str(), opt.engine.as_str()) {
        ("c", "xlibfuzzer") => libfuzzer_c::run_fuzzer(opt).await,
        // ("c#", "xlibfuzzer") => libfuzzer_csharp::run_fuzzer(opt).await,
        // ("go", "xlibfuzzer") => libfuzzer_go::run_fuzzer(opt).await,
        // ("java", "xlibfuzzer") => libfuzzer_java::run_fuzzer(opt).await,
        // ("rust", "xlibfuzzer") => libfuzzer_rust::run_fuzzer(opt).await,
        _ => {
            println!("Unsupported language: {} or engine: {}", opt.language, opt.engine);
            return;
        }
    }
}
