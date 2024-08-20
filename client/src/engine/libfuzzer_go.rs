use crate::parse::CommandLine;


/// go语言的libfuzzer
pub fn run_fuzzer(opt: &CommandLine){
    println!("opt: {:?}", opt);
}