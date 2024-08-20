use crate::parse::CommandLine;


/// rust语言的libfuzzer
pub fn run_fuzzer(opt: &CommandLine){
    println!("opt: {:?}", opt);
}