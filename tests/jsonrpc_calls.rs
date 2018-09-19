extern crate env_logger;
extern crate jsonrpc_client;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate spectral;

use jsonrpc_client::*;
use jsonrpc_core::*;
use jsonrpc_http_server::*;
use spectral::prelude::*;

fn main() {}

#[test]

fn get_server_response() {
    let _ = env_logger::try_init();

    let mut io = IoHandler::new();
    io.add_method("helloworld", |_: Params| {
        Ok(Value::String("hello".to_string()))
    });
    let server = ServerBuilder::new(io)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();
    let client = RpcClient::new(HTTPClient::new(), "http://127.0.0.1:3030");

    let res =
        client.send::<String, ()>(&RpcRequest::new0(JsonRpcVersion::V2, "test", "helloworld"));

    assert_that(&res)
        .is_ok()
        .is_ok()
        .is_equal_to(&String::from("hello"));

    server.close();
}
