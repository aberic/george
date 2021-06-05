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
//! Generated file from `db/index.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Index {
    // message fields
    pub name: ::std::string::String,
    pub engine: Engine,
    pub primary: bool,
    pub unique: bool,
    pub null: bool,
    pub key_type: KeyType,
    pub create_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Index {
    fn default() -> &'a Index {
        <Index as ::protobuf::Message>::default_instance()
    }
}

impl Index {
    pub fn new() -> Index {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // .db.Engine engine = 2;


    pub fn get_engine(&self) -> Engine {
        self.engine
    }
    pub fn clear_engine(&mut self) {
        self.engine = Engine::None;
    }

    // Param is passed by value, moved
    pub fn set_engine(&mut self, v: Engine) {
        self.engine = v;
    }

    // bool primary = 3;


    pub fn get_primary(&self) -> bool {
        self.primary
    }
    pub fn clear_primary(&mut self) {
        self.primary = false;
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: bool) {
        self.primary = v;
    }

    // bool unique = 4;


    pub fn get_unique(&self) -> bool {
        self.unique
    }
    pub fn clear_unique(&mut self) {
        self.unique = false;
    }

    // Param is passed by value, moved
    pub fn set_unique(&mut self, v: bool) {
        self.unique = v;
    }

    // bool null = 5;


    pub fn get_null(&self) -> bool {
        self.null
    }
    pub fn clear_null(&mut self) {
        self.null = false;
    }

    // Param is passed by value, moved
    pub fn set_null(&mut self, v: bool) {
        self.null = v;
    }

    // .db.KeyType key_type = 6;


    pub fn get_key_type(&self) -> KeyType {
        self.key_type
    }
    pub fn clear_key_type(&mut self) {
        self.key_type = KeyType::String;
    }

    // Param is passed by value, moved
    pub fn set_key_type(&mut self, v: KeyType) {
        self.key_type = v;
    }

    // .google.protobuf.Timestamp create_time = 7;


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

impl ::protobuf::Message for Index {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.engine, 2, &mut self.unknown_fields)?
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.primary = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.unique = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.null = tmp;
                },
                6 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.key_type, 6, &mut self.unknown_fields)?
                },
                7 => {
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.engine != Engine::None {
            my_size += ::protobuf::rt::enum_size(2, self.engine);
        }
        if self.primary != false {
            my_size += 2;
        }
        if self.unique != false {
            my_size += 2;
        }
        if self.null != false {
            my_size += 2;
        }
        if self.key_type != KeyType::String {
            my_size += ::protobuf::rt::enum_size(6, self.key_type);
        }
        if let Some(ref v) = self.create_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.engine != Engine::None {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.engine))?;
        }
        if self.primary != false {
            os.write_bool(3, self.primary)?;
        }
        if self.unique != false {
            os.write_bool(4, self.unique)?;
        }
        if self.null != false {
            os.write_bool(5, self.null)?;
        }
        if self.key_type != KeyType::String {
            os.write_enum(6, ::protobuf::ProtobufEnum::value(&self.key_type))?;
        }
        if let Some(ref v) = self.create_time.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> Index {
        Index::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Index| { &m.name },
                |m: &mut Index| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Engine>>(
                "engine",
                |m: &Index| { &m.engine },
                |m: &mut Index| { &mut m.engine },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "primary",
                |m: &Index| { &m.primary },
                |m: &mut Index| { &mut m.primary },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "unique",
                |m: &Index| { &m.unique },
                |m: &mut Index| { &mut m.unique },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "null",
                |m: &Index| { &m.null },
                |m: &mut Index| { &mut m.null },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<KeyType>>(
                "key_type",
                |m: &Index| { &m.key_type },
                |m: &mut Index| { &mut m.key_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "create_time",
                |m: &Index| { &m.create_time },
                |m: &mut Index| { &mut m.create_time },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Index>(
                "Index",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Index {
        static instance: ::protobuf::rt::LazyV2<Index> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Index::new)
    }
}

impl ::protobuf::Clear for Index {
    fn clear(&mut self) {
        self.name.clear();
        self.engine = Engine::None;
        self.primary = false;
        self.unique = false;
        self.null = false;
        self.key_type = KeyType::String;
        self.create_time.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Index {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Index {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct IndexList {
    // message fields
    pub indexes: ::protobuf::RepeatedField<Index>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a IndexList {
    fn default() -> &'a IndexList {
        <IndexList as ::protobuf::Message>::default_instance()
    }
}

impl IndexList {
    pub fn new() -> IndexList {
        ::std::default::Default::default()
    }

    // repeated .db.Index indexes = 1;


    pub fn get_indexes(&self) -> &[Index] {
        &self.indexes
    }
    pub fn clear_indexes(&mut self) {
        self.indexes.clear();
    }

    // Param is passed by value, moved
    pub fn set_indexes(&mut self, v: ::protobuf::RepeatedField<Index>) {
        self.indexes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_indexes(&mut self) -> &mut ::protobuf::RepeatedField<Index> {
        &mut self.indexes
    }

    // Take field
    pub fn take_indexes(&mut self) -> ::protobuf::RepeatedField<Index> {
        ::std::mem::replace(&mut self.indexes, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for IndexList {
    fn is_initialized(&self) -> bool {
        for v in &self.indexes {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.indexes)?;
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
        for value in &self.indexes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.indexes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> IndexList {
        IndexList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Index>>(
                "indexes",
                |m: &IndexList| { &m.indexes },
                |m: &mut IndexList| { &mut m.indexes },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<IndexList>(
                "IndexList",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static IndexList {
        static instance: ::protobuf::rt::LazyV2<IndexList> = ::protobuf::rt::LazyV2::INIT;
        instance.get(IndexList::new)
    }
}

impl ::protobuf::Clear for IndexList {
    fn clear(&mut self) {
        self.indexes.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IndexList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IndexList {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct RequestIndexList {
    // message fields
    pub database_name: ::std::string::String,
    pub view_name: ::std::string::String,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RequestIndexList {
    fn default() -> &'a RequestIndexList {
        <RequestIndexList as ::protobuf::Message>::default_instance()
    }
}

impl RequestIndexList {
    pub fn new() -> RequestIndexList {
        ::std::default::Default::default()
    }

    // string database_name = 1;


    pub fn get_database_name(&self) -> &str {
        &self.database_name
    }
    pub fn clear_database_name(&mut self) {
        self.database_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_database_name(&mut self, v: ::std::string::String) {
        self.database_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_database_name(&mut self) -> &mut ::std::string::String {
        &mut self.database_name
    }

    // Take field
    pub fn take_database_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.database_name, ::std::string::String::new())
    }

    // string view_name = 2;


    pub fn get_view_name(&self) -> &str {
        &self.view_name
    }
    pub fn clear_view_name(&mut self) {
        self.view_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_view_name(&mut self, v: ::std::string::String) {
        self.view_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_view_name(&mut self) -> &mut ::std::string::String {
        &mut self.view_name
    }

    // Take field
    pub fn take_view_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.view_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for RequestIndexList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.database_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.view_name)?;
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
        if !self.database_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.database_name);
        }
        if !self.view_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.view_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.database_name.is_empty() {
            os.write_string(1, &self.database_name)?;
        }
        if !self.view_name.is_empty() {
            os.write_string(2, &self.view_name)?;
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

    fn new() -> RequestIndexList {
        RequestIndexList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "database_name",
                |m: &RequestIndexList| { &m.database_name },
                |m: &mut RequestIndexList| { &mut m.database_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "view_name",
                |m: &RequestIndexList| { &m.view_name },
                |m: &mut RequestIndexList| { &mut m.view_name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RequestIndexList>(
                "RequestIndexList",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RequestIndexList {
        static instance: ::protobuf::rt::LazyV2<RequestIndexList> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RequestIndexList::new)
    }
}

impl ::protobuf::Clear for RequestIndexList {
    fn clear(&mut self) {
        self.database_name.clear();
        self.view_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestIndexList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestIndexList {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum Engine {
    None = 0,
    Increment = 1,
    Sequence = 2,
    Disk = 3,
    Block = 4,
}

impl ::protobuf::ProtobufEnum for Engine {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Engine> {
        match value {
            0 => ::std::option::Option::Some(Engine::None),
            1 => ::std::option::Option::Some(Engine::Increment),
            2 => ::std::option::Option::Some(Engine::Sequence),
            3 => ::std::option::Option::Some(Engine::Disk),
            4 => ::std::option::Option::Some(Engine::Block),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Engine] = &[
            Engine::None,
            Engine::Increment,
            Engine::Sequence,
            Engine::Disk,
            Engine::Block,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Engine>("Engine", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Engine {
}

impl ::std::default::Default for Engine {
    fn default() -> Self {
        Engine::None
    }
}

impl ::protobuf::reflect::ProtobufValue for Engine {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum KeyType {
    String = 0,
    UInt = 1,
    Int = 2,
    Float = 3,
    Bool = 4,
    Nonsupport = 5,
}

impl ::protobuf::ProtobufEnum for KeyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KeyType> {
        match value {
            0 => ::std::option::Option::Some(KeyType::String),
            1 => ::std::option::Option::Some(KeyType::UInt),
            2 => ::std::option::Option::Some(KeyType::Int),
            3 => ::std::option::Option::Some(KeyType::Float),
            4 => ::std::option::Option::Some(KeyType::Bool),
            5 => ::std::option::Option::Some(KeyType::Nonsupport),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [KeyType] = &[
            KeyType::String,
            KeyType::UInt,
            KeyType::Int,
            KeyType::Float,
            KeyType::Bool,
            KeyType::Nonsupport,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<KeyType>("KeyType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for KeyType {
}

impl ::std::default::Default for KeyType {
    fn default() -> Self {
        KeyType::String
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0edb/index.proto\x12\x02db\x1a\x1fgoogle/protobuf/timestamp.proto\"\
    \xea\x01\n\x05Index\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\"\
    \n\x06engine\x18\x02\x20\x01(\x0e2\n.db.EngineR\x06engine\x12\x18\n\x07p\
    rimary\x18\x03\x20\x01(\x08R\x07primary\x12\x16\n\x06unique\x18\x04\x20\
    \x01(\x08R\x06unique\x12\x12\n\x04null\x18\x05\x20\x01(\x08R\x04null\x12\
    &\n\x08key_type\x18\x06\x20\x01(\x0e2\x0b.db.KeyTypeR\x07keyType\x12;\n\
    \x0bcreate_time\x18\x07\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\ncr\
    eateTime\"0\n\tIndexList\x12#\n\x07indexes\x18\x01\x20\x03(\x0b2\t.db.In\
    dexR\x07indexes\"T\n\x10RequestIndexList\x12#\n\rdatabase_name\x18\x01\
    \x20\x01(\tR\x0cdatabaseName\x12\x1b\n\tview_name\x18\x02\x20\x01(\tR\
    \x08viewName*D\n\x06Engine\x12\x08\n\x04None\x10\0\x12\r\n\tIncrement\
    \x10\x01\x12\x0c\n\x08Sequence\x10\x02\x12\x08\n\x04Disk\x10\x03\x12\t\n\
    \x05Block\x10\x04*M\n\x07KeyType\x12\n\n\x06String\x10\0\x12\x08\n\x04UI\
    nt\x10\x01\x12\x07\n\x03Int\x10\x02\x12\t\n\x05Float\x10\x03\x12\x08\n\
    \x04Bool\x10\x04\x12\x0e\n\nNonsupport\x10\x05BK\n\x1dcn.aberic.george.p\
    rotocols.dbB\nIndexProtoZ\x1egithub.com/george/protocols/dbb\x06proto3\
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
