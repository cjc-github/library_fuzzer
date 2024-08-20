use crate::parse::CommandLine;


/// java语言的libfuzzer
pub fn run_fuzzer(opt: &CommandLine){
    println!("opt: {:?}", opt);
}