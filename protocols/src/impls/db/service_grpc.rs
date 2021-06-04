// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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


// server interface

pub trait PageService {
    fn pages(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::service::Request>, resp: ::grpc::ServerResponseUnarySink<super::page::PageList>) -> ::grpc::Result<()>;

    fn page_create(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::page::RequestPageCreate>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn page_modify(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::page::RequestPageModify>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn page_info(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::page::RequestPageInfo>, resp: ::grpc::ServerResponseUnarySink<super::page::ResponsePageInfo>) -> ::grpc::Result<()>;
}

// client

pub struct PageServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for PageServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        PageServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl PageServiceClient {
    pub fn pages(&self, o: ::grpc::RequestOptions, req: super::service::Request) -> ::grpc::SingleResponse<super::page::PageList> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.PageService/pages"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn page_create(&self, o: ::grpc::RequestOptions, req: super::page::RequestPageCreate) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.PageService/page_create"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn page_modify(&self, o: ::grpc::RequestOptions, req: super::page::RequestPageModify) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.PageService/page_modify"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn page_info(&self, o: ::grpc::RequestOptions, req: super::page::RequestPageInfo) -> ::grpc::SingleResponse<super::page::ResponsePageInfo> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.PageService/page_info"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct PageServiceServer;


impl PageServiceServer {
    pub fn new_service_def<H : PageService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/db.PageService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.PageService/pages"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).pages(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.PageService/page_create"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).page_create(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.PageService/page_modify"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).page_modify(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.PageService/page_info"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).page_info(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait DatabaseService {
    fn databases(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::service::Request>, resp: ::grpc::ServerResponseUnarySink<super::database::DatabaseList>) -> ::grpc::Result<()>;
}

// client

pub struct DatabaseServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for DatabaseServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        DatabaseServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl DatabaseServiceClient {
    pub fn databases(&self, o: ::grpc::RequestOptions, req: super::service::Request) -> ::grpc::SingleResponse<super::database::DatabaseList> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/databases"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct DatabaseServiceServer;


impl DatabaseServiceServer {
    pub fn new_service_def<H : DatabaseService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/db.DatabaseService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/databases"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).databases(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait ViewService {
    fn views(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::view::RequestViewList>, resp: ::grpc::ServerResponseUnarySink<super::view::ViewList>) -> ::grpc::Result<()>;
}

// client

pub struct ViewServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for ViewServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        ViewServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl ViewServiceClient {
    pub fn views(&self, o: ::grpc::RequestOptions, req: super::view::RequestViewList) -> ::grpc::SingleResponse<super::view::ViewList> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/views"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct ViewServiceServer;


impl ViewServiceServer {
    pub fn new_service_def<H : ViewService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/db.ViewService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/views"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).views(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait IndexService {
    fn indexes(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::index::RequestIndexList>, resp: ::grpc::ServerResponseUnarySink<super::index::IndexList>) -> ::grpc::Result<()>;
}

// client

pub struct IndexServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for IndexServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        IndexServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl IndexServiceClient {
    pub fn indexes(&self, o: ::grpc::RequestOptions, req: super::index::RequestIndexList) -> ::grpc::SingleResponse<super::index::IndexList> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.IndexService/indexes"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct IndexServiceServer;


impl IndexServiceServer {
    pub fn new_service_def<H : IndexService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/db.IndexService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.IndexService/indexes"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).indexes(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
