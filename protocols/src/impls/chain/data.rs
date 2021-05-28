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
//! Generated file from `src/protos/chain/data.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Data {
    // message fields
    pub header: ::protobuf::SingularPtrField<Header>,
    pub info: ::std::vec::Vec<u8>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Data {
    fn default() -> &'a Data {
        <Data as ::protobuf::Message>::default_instance()
    }
}

impl Data {
    pub fn new() -> Data {
        ::std::default::Default::default()
    }

    // .chain.Header header = 1;


    pub fn get_header(&self) -> &Header {
        self.header.as_ref().unwrap_or_else(|| <Header as ::protobuf::Message>::default_instance())
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: Header) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut Header {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> Header {
        self.header.take().unwrap_or_else(|| Header::new())
    }

    // bytes info = 2;


    pub fn get_info(&self) -> &[u8] {
        &self.info
    }
    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::vec::Vec<u8>) {
        self.info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.info
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.info, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Data {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.info)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.info.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.info);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.info.is_empty() {
            os.write_bytes(2, &self.info)?;
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

    fn new() -> Data {
        Data::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Header>>(
                "header",
                |m: &Data| { &m.header },
                |m: &mut Data| { &mut m.header },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "info",
                |m: &Data| { &m.info },
                |m: &mut Data| { &mut m.info },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Data>(
                "Data",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Data {
        static instance: ::protobuf::rt::LazyV2<Data> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Data::new)
    }
}

impl ::protobuf::Clear for Data {
    fn clear(&mut self) {
        self.header.clear();
        self.info.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Data {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Header {
    // message fields
    pub field_type: HeaderType,
    pub signature: ::std::vec::Vec<u8>,
    pub ledger: ::std::vec::Vec<u8>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Header {
    fn default() -> &'a Header {
        <Header as ::protobuf::Message>::default_instance()
    }
}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    // .chain.HeaderType type = 1;


    pub fn get_field_type(&self) -> HeaderType {
        self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type = HeaderType::UNDEFINED;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: HeaderType) {
        self.field_type = v;
    }

    // bytes signature = 2;


    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }
    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    // bytes ledger = 3;


    pub fn get_ledger(&self) -> &[u8] {
        &self.ledger
    }
    pub fn clear_ledger(&mut self) {
        self.ledger.clear();
    }

    // Param is passed by value, moved
    pub fn set_ledger(&mut self, v: ::std::vec::Vec<u8>) {
        self.ledger = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ledger(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.ledger
    }

    // Take field
    pub fn take_ledger(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.ledger, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Header {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.ledger)?;
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
        if self.field_type != HeaderType::UNDEFINED {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.signature);
        }
        if !self.ledger.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.ledger);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != HeaderType::UNDEFINED {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.field_type))?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(2, &self.signature)?;
        }
        if !self.ledger.is_empty() {
            os.write_bytes(3, &self.ledger)?;
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

    fn new() -> Header {
        Header::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<HeaderType>>(
                "type",
                |m: &Header| { &m.field_type },
                |m: &mut Header| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "signature",
                |m: &Header| { &m.signature },
                |m: &mut Header| { &mut m.signature },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "ledger",
                |m: &Header| { &m.ledger },
                |m: &mut Header| { &mut m.ledger },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Header>(
                "Header",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Header {
        static instance: ::protobuf::rt::LazyV2<Header> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Header::new)
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.field_type = HeaderType::UNDEFINED;
        self.signature.clear();
        self.ledger.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Header {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct SignatureHeader {
    // message fields
    pub creator: ::std::vec::Vec<u8>,
    pub sign: ::std::vec::Vec<u8>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SignatureHeader {
    fn default() -> &'a SignatureHeader {
        <SignatureHeader as ::protobuf::Message>::default_instance()
    }
}

impl SignatureHeader {
    pub fn new() -> SignatureHeader {
        ::std::default::Default::default()
    }

    // bytes creator = 1;


    pub fn get_creator(&self) -> &[u8] {
        &self.creator
    }
    pub fn clear_creator(&mut self) {
        self.creator.clear();
    }

    // Param is passed by value, moved
    pub fn set_creator(&mut self, v: ::std::vec::Vec<u8>) {
        self.creator = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creator(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.creator
    }

    // Take field
    pub fn take_creator(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.creator, ::std::vec::Vec::new())
    }

    // bytes sign = 2;


    pub fn get_sign(&self) -> &[u8] {
        &self.sign
    }
    pub fn clear_sign(&mut self) {
        self.sign.clear();
    }

    // Param is passed by value, moved
    pub fn set_sign(&mut self, v: ::std::vec::Vec<u8>) {
        self.sign = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sign(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.sign
    }

    // Take field
    pub fn take_sign(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.sign, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for SignatureHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.creator)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.sign)?;
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
        if !self.creator.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.creator);
        }
        if !self.sign.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.sign);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.creator.is_empty() {
            os.write_bytes(1, &self.creator)?;
        }
        if !self.sign.is_empty() {
            os.write_bytes(2, &self.sign)?;
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

    fn new() -> SignatureHeader {
        SignatureHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "creator",
                |m: &SignatureHeader| { &m.creator },
                |m: &mut SignatureHeader| { &mut m.creator },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "sign",
                |m: &SignatureHeader| { &m.sign },
                |m: &mut SignatureHeader| { &mut m.sign },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SignatureHeader>(
                "SignatureHeader",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SignatureHeader {
        static instance: ::protobuf::rt::LazyV2<SignatureHeader> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SignatureHeader::new)
    }
}

impl ::protobuf::Clear for SignatureHeader {
    fn clear(&mut self) {
        self.creator.clear();
        self.sign.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignatureHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignatureHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct LedgerHeader {
    // message fields
    pub id: ::std::string::String,
    pub version: u32,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LedgerHeader {
    fn default() -> &'a LedgerHeader {
        <LedgerHeader as ::protobuf::Message>::default_instance()
    }
}

impl LedgerHeader {
    pub fn new() -> LedgerHeader {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // uint32 version = 2;


    pub fn get_version(&self) -> u32 {
        self.version
    }
    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

impl ::protobuf::Message for LedgerHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if self.version != 0 {
            os.write_uint32(2, self.version)?;
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

    fn new() -> LedgerHeader {
        LedgerHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &LedgerHeader| { &m.id },
                |m: &mut LedgerHeader| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "version",
                |m: &LedgerHeader| { &m.version },
                |m: &mut LedgerHeader| { &mut m.version },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LedgerHeader>(
                "LedgerHeader",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LedgerHeader {
        static instance: ::protobuf::rt::LazyV2<LedgerHeader> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LedgerHeader::new)
    }
}

impl ::protobuf::Clear for LedgerHeader {
    fn clear(&mut self) {
        self.id.clear();
        self.version = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LedgerHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LedgerHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum HeaderType {
    UNDEFINED = 0,
    LEDGER = 1,
    CONTRACT = 2,
    BLOCK = 3,
    TRANSACTION = 4,
}

impl ::protobuf::ProtobufEnum for HeaderType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HeaderType> {
        match value {
            0 => ::std::option::Option::Some(HeaderType::UNDEFINED),
            1 => ::std::option::Option::Some(HeaderType::LEDGER),
            2 => ::std::option::Option::Some(HeaderType::CONTRACT),
            3 => ::std::option::Option::Some(HeaderType::BLOCK),
            4 => ::std::option::Option::Some(HeaderType::TRANSACTION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HeaderType] = &[
            HeaderType::UNDEFINED,
            HeaderType::LEDGER,
            HeaderType::CONTRACT,
            HeaderType::BLOCK,
            HeaderType::TRANSACTION,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<HeaderType>("HeaderType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for HeaderType {
}

impl ::std::default::Default for HeaderType {
    fn default() -> Self {
        HeaderType::UNDEFINED
    }
}

impl ::protobuf::reflect::ProtobufValue for HeaderType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bsrc/protos/chain/data.proto\x12\x05chain\"A\n\x04Data\x12%\n\x06he\
    ader\x18\x01\x20\x01(\x0b2\r.chain.HeaderR\x06header\x12\x12\n\x04info\
    \x18\x02\x20\x01(\x0cR\x04info\"e\n\x06Header\x12%\n\x04type\x18\x01\x20\
    \x01(\x0e2\x11.chain.HeaderTypeR\x04type\x12\x1c\n\tsignature\x18\x02\
    \x20\x01(\x0cR\tsignature\x12\x16\n\x06ledger\x18\x03\x20\x01(\x0cR\x06l\
    edger\"?\n\x0fSignatureHeader\x12\x18\n\x07creator\x18\x01\x20\x01(\x0cR\
    \x07creator\x12\x12\n\x04sign\x18\x02\x20\x01(\x0cR\x04sign\"8\n\x0cLedg\
    erHeader\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x18\n\x07version\
    \x18\x02\x20\x01(\rR\x07version*Q\n\nHeaderType\x12\r\n\tUNDEFINED\x10\0\
    \x12\n\n\x06LEDGER\x10\x01\x12\x0c\n\x08CONTRACT\x10\x02\x12\t\n\x05BLOC\
    K\x10\x03\x12\x0f\n\x0bTRANSACTION\x10\x04BP\n\x20cn.aberic.george.proto\
    cols.chainB\tDataProtoZ!github.com/george/protocols/chainb\x06proto3\
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