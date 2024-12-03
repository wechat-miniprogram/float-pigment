# float-pigment

CSS and layout utilities for building user interfaces.

[![Crates.io Version](https://img.shields.io/crates/v/float-pigment?style=flat-square)](https://crates.io/crates/float-pigment)
[![Build Status](https://img.shields.io/github/actions/workflow/status/wechat-miniprogram/float-pigment/rust.yml?style=flat-square)](https://github.com/wechat-miniprogram/float-pigment/actions/workflows/rust.yml)

float-pigment is a group of low-level rust crates. It helps builds GUI applications with CSS-like technologies. These crates serve as low-level dependencies of UI frameworks.

Major crates:

| Crate Name | Description | Docs |
| ---------- | ----------- | ---- |
| <span style="white-space: nowrap">[float-pigment-css](https://crates.io/crates/float-pigment-css)</span> | CSS parser. | [![docs.rs](https://img.shields.io/docsrs/float-pigment-css?style=flat-square)](https://docs.rs/float-pigment-css) |
| <span style="white-space: nowrap">[float-pigment-layout](https://crates.io/crates/float-pigment-layout)</span> | Layout engine which supports common CSS `display`, including `flex` `block` and `inline`. | [![docs.rs](https://img.shields.io/docsrs/float-pigment-layout?style=flat-square)](https://docs.rs/float-pigment-layout) |
| <span style="white-space: nowrap">[float-pigment-forest](https://crates.io/crates/float-pigment-forest)</span> | Tree implementation that works with `float-pigment-layout`. | [![docs.rs](https://img.shields.io/docsrs/float-pigment-forest?style=flat-square)](https://docs.rs/float-pigment-forest) |
| <span style="white-space: nowrap">[float-pigment](https://crates.io/crates/float-pigment)</span> | The collection of all crates above, with C++ bindings. | [![docs.rs](https://img.shields.io/docsrs/float-pigment?style=flat-square)](https://docs.rs/float-pigment) |


## float-pigment-css

float-pigment-css can parse a practical subset of CSS, so that high-level libraries can perform CSS queries with ease.

Features:

* Parses CSS string.
* Perform queries on parsed structures.
* Supports a subset of CSS selectors and properties, and generate warnings wherever not supported.
* Serializes some parsed CSS into a more efficient binary format, which can be deserialized later.
* C++ bindings.


## float-pigment-layout

float-pigment-layout is a light-weight layout engine which supports common web layout algorithms, such as `display: block` and `display: flex`.

Features:

* Work with float-pigment-css for CSS types.
* High performance.
* Can (optionally) work with external text-layout engine to handle inline contents.
* C++ bindings.

Supported `display`:

* `block`
* `flex`
* `inline` `inline-block` `inline-flex` (if work with external text-layout engine)

Supported `position`:

* `relative`
* `absolute`


## float-pigment-forest

float-pigment-layout requires an external node tree implementation.

Usually the node tree should be implemented in high-level code or some other dedicated modules, but that is not always this case.

If you do not implement a node tree yourself, or the tree implementation is not in rust, this crate can help.


## LICENSE

Copyright 2024 wechat-miniprogram

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
