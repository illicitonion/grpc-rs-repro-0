// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_BYTE_STREAM_READ: ::grpcio::Method<
    super::bytestream::ReadRequest,
    super::bytestream::ReadResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/google.bytestream.ByteStream/Read",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_BYTE_STREAM_WRITE: ::grpcio::Method<
    super::bytestream::WriteRequest,
    super::bytestream::WriteResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/google.bytestream.ByteStream/Write",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_BYTE_STREAM_QUERY_WRITE_STATUS: ::grpcio::Method<super::bytestream::QueryWriteStatusRequest, super::bytestream::QueryWriteStatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.bytestream.ByteStream/QueryWriteStatus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ByteStreamClient {
    client: ::grpcio::Client,
}

impl ByteStreamClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ByteStreamClient { client: ::grpcio::Client::new(channel) }
    }

    pub fn read_opt(
        &self,
        req: super::bytestream::ReadRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::ClientSStreamReceiver<super::bytestream::ReadResponse> {
        self.client.server_streaming(
            &METHOD_BYTE_STREAM_READ,
            req,
            opt,
        )
    }

    pub fn read(
        &self,
        req: super::bytestream::ReadRequest,
    ) -> ::grpcio::ClientSStreamReceiver<super::bytestream::ReadResponse> {
        self.read_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> (::grpcio::ClientCStreamSender<super::bytestream::WriteRequest>,
              ::grpcio::ClientCStreamReceiver<super::bytestream::WriteResponse>) {
        self.client.client_streaming(&METHOD_BYTE_STREAM_WRITE, opt)
    }

    pub fn write(
        &self,
    ) -> (::grpcio::ClientCStreamSender<super::bytestream::WriteRequest>,
              ::grpcio::ClientCStreamReceiver<super::bytestream::WriteResponse>) {
        self.write_opt(::grpcio::CallOption::default())
    }

    pub fn query_write_status_opt(
        &self,
        req: super::bytestream::QueryWriteStatusRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::bytestream::QueryWriteStatusResponse> {
        self.client.unary_call(
            &METHOD_BYTE_STREAM_QUERY_WRITE_STATUS,
            req,
            opt,
        )
    }

    pub fn query_write_status(
        &self,
        req: super::bytestream::QueryWriteStatusRequest,
    ) -> ::grpcio::Result<super::bytestream::QueryWriteStatusResponse> {
        self.query_write_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_write_status_async_opt(
        &self,
        req: super::bytestream::QueryWriteStatusRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::ClientUnaryReceiver<super::bytestream::QueryWriteStatusResponse> {
        self.client.unary_call_async(
            &METHOD_BYTE_STREAM_QUERY_WRITE_STATUS,
            req,
            opt,
        )
    }

    pub fn query_write_status_async(
        &self,
        req: super::bytestream::QueryWriteStatusRequest,
    ) -> ::grpcio::ClientUnaryReceiver<super::bytestream::QueryWriteStatusResponse> {
        self.query_write_status_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait ByteStream {
    fn read(
        &self,
        ctx: ::grpcio::RpcContext,
        req: super::bytestream::ReadRequest,
        sink: ::grpcio::ServerStreamingSink<super::bytestream::ReadResponse>,
    );
    fn write(
        &self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<super::bytestream::WriteRequest>,
        sink: ::grpcio::ClientStreamingSink<super::bytestream::WriteResponse>,
    );
    fn query_write_status(
        &self,
        ctx: ::grpcio::RpcContext,
        req: super::bytestream::QueryWriteStatusRequest,
        sink: ::grpcio::UnarySink<super::bytestream::QueryWriteStatusResponse>,
    );
}

pub fn create_byte_stream<S: ByteStream + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(
        &METHOD_BYTE_STREAM_READ,
        move |ctx, req, resp| instance.read(ctx, req, resp),
    );
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(
        &METHOD_BYTE_STREAM_WRITE,
        move |ctx, req, resp| instance.write(ctx, req, resp),
    );
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BYTE_STREAM_QUERY_WRITE_STATUS, move |ctx,
          req,
          resp| {
        instance.query_write_status(ctx, req, resp)
    });
    builder.build()
}
