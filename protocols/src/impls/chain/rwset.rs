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
//! Generated file from `src/protos/chain/rwset.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct RWSet {
    // message fields
    pub block_number: u64,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RWSet {
    fn default() -> &'a RWSet {
        <RWSet as ::protobuf::Message>::default_instance()
    }
}

impl RWSet {
    pub fn new() -> RWSet {
        ::std::default::Default::default()
    }

    // uint64 block_number = 1;


    pub fn get_block_number(&self) -> u64 {
        self.block_number
    }
    pub fn clear_block_number(&mut self) {
        self.block_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_number(&mut self, v: u64) {
        self.block_number = v;
    }
}

impl ::protobuf::Message for RWSet {
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
                    let tmp = is.read_uint64()?;
                    self.block_number = tmp;
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
        if self.block_number != 0 {
            my_size += ::protobuf::rt::value_size(1, self.block_number, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.block_number != 0 {
            os.write_uint64(1, self.block_number)?;
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

    fn new() -> RWSet {
        RWSet::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "block_number",
                |m: &RWSet| { &m.block_number },
                |m: &mut RWSet| { &mut m.block_number },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RWSet>(
                "RWSet",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RWSet {
        static instance: ::protobuf::rt::LazyV2<RWSet> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RWSet::new)
    }
}

impl ::protobuf::Clear for RWSet {
    fn clear(&mut self) {
        self.block_number = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RWSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RWSet {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ReadKey {
    // message fields
    pub key: ::std::string::String,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ReadKey {
    fn default() -> &'a ReadKey {
        <ReadKey as ::protobuf::Message>::default_instance()
    }
}

impl ReadKey {
    pub fn new() -> ReadKey {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }
}

impl ::protobuf::Message for ReadKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
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

    fn new() -> ReadKey {
        ReadKey::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "key",
                |m: &ReadKey| { &m.key },
                |m: &mut ReadKey| { &mut m.key },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ReadKey>(
                "ReadKey",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ReadKey {
        static instance: ::protobuf::rt::LazyV2<ReadKey> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ReadKey::new)
    }
}

impl ::protobuf::Clear for ReadKey {
    fn clear(&mut self) {
        self.key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadKey {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct WriteKey {
    // message fields
    pub key: ::std::string::String,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WriteKey {
    fn default() -> &'a WriteKey {
        <WriteKey as ::protobuf::Message>::default_instance()
    }
}

impl WriteKey {
    pub fn new() -> WriteKey {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }
}

impl ::protobuf::Message for WriteKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
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

    fn new() -> WriteKey {
        WriteKey::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "key",
                |m: &WriteKey| { &m.key },
                |m: &mut WriteKey| { &mut m.key },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<WriteKey>(
                "WriteKey",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static WriteKey {
        static instance: ::protobuf::rt::LazyV2<WriteKey> = ::protobuf::rt::LazyV2::INIT;
        instance.get(WriteKey::new)
    }
}

impl ::protobuf::Clear for WriteKey {
    fn clear(&mut self) {
        self.key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteKey {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1csrc/protos/chain/rwset.proto\x12\x05chain\"*\n\x05RWSet\x12!\n\x0c\
    block_number\x18\x01\x20\x01(\x04R\x0bblockNumber\"\x1b\n\x07ReadKey\x12\
    \x10\n\x03key\x18\x01\x20\x01(\tR\x03key\"\x1c\n\x08WriteKey\x12\x10\n\
    \x03key\x18\x01\x20\x01(\tR\x03keyBQ\n\x20cn.aberic.george.protocols.cha\
    inB\nRWSetProtoZ!github.com/george/protocols/chainb\x06proto3\
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
