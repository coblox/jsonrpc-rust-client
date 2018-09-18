# JsonRPC Rust Client

[![Build Status](https://travis-ci.com/coblox/jsonrpc-rust-client.svg?token=EmrV8tpmLgu5PjZWH7QN&branch=master)](https://travis-ci.com/coblox/jsonrpc-rust-client)
[![Crates.io](https://img.shields.io/crates/v/jsonrpc_client.svg)](https://crates.io/crates/jsonrpc_client)

This repository contains a simple Rust client for talking to JsonRPC APIs.

## Features

- Works without macros
- Based on the `reqwest` library
- Gives full control over the underlying HTTP client

## Limitations

At the moment, only JsonRPC v1 APIs are supported. Support for v2 APIs may be added in the future.

## Usage

See `examples/`.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-Apache-2.0) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
