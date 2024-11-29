# float-pigment-css

The CSS parser for the float-pigment project.

This is a crate of the [float-pigment](https://github.com/wechat-miniprogram/float-pigment) project. See its documentation for details.


## Compile to WebAssembly

In most cases this module is a dependency for high-level modules.

But more, this module can be compiled to WebAssembly itself, and can be called from JavaScript.

With [wasm-pack](https://github.com/rustwasm/wasm-pack) installed globally:

```shell
wasm-pack build float-pigment-css --target nodejs --features wasm-entrance
```


## Generate C++ Headers

This module contains C++ bindings, and it generates a C++ header file for visiting from C++. However, the C++ header should be updated manually.

To update the C++ header, make sure installed:

* [cargo-expand](https://github.com/dtolnay/cargo-expand)

The C++ header update command:

```shell
cargo run --bin float_pigment_css_cpp_binding_gen_tool --features build-cpp-header
```


## Compatibility Checks

Some `serde` structs and enums needs compatibility checks.

This is because the `serialize_bincode` and `deserialize_bincode` forbid some changes to keep compatibilities across versions.
It means that structs and enums can only *add new fields* but not *modify fields*.
This is done by compare the current version of struct/enum definitions and the corresponding previous one.
The previous struct/enum definitions are stored in the `compile_cache` dir.

For new struct/enum that needs across-version compatibilities, `CompatibilityStructCheck` and `CompatibilityEnumCheck` macros must be derived.
When struct/enum name conflicts, `compatibility_struct_check` can be used to specify a name prefix. For example:

```rust
#[derive(Serialize, Deserialize, CompatibilityEnumCheck)]
enum Name {
   None
}               

#[derive(Serialize, Deserialize, CompatibilityStructCheck)]
struct Hello {
    name: String
}

#[compatibility_struct_check(mod_name)]
#[derive(Serialize, Deserialize)]
struct Hello {
    name: String
}
```

To update the `compile_cache`, this steps should be followed:

* Make sure the HEAD have the published commit merged.
* Update the cargo version of float-pigment-css.
* Run `cargo run --bin float_pigment_css_update_version`.
* The `compile_cache/publish/version.toml` should be updated.
* Run `cargo run --bin publish` (it will git-tag and git-push).
