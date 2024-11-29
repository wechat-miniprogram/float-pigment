# float-pigment

A collection of major float-pigment crates.

This is a crate of the [float-pigment](https://github.com/wechat-miniprogram/float-pigment) project. See its documentation for details.


## Generate C++ Headers

This module contains C++ bindings, and it generates a C++ header file for visiting from C++. However, the C++ header should be updated manually.

To update the C++ header, make sure installed:

* [cargo-expand](https://github.com/dtolnay/cargo-expand)

The C++ header update command:

```shell
cargo run --bin float_pigment_cpp_binding_gen_tool
```
