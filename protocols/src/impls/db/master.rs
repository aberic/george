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
//! Generated file from `db/master.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Master {
    // message fields
    pub default_page_name: ::std::string::String,
    pub pages: ::protobuf::RepeatedField<super::page::Page>,
    pub databases: ::protobuf::RepeatedField<super::database::Database>,
    pub create_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Master {
    fn default() -> &'a Master {
        <Master as ::protobuf::Message>::default_instance()
    }
}

impl Master {
    pub fn new() -> Master {
        ::std::default::Default::default()
    }

    // string default_page_name = 1;


    pub fn get_default_page_name(&self) -> &str {
        &self.default_page_name
    }
    pub fn clear_default_page_name(&mut self) {
        self.default_page_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_default_page_name(&mut self, v: ::std::string::String) {
        self.default_page_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_page_name(&mut self) -> &mut ::std::string::String {
        &mut self.default_page_name
    }

    // Take field
    pub fn take_default_page_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.default_page_name, ::std::string::String::new())
    }

    // repeated .db.Page pages = 2;


    pub fn get_pages(&self) -> &[super::page::Page] {
        &self.pages
    }
    pub fn clear_pages(&mut self) {
        self.pages.clear();
    }

    // Param is passed by value, moved
    pub fn set_pages(&mut self, v: ::protobuf::RepeatedField<super::page::Page>) {
        self.pages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pages(&mut self) -> &mut ::protobuf::RepeatedField<super::page::Page> {
        &mut self.pages
    }

    // Take field
    pub fn take_pages(&mut self) -> ::protobuf::RepeatedField<super::page::Page> {
        ::std::mem::replace(&mut self.pages, ::protobuf::RepeatedField::new())
    }

    // repeated .db.Database databases = 3;


    pub fn get_databases(&self) -> &[super::database::Database] {
        &self.databases
    }
    pub fn clear_databases(&mut self) {
        self.databases.clear();
    }

    // Param is passed by value, moved
    pub fn set_databases(&mut self, v: ::protobuf::RepeatedField<super::database::Database>) {
        self.databases = v;
    }

    // Mutable pointer to the field.
    pub fn mut_databases(&mut self) -> &mut ::protobuf::RepeatedField<super::database::Database> {
        &mut self.databases
    }

    // Take field
    pub fn take_databases(&mut self) -> ::protobuf::RepeatedField<super::database::Database> {
        ::std::mem::replace(&mut self.databases, ::protobuf::RepeatedField::new())
    }

    // .google.protobuf.Timestamp create_time = 4;


    pub fn get_create_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.create_time.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_create_time(&mut self) {
        self.create_time.clear();
    }

    pub fn has_create_time(&self) -> bool {
        self.create_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.create_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.create_time.is_none() {
            self.create_time.set_default();
        }
        self.create_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_create_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.create_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }
}

impl ::protobuf::Message for Master {
    fn is_initialized(&self) -> bool {
        for v in &self.pages {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.databases {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.create_time {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.default_page_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pages)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.databases)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.create_time)?;
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
        if !self.default_page_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.default_page_name);
        }
        for value in &self.pages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.databases {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.create_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.default_page_name.is_empty() {
            os.write_string(1, &self.default_page_name)?;
        }
        for v in &self.pages {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.databases {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.create_time.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> Master {
        Master::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "default_page_name",
                |m: &Master| { &m.default_page_name },
                |m: &mut Master| { &mut m.default_page_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::page::Page>>(
                "pages",
                |m: &Master| { &m.pages },
                |m: &mut Master| { &mut m.pages },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::database::Database>>(
                "databases",
                |m: &Master| { &m.databases },
                |m: &mut Master| { &mut m.databases },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "create_time",
                |m: &Master| { &m.create_time },
                |m: &mut Master| { &mut m.create_time },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Master>(
                "Master",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Master {
        static instance: ::protobuf::rt::LazyV2<Master> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Master::new)
    }
}

impl ::protobuf::Clear for Master {
    fn clear(&mut self) {
        self.default_page_name.clear();
        self.pages.clear();
        self.databases.clear();
        self.create_time.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Master {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Master {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fdb/master.proto\x12\x02db\x1a\x1fgoogle/protobuf/timestamp.proto\
    \x1a\x11db/database.proto\x1a\rdb/page.proto\"\xbd\x01\n\x06Master\x12*\
    \n\x11default_page_name\x18\x01\x20\x01(\tR\x0fdefaultPageName\x12\x1e\n\
    \x05pages\x18\x02\x20\x03(\x0b2\x08.db.PageR\x05pages\x12*\n\tdatabases\
    \x18\x03\x20\x03(\x0b2\x0c.db.DatabaseR\tdatabases\x12;\n\x0bcreate_time\
    \x18\x04\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\ncreateTimeBL\n\
    \x1dcn.aberic.george.protocols.dbB\x0bMasterProtoZ\x1egithub.com/george/\
    protocols/dbb\x06proto3\
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