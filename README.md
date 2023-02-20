# ONNX command-line tool based on Rust
# Introduction
This is a mini project based on the course example of ONNX command line tool. This is based on the onnxruntime, and it can infer the score of the input model. For simplicity, the test model is squeezenet1.0 just the same as the model in the course example.

# Usage
Use ```cargo run -- onnxrun``` to run the application.
# Issues
There is one potential issue that this project could be failed on Apple Silicon Macbook. As the onnxruntime mentions that there is no pre-build binary for M-cpu Mac. One solution to it is to export two environment variables. Or for testing, you can just use a vm to test.
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
