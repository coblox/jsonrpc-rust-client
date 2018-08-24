# JsonRPC Rust Client

[![Build Status](https://travis-ci.com/coblox/jsonrpc_rust_client.svg?token=EmrV8tpmLgu5PjZWH7QN&branch=master)](https://travis-ci.com/coblox/jsonrpc_rust_client)

This repository contains a simple Rust client for talking to JsonRPC APIs.

## Features

- Works without macros
- Based on the `reqwest` library
- Gives full control over the underlying HTTP client

## Limitations

At the moment, only JsonRPC v1 APIs are supported. Support for v2 APIs may be added in the future.

## Usage

See `examples/`.
