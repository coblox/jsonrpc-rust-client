extern crate jsonrpc;

use jsonrpc::*;

/// A trait defining the API you want to talk to. Not strictly necessary but nice for mocking.
trait HelloWorldApi {
    fn say_hello(&self, to: &str) -> Result<RpcResponse<String>, HTTPError>;
}

/// An actual implementation of your client
struct HelloWorld {
    client: RpcClient,
}

impl HelloWorldApi for HelloWorld {
    fn say_hello(&self, to: &str) -> Result<RpcResponse<String>, HTTPError> {
        self.client.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "test",
            "helloworld",
            to,
        ))
    }
}

fn main() {
    let api_client = HelloWorld {
        client: RpcClient::new(HTTPClient::new(), "http://example.org"),
    };

    let result = api_client
        .say_hello("World")
        .unwrap() // Handle network error
        .into_result()
        .unwrap(); // Handle RpcError

    println!("{}", result);
}
