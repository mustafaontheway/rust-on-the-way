In Rust, every operation has its own specific error type:

File reading errors are std::io::Error.

Parsing errors are std::num::ParseIntError.

If your program reads a file and parses numbers, main can't predict which specific error type will happen because there are multiple types.

Box<dyn std::error::Error> acts as a universal joker card here:

Error: The standard error trait covering all error types.

dyn (Dynamic): Allows any error type to fit into the bucket at runtime.

Box: A smart pointer wrapper to safely store these errors in memory.
