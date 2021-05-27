/*
 * Copyright (c) 2021. Aberic - All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// This file is generated by rust-protobuf 2.18.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `src/protos/chain/service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct BlockGet {
    // message oneof groups
    pub get: ::std::option::Option<BlockGet_oneof_get>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BlockGet {
    fn default() -> &'a BlockGet {
        <BlockGet as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum BlockGet_oneof_get {
    height(u32),
    hash(::std::string::String),
    tx_hash(::std::string::String),
}

impl BlockGet {
    pub fn new() -> BlockGet {
        ::std::default::Default::default()
    }

    // uint32 height = 1;


    pub fn get_height(&self) -> u32 {
        match self.get {
            ::std::option::Option::Some(BlockGet_oneof_get::height(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_height(&mut self) {
        self.get = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        match self.get {
            ::std::option::Option::Some(BlockGet_oneof_get::height(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.get = ::std::option::Option::Some(BlockGet_oneof_get::height(v))
    }

    // string hash = 2;


    pub fn get_hash(&self) -> &str {
        match self.get {
            ::std::option::Option::Some(BlockGet_oneof_get::hash(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_hash(&mut self) {
        self.get = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        match self.get {
            ::std::option::Option::Some(BlockGet_oneof_get::hash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::string::String) {
        self.get = ::std::option::Option::Some(BlockGet_oneof_get::hash(v))
    }

    // Mutable pointer to the field.
    pub fn mut_hash(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(BlockGet_oneof_get::hash(_)) = self.get {
        } else {
            self.get = ::std::option::Option::Some(BlockGet_oneof_get::hash(::std::string::String::new()));
        }
        match self.get {
            ::std::option::Option::Some(BlockGet_oneof_get::hash(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::string::String {
        if self.has_hash() {
            match self.get.take() {
                ::std::option::Option::Some(BlockGet_oneof_get::hash(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string tx_hash = 3;


    pub fn get_tx_hash(&self) -> &str {
        match self.get {
            ::std::option::Option::Some(BlockGet_oneof_get::tx_hash(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_tx_hash(&mut self) {
        self.get = ::std::option::Option::None;
    }

    pub fn has_tx_hash(&self) -> bool {
        match self.get {
            ::std::option::Option::Some(BlockGet_oneof_get::tx_hash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tx_hash(&mut self, v: ::std::string::String) {
        self.get = ::std::option::Option::Some(BlockGet_oneof_get::tx_hash(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tx_hash(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(BlockGet_oneof_get::tx_hash(_)) = self.get {
        } else {
            self.get = ::std::option::Option::Some(BlockGet_oneof_get::tx_hash(::std::string::String::new()));
        }
        match self.get {
            ::std::option::Option::Some(BlockGet_oneof_get::tx_hash(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tx_hash(&mut self) -> ::std::string::String {
        if self.has_tx_hash() {
            match self.get.take() {
                ::std::option::Option::Some(BlockGet_oneof_get::tx_hash(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for BlockGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.get = ::std::option::Option::Some(BlockGet_oneof_get::height(is.read_uint32()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.get = ::std::option::Option::Some(BlockGet_oneof_get::hash(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.get = ::std::option::Option::Some(BlockGet_oneof_get::tx_hash(is.read_string()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.get {
            match v {
                &BlockGet_oneof_get::height(v) => {
                    my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &BlockGet_oneof_get::hash(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
                &BlockGet_oneof_get::tx_hash(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.get {
            match v {
                &BlockGet_oneof_get::height(v) => {
                    os.write_uint32(1, v)?;
                },
                &BlockGet_oneof_get::hash(ref v) => {
                    os.write_string(2, v)?;
                },
                &BlockGet_oneof_get::tx_hash(ref v) => {
                    os.write_string(3, v)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BlockGet {
        BlockGet::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                "height",
                BlockGet::has_height,
                BlockGet::get_height,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "hash",
                BlockGet::has_hash,
                BlockGet::get_hash,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "tx_hash",
                BlockGet::has_tx_hash,
                BlockGet::get_tx_hash,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<BlockGet>(
                "BlockGet",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static BlockGet {
        static instance: ::protobuf::rt::LazyV2<BlockGet> = ::protobuf::rt::LazyV2::INIT;
        instance.get(BlockGet::new)
    }
}

impl ::protobuf::Clear for BlockGet {
    fn clear(&mut self) {
        self.get = ::std::option::Option::None;
        self.get = ::std::option::Option::None;
        self.get = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockGet {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Response {
    // message fields
    pub status: Status,
    pub error: ::std::string::String,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Response {
    fn default() -> &'a Response {
        <Response as ::protobuf::Message>::default_instance()
    }
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    // .chain.Status status = 1;


    pub fn get_status(&self) -> Status {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = Status::SUCCESS;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    // string error = 2;


    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != Status::SUCCESS {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.status != Status::SUCCESS {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.status))?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                "status",
                |m: &Response| { &m.status },
                |m: &mut Response| { &mut m.status },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "error",
                |m: &Response| { &m.error },
                |m: &mut Response| { &mut m.error },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Response>(
                "Response",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Response {
        static instance: ::protobuf::rt::LazyV2<Response> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Response::new)
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.status = Status::SUCCESS;
        self.error.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum Status {
    SUCCESS = 0,
    FAIL = 1,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::SUCCESS),
            1 => ::std::option::Option::Some(Status::FAIL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::SUCCESS,
            Status::FAIL,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Status>("Status", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Status {
}

impl ::std::default::Default for Status {
    fn default() -> Self {
        Status::SUCCESS
    }
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1esrc/protos/chain/service.proto\x12\x05chain\x1a\x1csrc/protos/chai\
    n/block.proto\"\\\n\x08BlockGet\x12\x18\n\x06height\x18\x01\x20\x01(\rH\
    \0R\x06height\x12\x14\n\x04hash\x18\x02\x20\x01(\tH\0R\x04hash\x12\x19\n\
    \x07tx_hash\x18\x03\x20\x01(\tH\0R\x06txHashB\x05\n\x03get\"G\n\x08Respo\
    nse\x12%\n\x06status\x18\x01\x20\x01(\x0e2\r.chain.StatusR\x06status\x12\
    \x14\n\x05error\x18\x02\x20\x01(\tR\x05error*\x1f\n\x06Status\x12\x0b\n\
    \x07SUCCESS\x10\0\x12\x08\n\x04FAIL\x10\x012T\n\x06Blocks\x12$\n\x03add\
    \x12\x0c.chain.Block\x1a\x0f.chain.Response\x12$\n\x03get\x12\x0f.chain.\
    BlockGet\x1a\x0c.chain.BlockBS\n\x20cn.aberic.george.protocols.chainB\
    \x0cServiceProtoZ!github.com/george/protocols/chainb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
