extern crate futures;
extern crate grpcio;
extern crate protobuf;

use futures::{Future, Stream};

mod bytestream;
mod bytestream_grpc;
mod server_impl;

fn main() {
    let _server = server_impl::ServerImpl::new();

    println!("{}", call_sync());
    println!("{}", call_future().wait().unwrap());
}

fn call_sync() -> String {
    let channel = grpcio::ChannelBuilder::new(
        std::sync::Arc::new(
            grpcio::Environment::new(1)
        )
    ).connect("localhost:8000");
    let client = bytestream_grpc::ByteStreamClient::new(channel);
    let stream = client.read(bytestream::ReadRequest::new());

    stream
        .map(|r| r.data)
        .map_err(|e| format!("{:?}", e))
        .concat2()
        .and_then(|val| String::from_utf8(val).map_err(|e| format!("{:?}", e)))
        .wait()
        .unwrap()

}

fn call_future() -> Box<Future<Item = String, Error = String>> {
    let channel = grpcio::ChannelBuilder::new(
        std::sync::Arc::new(
            grpcio::Environment::new(1)
        )
    ).connect("localhost:8000");
    let client = bytestream_grpc::ByteStreamClient::new(channel);
    let stream = client.read(bytestream::ReadRequest::new());

    Box::new(
        stream
            .map(|r| r.data)
            .map_err(|e| format!("{:?}", e))
            .concat2()
            .and_then(|val| String::from_utf8(val).map_err(|e| format!("{:?}", e))),
    )
}
