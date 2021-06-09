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

pub trait UserService {
    fn login(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::user::RequestLogin>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;
}

// client

pub struct UserServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for UserServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        UserServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl UserServiceClient {
    pub fn login(&self, o: ::grpc::RequestOptions, req: super::user::RequestLogin) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.UserService/login"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct UserServiceServer;


impl UserServiceServer {
    pub fn new_service_def<H : UserService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/db.UserService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.UserService/login"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).login(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait PageService {
    fn pages(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::service::Request>, resp: ::grpc::ServerResponseUnarySink<super::page::PageList>) -> ::grpc::Result<()>;

    fn page_create(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::page::RequestPageCreate>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn page_modify(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::page::RequestPageModify>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn page_info(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::page::RequestPageInfo>, resp: ::grpc::ServerResponseUnarySink<super::page::ResponsePageInfo>) -> ::grpc::Result<()>;

    fn page_remove(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::page::RequestPageRemove>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;
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

    pub fn page_remove(&self, o: ::grpc::RequestOptions, req: super::page::RequestPageRemove) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.PageService/page_remove"),
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
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.PageService/page_remove"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).page_remove(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait DatabaseService {
    fn databases(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::service::Request>, resp: ::grpc::ServerResponseUnarySink<super::database::DatabaseList>) -> ::grpc::Result<()>;

    fn database_create(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::database::RequestDatabaseCreate>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn database_modify(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::database::RequestDatabaseModify>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn database_info(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::database::RequestDatabaseInfo>, resp: ::grpc::ServerResponseUnarySink<super::database::ResponseDatabaseInfo>) -> ::grpc::Result<()>;

    fn database_remove(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::database::RequestDatabaseRemove>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;
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

    pub fn database_create(&self, o: ::grpc::RequestOptions, req: super::database::RequestDatabaseCreate) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/database_create"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn database_modify(&self, o: ::grpc::RequestOptions, req: super::database::RequestDatabaseModify) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/database_modify"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn database_info(&self, o: ::grpc::RequestOptions, req: super::database::RequestDatabaseInfo) -> ::grpc::SingleResponse<super::database::ResponseDatabaseInfo> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/database_info"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn database_remove(&self, o: ::grpc::RequestOptions, req: super::database::RequestDatabaseRemove) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/database_remove"),
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
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/database_create"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).database_create(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/database_modify"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).database_modify(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/database_info"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).database_info(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DatabaseService/database_remove"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).database_remove(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait ViewService {
    fn views(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::view::RequestViewList>, resp: ::grpc::ServerResponseUnarySink<super::view::ViewList>) -> ::grpc::Result<()>;

    fn view_create(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::view::RequestViewCreate>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn view_modify(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::view::RequestViewModify>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn view_info(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::view::RequestViewInfo>, resp: ::grpc::ServerResponseUnarySink<super::view::ResponseViewInfo>) -> ::grpc::Result<()>;

    fn view_remove(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::view::RequestViewRemove>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn view_archive(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::view::RequestViewArchive>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn view_record(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::view::RequestViewRecord>, resp: ::grpc::ServerResponseUnarySink<super::view::ResponseViewRecord>) -> ::grpc::Result<()>;
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

    pub fn view_create(&self, o: ::grpc::RequestOptions, req: super::view::RequestViewCreate) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_create"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn view_modify(&self, o: ::grpc::RequestOptions, req: super::view::RequestViewModify) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_modify"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn view_info(&self, o: ::grpc::RequestOptions, req: super::view::RequestViewInfo) -> ::grpc::SingleResponse<super::view::ResponseViewInfo> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_info"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn view_remove(&self, o: ::grpc::RequestOptions, req: super::view::RequestViewRemove) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_remove"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn view_archive(&self, o: ::grpc::RequestOptions, req: super::view::RequestViewArchive) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_archive"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn view_record(&self, o: ::grpc::RequestOptions, req: super::view::RequestViewRecord) -> ::grpc::SingleResponse<super::view::ResponseViewRecord> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_record"),
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
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_create"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).view_create(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_modify"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).view_modify(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_info"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).view_info(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_remove"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).view_remove(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_archive"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).view_archive(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.ViewService/view_record"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).view_record(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait IndexService {
    fn indexes(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::index::RequestIndexList>, resp: ::grpc::ServerResponseUnarySink<super::index::IndexList>) -> ::grpc::Result<()>;

    fn index_create(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::index::RequestIndexCreate>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn index_info(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::index::RequestIndexInfo>, resp: ::grpc::ServerResponseUnarySink<super::index::ResponseIndexInfo>) -> ::grpc::Result<()>;
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

    pub fn index_create(&self, o: ::grpc::RequestOptions, req: super::index::RequestIndexCreate) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.IndexService/index_create"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn index_info(&self, o: ::grpc::RequestOptions, req: super::index::RequestIndexInfo) -> ::grpc::SingleResponse<super::index::ResponseIndexInfo> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.IndexService/index_info"),
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
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.IndexService/index_create"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).index_create(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.IndexService/index_info"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).index_info(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait DiskService {
    fn put(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::disk::RequestDiskInto>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn set(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::disk::RequestDiskInto>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn get(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::disk::RequestDiskOut>, resp: ::grpc::ServerResponseUnarySink<super::disk::ResponseDiskOut>) -> ::grpc::Result<()>;

    fn get_by_index(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::disk::RequestDiskIOut>, resp: ::grpc::ServerResponseUnarySink<super::disk::ResponseDiskOut>) -> ::grpc::Result<()>;

    fn remove(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::disk::RequestDiskRemove>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn select(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::disk::RequestDiskSelect>, resp: ::grpc::ServerResponseUnarySink<super::disk::ResponseDiskSelect>) -> ::grpc::Result<()>;

    fn delete(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::disk::RequestDiskDelete>, resp: ::grpc::ServerResponseUnarySink<super::disk::ResponseDiskDelete>) -> ::grpc::Result<()>;
}

// client

pub struct DiskServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for DiskServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        DiskServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl DiskServiceClient {
    pub fn put(&self, o: ::grpc::RequestOptions, req: super::disk::RequestDiskInto) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/put"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn set(&self, o: ::grpc::RequestOptions, req: super::disk::RequestDiskInto) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/set"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get(&self, o: ::grpc::RequestOptions, req: super::disk::RequestDiskOut) -> ::grpc::SingleResponse<super::disk::ResponseDiskOut> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/get"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_by_index(&self, o: ::grpc::RequestOptions, req: super::disk::RequestDiskIOut) -> ::grpc::SingleResponse<super::disk::ResponseDiskOut> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/get_by_index"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn remove(&self, o: ::grpc::RequestOptions, req: super::disk::RequestDiskRemove) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/remove"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn select(&self, o: ::grpc::RequestOptions, req: super::disk::RequestDiskSelect) -> ::grpc::SingleResponse<super::disk::ResponseDiskSelect> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/select"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn delete(&self, o: ::grpc::RequestOptions, req: super::disk::RequestDiskDelete) -> ::grpc::SingleResponse<super::disk::ResponseDiskDelete> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/delete"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct DiskServiceServer;


impl DiskServiceServer {
    pub fn new_service_def<H : DiskService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/db.DiskService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/put"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).put(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/set"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).set(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/get"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/get_by_index"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_by_index(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/remove"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).remove(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/select"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).select(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.DiskService/delete"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).delete(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}

// server interface

pub trait MemoryService {
    fn put(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::memory::RequestMemoryInto>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn set(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::memory::RequestMemoryInto>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn get(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::memory::RequestMemoryOut>, resp: ::grpc::ServerResponseUnarySink<super::memory::ResponseMemoryOut>) -> ::grpc::Result<()>;

    fn remove(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::memory::RequestMemoryRemove>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn put_by_page(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::memory::RequestMemoryPInto>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn set_by_page(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::memory::RequestMemoryPInto>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;

    fn get_by_page(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::memory::RequestMemoryPOut>, resp: ::grpc::ServerResponseUnarySink<super::memory::ResponseMemoryPOut>) -> ::grpc::Result<()>;

    fn remove_by_page(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::memory::RequestMemoryPRemove>, resp: ::grpc::ServerResponseUnarySink<super::service::Response>) -> ::grpc::Result<()>;
}

// client

pub struct MemoryServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for MemoryServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        MemoryServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl MemoryServiceClient {
    pub fn put(&self, o: ::grpc::RequestOptions, req: super::memory::RequestMemoryInto) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/put"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn set(&self, o: ::grpc::RequestOptions, req: super::memory::RequestMemoryInto) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/set"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get(&self, o: ::grpc::RequestOptions, req: super::memory::RequestMemoryOut) -> ::grpc::SingleResponse<super::memory::ResponseMemoryOut> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/get"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn remove(&self, o: ::grpc::RequestOptions, req: super::memory::RequestMemoryRemove) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/remove"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn put_by_page(&self, o: ::grpc::RequestOptions, req: super::memory::RequestMemoryPInto) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/put_by_page"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn set_by_page(&self, o: ::grpc::RequestOptions, req: super::memory::RequestMemoryPInto) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/set_by_page"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_by_page(&self, o: ::grpc::RequestOptions, req: super::memory::RequestMemoryPOut) -> ::grpc::SingleResponse<super::memory::ResponseMemoryPOut> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/get_by_page"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn remove_by_page(&self, o: ::grpc::RequestOptions, req: super::memory::RequestMemoryPRemove) -> ::grpc::SingleResponse<super::service::Response> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/remove_by_page"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct MemoryServiceServer;


impl MemoryServiceServer {
    pub fn new_service_def<H : MemoryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/db.MemoryService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/put"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).put(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/set"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).set(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/get"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/remove"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).remove(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/put_by_page"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).put_by_page(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/set_by_page"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).set_by_page(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/get_by_page"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_by_page(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/db.MemoryService/remove_by_page"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).remove_by_page(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
