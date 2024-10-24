# from-c-use-rust
A Rust static library called from C.

## Features
- Creates static library in Rust
- Calls cbindgen to create the C-header files
- Demo C-project to call the Rust functions
- Using valgrind to prove there are no memory leaks

## Usage
```Bash
cargo build --release
cd c-project
make
```

