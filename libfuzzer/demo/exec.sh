cd ../
./build.sh

cd demo
rm -rf fisrt_fuzzer
clang++ -g -std=c++11 -fsanitize=fuzzer-no-link,address first_fuzzer.cc ../libFuzzer.a -o first_fuzzer
