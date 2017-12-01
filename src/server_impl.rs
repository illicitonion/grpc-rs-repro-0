use super::{bytestream, bytestream_grpc};
use futures;
use grpcio;

use futures::Future;
use std::sync::Arc;

pub struct ServerImpl {
    _server_transport: grpcio::Server,
}

impl ServerImpl {
    pub fn new() -> ServerImpl {
        let env = Arc::new(grpcio::Environment::new(1));
        let responder = Responder {};
        let mut _server_transport = grpcio::ServerBuilder::new(env)
            .register_service(bytestream_grpc::create_byte_stream(responder.clone()))
            .bind("localhost", 8000)
            .build()
            .unwrap();
        _server_transport.start();

        ServerImpl { _server_transport }
    }
}

#[derive(Clone, Debug)]
pub struct Responder {}

impl Responder {
    fn send<Item, S>(
        &self,
        ctx: grpcio::RpcContext,
        sink: grpcio::ServerStreamingSink<Item>,
        stream: S,
    ) where
        Item: Send + 'static,
        S: futures::Stream<Item = (Item, grpcio::WriteFlags), Error = grpcio::Error> + Send + 'static,
    {
        ctx.spawn(stream.forward(sink).map(|_| ()).map_err(|_| ()));
    }
}

impl bytestream_grpc::ByteStream for Responder {
    fn read(
        &self,
        ctx: grpcio::RpcContext,
        _req: bytestream::ReadRequest,
        sink: grpcio::ServerStreamingSink<bytestream::ReadResponse>,
    ) {
        self.send(
            ctx,
            sink,
            futures::stream::iter_ok(vec![
                (
                    {
                        let mut response = bytestream::ReadResponse::new();
                        response.set_data("a".as_bytes().to_vec());
                        response
                    },
                    grpcio::WriteFlags::default()
                ),
                (
                    {
                        let mut response = bytestream::ReadResponse::new();
                        response.set_data("b".as_bytes().to_vec());
                        response
                    },
                    grpcio::WriteFlags::default()
                ),
            ]),
        )
    }

    fn write(
        &self,
        _ctx: grpcio::RpcContext,
        _stream: grpcio::RequestStream<bytestream::WriteRequest>,
        sink: grpcio::ClientStreamingSink<bytestream::WriteResponse>,
    ) {
        sink.fail(grpcio::RpcStatus::new(
            grpcio::RpcStatusCode::Unimplemented,
            None,
        ));
    }

    fn query_write_status(
        &self,
        _ctx: grpcio::RpcContext,
        _req: bytestream::QueryWriteStatusRequest,
        sink: grpcio::UnarySink<bytestream::QueryWriteStatusResponse>,
    ) {
        sink.fail(grpcio::RpcStatus::new(
            grpcio::RpcStatusCode::Unimplemented,
            None,
        ));
    }
}
