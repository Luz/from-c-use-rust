# from-c-use-rust
A Rust static library called from C.

## Features
- Creates static library in Rust
- Calls cbindgen to create the C-header files
- Demo C-project to call the Rust functions
- Using valgrind to prove there are no memory leaks

## Dependencies
- Rust and C tooling
- valgrind

## Usage
```Bash
cargo build --release
cd c-project
make
```

## Notes
### Vec
Vectors in Rust do not have a defined ABI, therefore we can not pass them over to C.
There are expensive workarounds, but we also have the option to chose a simpler solution.
In this code, the vectors are not passed to C at all, only handles to it.

### Raw pointers
We pass handles of Rust objects to C and later need them back for further processing.
Cbindgen shall create the necessary structs, as we do not want to use void pointers.
Therefore prefer 'handles to these structs' over 'c_void'. (-> dont "use std::os::raw::c_void")
By omitting the "#[repr(C)]" for such handles, cbindgen creates those structs without failing.
For example, see the Path struct in the Rust code.

