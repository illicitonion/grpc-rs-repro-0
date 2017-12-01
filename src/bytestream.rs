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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq, Clone, Default)]
pub struct ReadRequest {
    // message fields
    pub resource_name: ::std::string::String,
    pub read_offset: i64,
    pub read_limit: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadRequest {}

impl ReadRequest {
    pub fn new() -> ReadRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadRequest {
        static mut instance: ::protobuf::lazy::Lazy<ReadRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadRequest,
        };
        unsafe { instance.get(ReadRequest::new) }
    }

    // string resource_name = 1;

    pub fn clear_resource_name(&mut self) {
        self.resource_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_name(&mut self, v: ::std::string::String) {
        self.resource_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource_name(&mut self) -> &mut ::std::string::String {
        &mut self.resource_name
    }

    // Take field
    pub fn take_resource_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource_name, ::std::string::String::new())
    }

    pub fn get_resource_name(&self) -> &str {
        &self.resource_name
    }

    fn get_resource_name_for_reflect(&self) -> &::std::string::String {
        &self.resource_name
    }

    fn mut_resource_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.resource_name
    }

    // int64 read_offset = 2;

    pub fn clear_read_offset(&mut self) {
        self.read_offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_read_offset(&mut self, v: i64) {
        self.read_offset = v;
    }

    pub fn get_read_offset(&self) -> i64 {
        self.read_offset
    }

    fn get_read_offset_for_reflect(&self) -> &i64 {
        &self.read_offset
    }

    fn mut_read_offset_for_reflect(&mut self) -> &mut i64 {
        &mut self.read_offset
    }

    // int64 read_limit = 3;

    pub fn clear_read_limit(&mut self) {
        self.read_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_read_limit(&mut self, v: i64) {
        self.read_limit = v;
    }

    pub fn get_read_limit(&self) -> i64 {
        self.read_limit
    }

    fn get_read_limit_for_reflect(&self) -> &i64 {
        &self.read_limit
    }

    fn mut_read_limit_for_reflect(&mut self) -> &mut i64 {
        &mut self.read_limit
    }
}

impl ::protobuf::Message for ReadRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.resource_name,
                    )?;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    let tmp = is.read_int64()?;
                    self.read_offset = tmp;
                }
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    let tmp = is.read_int64()?;
                    self.read_limit = tmp;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.resource_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource_name);
        }
        if self.read_offset != 0 {
            my_size += ::protobuf::rt::value_size(
                2,
                self.read_offset,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.read_limit != 0 {
            my_size += ::protobuf::rt::value_size(
                3,
                self.read_limit,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if !self.resource_name.is_empty() {
            os.write_string(1, &self.resource_name)?;
        }
        if self.read_offset != 0 {
            os.write_int64(2, self.read_offset)?;
        }
        if self.read_limit != 0 {
            os.write_int64(3, self.read_limit)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadRequest {
    fn new() -> ReadRequest {
        ReadRequest::new()
    }

    fn descriptor_static(
        _: ::std::option::Option<ReadRequest>,
    ) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resource_name",
                    ReadRequest::get_resource_name_for_reflect,
                    ReadRequest::mut_resource_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "read_offset",
                    ReadRequest::get_read_offset_for_reflect,
                    ReadRequest::mut_read_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "read_limit",
                    ReadRequest::get_read_limit_for_reflect,
                    ReadRequest::mut_read_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadRequest>(
                    "ReadRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadRequest {
    fn clear(&mut self) {
        self.clear_resource_name();
        self.clear_read_offset();
        self.clear_read_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct ReadResponse {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadResponse {}

impl ReadResponse {
    pub fn new() -> ReadResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadResponse {
        static mut instance: ::protobuf::lazy::Lazy<ReadResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadResponse,
        };
        unsafe { instance.get(ReadResponse::new) }
    }

    // bytes data = 10;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for ReadResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(10, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(10, &self.data)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadResponse {
    fn new() -> ReadResponse {
        ReadResponse::new()
    }

    fn descriptor_static(
        _: ::std::option::Option<ReadResponse>,
    ) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ReadResponse::get_data_for_reflect,
                    ReadResponse::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadResponse>(
                    "ReadResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadResponse {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct WriteRequest {
    // message fields
    pub resource_name: ::std::string::String,
    pub write_offset: i64,
    pub finish_write: bool,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteRequest {}

impl WriteRequest {
    pub fn new() -> WriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<WriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteRequest,
        };
        unsafe { instance.get(WriteRequest::new) }
    }

    // string resource_name = 1;

    pub fn clear_resource_name(&mut self) {
        self.resource_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_name(&mut self, v: ::std::string::String) {
        self.resource_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource_name(&mut self) -> &mut ::std::string::String {
        &mut self.resource_name
    }

    // Take field
    pub fn take_resource_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource_name, ::std::string::String::new())
    }

    pub fn get_resource_name(&self) -> &str {
        &self.resource_name
    }

    fn get_resource_name_for_reflect(&self) -> &::std::string::String {
        &self.resource_name
    }

    fn mut_resource_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.resource_name
    }

    // int64 write_offset = 2;

    pub fn clear_write_offset(&mut self) {
        self.write_offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_write_offset(&mut self, v: i64) {
        self.write_offset = v;
    }

    pub fn get_write_offset(&self) -> i64 {
        self.write_offset
    }

    fn get_write_offset_for_reflect(&self) -> &i64 {
        &self.write_offset
    }

    fn mut_write_offset_for_reflect(&mut self) -> &mut i64 {
        &mut self.write_offset
    }

    // bool finish_write = 3;

    pub fn clear_finish_write(&mut self) {
        self.finish_write = false;
    }

    // Param is passed by value, moved
    pub fn set_finish_write(&mut self, v: bool) {
        self.finish_write = v;
    }

    pub fn get_finish_write(&self) -> bool {
        self.finish_write
    }

    fn get_finish_write_for_reflect(&self) -> &bool {
        &self.finish_write
    }

    fn mut_finish_write_for_reflect(&mut self) -> &mut bool {
        &mut self.finish_write
    }

    // bytes data = 10;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for WriteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.resource_name,
                    )?;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    let tmp = is.read_int64()?;
                    self.write_offset = tmp;
                }
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    let tmp = is.read_bool()?;
                    self.finish_write = tmp;
                }
                10 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.resource_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource_name);
        }
        if self.write_offset != 0 {
            my_size += ::protobuf::rt::value_size(
                2,
                self.write_offset,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.finish_write != false {
            my_size += 2;
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(10, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if !self.resource_name.is_empty() {
            os.write_string(1, &self.resource_name)?;
        }
        if self.write_offset != 0 {
            os.write_int64(2, self.write_offset)?;
        }
        if self.finish_write != false {
            os.write_bool(3, self.finish_write)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(10, &self.data)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteRequest {
    fn new() -> WriteRequest {
        WriteRequest::new()
    }

    fn descriptor_static(
        _: ::std::option::Option<WriteRequest>,
    ) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resource_name",
                    WriteRequest::get_resource_name_for_reflect,
                    WriteRequest::mut_resource_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "write_offset",
                    WriteRequest::get_write_offset_for_reflect,
                    WriteRequest::mut_write_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "finish_write",
                    WriteRequest::get_finish_write_for_reflect,
                    WriteRequest::mut_finish_write_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    WriteRequest::get_data_for_reflect,
                    WriteRequest::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteRequest>(
                    "WriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteRequest {
    fn clear(&mut self) {
        self.clear_resource_name();
        self.clear_write_offset();
        self.clear_finish_write();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct WriteResponse {
    // message fields
    pub committed_size: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteResponse {}

impl WriteResponse {
    pub fn new() -> WriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<WriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteResponse,
        };
        unsafe { instance.get(WriteResponse::new) }
    }

    // int64 committed_size = 1;

    pub fn clear_committed_size(&mut self) {
        self.committed_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_committed_size(&mut self, v: i64) {
        self.committed_size = v;
    }

    pub fn get_committed_size(&self) -> i64 {
        self.committed_size
    }

    fn get_committed_size_for_reflect(&self) -> &i64 {
        &self.committed_size
    }

    fn mut_committed_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.committed_size
    }
}

impl ::protobuf::Message for WriteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    let tmp = is.read_int64()?;
                    self.committed_size = tmp;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.committed_size != 0 {
            my_size += ::protobuf::rt::value_size(
                1,
                self.committed_size,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if self.committed_size != 0 {
            os.write_int64(1, self.committed_size)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteResponse {
    fn new() -> WriteResponse {
        WriteResponse::new()
    }

    fn descriptor_static(
        _: ::std::option::Option<WriteResponse>,
    ) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "committed_size",
                    WriteResponse::get_committed_size_for_reflect,
                    WriteResponse::mut_committed_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteResponse>(
                    "WriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteResponse {
    fn clear(&mut self) {
        self.clear_committed_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct QueryWriteStatusRequest {
    // message fields
    pub resource_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryWriteStatusRequest {}

impl QueryWriteStatusRequest {
    pub fn new() -> QueryWriteStatusRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryWriteStatusRequest {
        static mut instance: ::protobuf::lazy::Lazy<QueryWriteStatusRequest> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const QueryWriteStatusRequest,
            };
        unsafe { instance.get(QueryWriteStatusRequest::new) }
    }

    // string resource_name = 1;

    pub fn clear_resource_name(&mut self) {
        self.resource_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_name(&mut self, v: ::std::string::String) {
        self.resource_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource_name(&mut self) -> &mut ::std::string::String {
        &mut self.resource_name
    }

    // Take field
    pub fn take_resource_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource_name, ::std::string::String::new())
    }

    pub fn get_resource_name(&self) -> &str {
        &self.resource_name
    }

    fn get_resource_name_for_reflect(&self) -> &::std::string::String {
        &self.resource_name
    }

    fn mut_resource_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.resource_name
    }
}

impl ::protobuf::Message for QueryWriteStatusRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.resource_name,
                    )?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.resource_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if !self.resource_name.is_empty() {
            os.write_string(1, &self.resource_name)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryWriteStatusRequest {
    fn new() -> QueryWriteStatusRequest {
        QueryWriteStatusRequest::new()
    }

    fn descriptor_static(
        _: ::std::option::Option<QueryWriteStatusRequest>,
    ) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resource_name",
                    QueryWriteStatusRequest::get_resource_name_for_reflect,
                    QueryWriteStatusRequest::mut_resource_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryWriteStatusRequest>(
                    "QueryWriteStatusRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryWriteStatusRequest {
    fn clear(&mut self) {
        self.clear_resource_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryWriteStatusRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryWriteStatusRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct QueryWriteStatusResponse {
    // message fields
    pub committed_size: i64,
    pub complete: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryWriteStatusResponse {}

impl QueryWriteStatusResponse {
    pub fn new() -> QueryWriteStatusResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryWriteStatusResponse {
        static mut instance: ::protobuf::lazy::Lazy<QueryWriteStatusResponse> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const QueryWriteStatusResponse,
            };
        unsafe { instance.get(QueryWriteStatusResponse::new) }
    }

    // int64 committed_size = 1;

    pub fn clear_committed_size(&mut self) {
        self.committed_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_committed_size(&mut self, v: i64) {
        self.committed_size = v;
    }

    pub fn get_committed_size(&self) -> i64 {
        self.committed_size
    }

    fn get_committed_size_for_reflect(&self) -> &i64 {
        &self.committed_size
    }

    fn mut_committed_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.committed_size
    }

    // bool complete = 2;

    pub fn clear_complete(&mut self) {
        self.complete = false;
    }

    // Param is passed by value, moved
    pub fn set_complete(&mut self, v: bool) {
        self.complete = v;
    }

    pub fn get_complete(&self) -> bool {
        self.complete
    }

    fn get_complete_for_reflect(&self) -> &bool {
        &self.complete
    }

    fn mut_complete_for_reflect(&mut self) -> &mut bool {
        &mut self.complete
    }
}

impl ::protobuf::Message for QueryWriteStatusResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    let tmp = is.read_int64()?;
                    self.committed_size = tmp;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    let tmp = is.read_bool()?;
                    self.complete = tmp;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.committed_size != 0 {
            my_size += ::protobuf::rt::value_size(
                1,
                self.committed_size,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if self.complete != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if self.committed_size != 0 {
            os.write_int64(1, self.committed_size)?;
        }
        if self.complete != false {
            os.write_bool(2, self.complete)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryWriteStatusResponse {
    fn new() -> QueryWriteStatusResponse {
        QueryWriteStatusResponse::new()
    }

    fn descriptor_static(
        _: ::std::option::Option<QueryWriteStatusResponse>,
    ) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "committed_size",
                    QueryWriteStatusResponse::get_committed_size_for_reflect,
                    QueryWriteStatusResponse::mut_committed_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "complete",
                    QueryWriteStatusResponse::get_complete_for_reflect,
                    QueryWriteStatusResponse::mut_complete_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryWriteStatusResponse>(
                    "QueryWriteStatusResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryWriteStatusResponse {
    fn clear(&mut self) {
        self.clear_committed_size();
        self.clear_complete();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryWriteStatusResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryWriteStatusResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"google/bytestream/bytestream.proto\x12\x11google.bytestream\x1a\x1cg\
    oogle/api/annotations.proto\x1a\x1egoogle/protobuf/wrappers.proto\"r\n\
    \x0bReadRequest\x12#\n\rresource_name\x18\x01\x20\x01(\tR\x0cresourceNam\
    e\x12\x1f\n\x0bread_offset\x18\x02\x20\x01(\x03R\nreadOffset\x12\x1d\n\n\
    read_limit\x18\x03\x20\x01(\x03R\treadLimit\"\"\n\x0cReadResponse\x12\
    \x12\n\x04data\x18\n\x20\x01(\x0cR\x04data\"\x8d\x01\n\x0cWriteRequest\
    \x12#\n\rresource_name\x18\x01\x20\x01(\tR\x0cresourceName\x12!\n\x0cwri\
    te_offset\x18\x02\x20\x01(\x03R\x0bwriteOffset\x12!\n\x0cfinish_write\
    \x18\x03\x20\x01(\x08R\x0bfinishWrite\x12\x12\n\x04data\x18\n\x20\x01(\
    \x0cR\x04data\"6\n\rWriteResponse\x12%\n\x0ecommitted_size\x18\x01\x20\
    \x01(\x03R\rcommittedSize\">\n\x17QueryWriteStatusRequest\x12#\n\rresour\
    ce_name\x18\x01\x20\x01(\tR\x0cresourceName\"]\n\x18QueryWriteStatusResp\
    onse\x12%\n\x0ecommitted_size\x18\x01\x20\x01(\x03R\rcommittedSize\x12\
    \x1a\n\x08complete\x18\x02\x20\x01(\x08R\x08complete2\x92\x02\n\nByteStr\
    eam\x12I\n\x04Read\x12\x1e.google.bytestream.ReadRequest\x1a\x1f.google.\
    bytestream.ReadResponse0\x01\x12L\n\x05Write\x12\x1f.google.bytestream.W\
    riteRequest\x1a\x20.google.bytestream.WriteResponse(\x01\x12k\n\x10Query\
    WriteStatus\x12*.google.bytestream.QueryWriteStatusRequest\x1a+.google.b\
    ytestream.QueryWriteStatusResponseBe\n\x15com.google.bytestreamB\x0fByte\
    StreamProtoZ;google.golang.org/genproto/googleapis/bytestream;bytestream\
    J\xf6<\n\x07\x12\x05\x0e\0\xb4\x01\x01\n\xbd\x04\n\x01\x0c\x12\x03\x0e\0\
    \x122\xb2\x04\x20Copyright\x202016\x20Google\x20Inc.\n\n\x20Licensed\x20\
    under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"Licens\
    e\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20c\
    ompliance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20c\
    opy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apac\
    he.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicabl\
    e\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distri\
    buted\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"A\
    S\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\
    \x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\
    \x20License\x20for\x20the\x20specific\x20language\x20governing\x20permis\
    sions\x20and\n\x20limitations\x20under\x20the\x20License.\n\n\x08\n\x01\
    \x02\x12\x03\x10\x08\x19\n\t\n\x02\x03\0\x12\x03\x12\x07%\n\t\n\x02\x03\
    \x01\x12\x03\x13\x07'\n\x08\n\x01\x08\x12\x03\x15\0R\n\x0b\n\x04\x08\xe7\
    \x07\0\x12\x03\x15\0R\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x15\x07\x11\
    \n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x15\x07\x11\n\x0e\n\x07\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\x15\x07\x11\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\
    \x03\x15\x14Q\n\x08\n\x01\x08\x12\x03\x16\00\n\x0b\n\x04\x08\xe7\x07\x01\
    \x12\x03\x16\00\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x16\x07\x1b\n\r\
    \n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x16\x07\x1b\n\x0e\n\x07\x08\xe7\x07\
    \x01\x02\0\x01\x12\x03\x16\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\
    \x03\x16\x1e/\n\x08\n\x01\x08\x12\x03\x17\0.\n\x0b\n\x04\x08\xe7\x07\x02\
    \x12\x03\x17\0.\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x17\x07\x13\n\r\
    \n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x17\x07\x13\n\x0e\n\x07\x08\xe7\x07\
    \x02\x02\0\x01\x12\x03\x17\x07\x13\n\x0c\n\x05\x08\xe7\x07\x02\x07\x12\
    \x03\x17\x16-\n\xbf\x06\n\x02\x06\0\x12\x041\0^\x01\x1a\xb2\x06\x20####\
    \x20Introduction\n\n\x20The\x20Byte\x20Stream\x20API\x20enables\x20a\x20\
    client\x20to\x20read\x20and\x20write\x20a\x20stream\x20of\x20bytes\x20to\
    \n\x20and\x20from\x20a\x20resource.\x20Resources\x20have\x20names,\x20an\
    d\x20these\x20names\x20are\x20supplied\x20in\n\x20the\x20API\x20calls\
    \x20below\x20to\x20identify\x20the\x20resource\x20that\x20is\x20being\
    \x20read\x20from\x20or\n\x20written\x20to.\n\n\x20All\x20implementations\
    \x20of\x20the\x20Byte\x20Stream\x20API\x20export\x20the\x20interface\x20\
    defined\x20here:\n\n\x20*\x20`Read()`:\x20Reads\x20the\x20contents\x20of\
    \x20a\x20resource.\n\n\x20*\x20`Write()`:\x20Writes\x20the\x20contents\
    \x20of\x20a\x20resource.\x20The\x20client\x20can\x20call\x20`Write()`\n\
    \x20\x20\x20multiple\x20times\x20with\x20the\x20same\x20resource\x20and\
    \x20can\x20check\x20the\x20status\x20of\x20the\x20write\n\x20\x20\x20by\
    \x20calling\x20`QueryWriteStatus()`.\n\n\x20####\x20Service\x20parameter\
    s\x20and\x20metadata\n\n\x20The\x20ByteStream\x20API\x20provides\x20no\
    \x20direct\x20way\x20to\x20access/modify\x20any\x20metadata\n\x20associa\
    ted\x20with\x20the\x20resource.\n\n\x20####\x20Errors\n\n\x20The\x20erro\
    rs\x20returned\x20by\x20the\x20service\x20are\x20in\x20the\x20Google\x20\
    canonical\x20error\x20space.\n\n\n\n\x03\x06\0\x01\x12\x031\x08\x12\n\
    \xe3\x01\n\x04\x06\0\x02\0\x12\x035\x026\x1a\xd5\x01\x20`Read()`\x20is\
    \x20used\x20to\x20retrieve\x20the\x20contents\x20of\x20a\x20resource\x20\
    as\x20a\x20sequence\n\x20of\x20bytes.\x20The\x20bytes\x20are\x20returned\
    \x20in\x20a\x20sequence\x20of\x20responses,\x20and\x20the\n\x20responses\
    \x20are\x20delivered\x20as\x20the\x20results\x20of\x20a\x20server-side\
    \x20streaming\x20RPC.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x035\x06\n\n\x0c\
    \n\x05\x06\0\x02\0\x02\x12\x035\x0b\x16\n\x0c\n\x05\x06\0\x02\0\x06\x12\
    \x035!'\n\x0c\n\x05\x06\0\x02\0\x03\x12\x035(4\n\xcb\t\n\x04\x06\0\x02\
    \x01\x12\x03M\x029\x1a\xbd\t\x20`Write()`\x20is\x20used\x20to\x20send\
    \x20the\x20contents\x20of\x20a\x20resource\x20as\x20a\x20sequence\x20of\
    \n\x20bytes.\x20The\x20bytes\x20are\x20sent\x20in\x20a\x20sequence\x20of\
    \x20request\x20protos\x20of\x20a\x20client-side\n\x20streaming\x20RPC.\n\
    \n\x20A\x20`Write()`\x20action\x20is\x20resumable.\x20If\x20there\x20is\
    \x20an\x20error\x20or\x20the\x20connection\x20is\n\x20broken\x20during\
    \x20the\x20`Write()`,\x20the\x20client\x20should\x20check\x20the\x20stat\
    us\x20of\x20the\n\x20`Write()`\x20by\x20calling\x20`QueryWriteStatus()`\
    \x20and\x20continue\x20writing\x20from\x20the\n\x20returned\x20`committe\
    d_size`.\x20This\x20may\x20be\x20less\x20than\x20the\x20amount\x20of\x20\
    data\x20the\n\x20client\x20previously\x20sent.\n\n\x20Calling\x20`Write(\
    )`\x20on\x20a\x20resource\x20name\x20that\x20was\x20previously\x20writte\
    n\x20and\n\x20finalized\x20could\x20cause\x20an\x20error,\x20depending\
    \x20on\x20whether\x20the\x20underlying\x20service\n\x20allows\x20over-wr\
    iting\x20of\x20previously\x20written\x20resources.\n\n\x20When\x20the\
    \x20client\x20closes\x20the\x20request\x20channel,\x20the\x20service\x20\
    will\x20respond\x20with\n\x20a\x20`WriteResponse`.\x20The\x20service\x20\
    will\x20not\x20view\x20the\x20resource\x20as\x20`complete`\n\x20until\
    \x20the\x20client\x20has\x20sent\x20a\x20`WriteRequest`\x20with\x20`fini\
    sh_write`\x20set\x20to\n\x20`true`.\x20Sending\x20any\x20requests\x20on\
    \x20a\x20stream\x20after\x20sending\x20a\x20request\x20with\n\x20`finish\
    _write`\x20set\x20to\x20`true`\x20will\x20cause\x20an\x20error.\x20The\
    \x20client\x20**should**\n\x20check\x20the\x20`WriteResponse`\x20it\x20r\
    eceives\x20to\x20determine\x20how\x20much\x20data\x20the\n\x20service\
    \x20was\x20able\x20to\x20commit\x20and\x20whether\x20the\x20service\x20v\
    iews\x20the\x20resource\x20as\n\x20`complete`\x20or\x20not.\n\n\x0c\n\
    \x05\x06\0\x02\x01\x01\x12\x03M\x06\x0b\n\x0c\n\x05\x06\0\x02\x01\x05\
    \x12\x03M\x0c\x12\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03M\x13\x1f\n\x0c\n\
    \x05\x06\0\x02\x01\x03\x12\x03M*7\n\xdf\x05\n\x04\x06\0\x02\x02\x12\x03]\
    \x02S\x1a\xd1\x05\x20`QueryWriteStatus()`\x20is\x20used\x20to\x20find\
    \x20the\x20`committed_size`\x20for\x20a\x20resource\n\x20that\x20is\x20b\
    eing\x20written,\x20which\x20can\x20then\x20be\x20used\x20as\x20the\x20`\
    write_offset`\x20for\n\x20the\x20next\x20`Write()`\x20call.\n\n\x20If\
    \x20the\x20resource\x20does\x20not\x20exist\x20(i.e.,\x20the\x20resource\
    \x20has\x20been\x20deleted,\x20or\x20the\n\x20first\x20`Write()`\x20has\
    \x20not\x20yet\x20reached\x20the\x20service),\x20this\x20method\x20retur\
    ns\x20the\n\x20error\x20`NOT_FOUND`.\n\n\x20The\x20client\x20**may**\x20\
    call\x20`QueryWriteStatus()`\x20at\x20any\x20time\x20to\x20determine\x20\
    how\n\x20much\x20data\x20has\x20been\x20processed\x20for\x20this\x20reso\
    urce.\x20This\x20is\x20useful\x20if\x20the\n\x20client\x20is\x20bufferin\
    g\x20data\x20and\x20needs\x20to\x20know\x20which\x20data\x20can\x20be\
    \x20safely\n\x20evicted.\x20For\x20any\x20sequence\x20of\x20`QueryWriteS\
    tatus()`\x20calls\x20for\x20a\x20given\n\x20resource\x20name,\x20the\x20\
    sequence\x20of\x20returned\x20`committed_size`\x20values\x20will\x20be\n\
    \x20non-decreasing.\n\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03]\x06\x16\n\
    \x0c\n\x05\x06\0\x02\x02\x02\x12\x03]\x17.\n\x0c\n\x05\x06\0\x02\x02\x03\
    \x12\x03]9Q\n1\n\x02\x04\0\x12\x04a\0t\x01\x1a%\x20Request\x20object\x20\
    for\x20ByteStream.Read.\n\n\n\n\x03\x04\0\x01\x12\x03a\x08\x13\n0\n\x04\
    \x04\0\x02\0\x12\x03c\x02\x1b\x1a#\x20The\x20name\x20of\x20the\x20resour\
    ce\x20to\x20read.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04c\x02a\x15\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03c\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03c\t\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03c\x19\x1a\n\xdd\x01\n\x04\
    \x04\0\x02\x01\x12\x03j\x02\x18\x1a\xcf\x01\x20The\x20offset\x20for\x20t\
    he\x20first\x20byte\x20to\x20return\x20in\x20the\x20read,\x20relative\
    \x20to\x20the\x20start\n\x20of\x20the\x20resource.\n\n\x20A\x20`read_off\
    set`\x20that\x20is\x20negative\x20or\x20greater\x20than\x20the\x20size\
    \x20of\x20the\x20resource\n\x20will\x20cause\x20an\x20`OUT_OF_RANGE`\x20\
    error.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04j\x02c\x1b\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03j\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03j\x08\
    \x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03j\x16\x17\n\x97\x03\n\x04\x04\
    \0\x02\x02\x12\x03s\x02\x17\x1a\x89\x03\x20The\x20maximum\x20number\x20o\
    f\x20`data`\x20bytes\x20the\x20server\x20is\x20allowed\x20to\x20return\
    \x20in\x20the\n\x20sum\x20of\x20all\x20`ReadResponse`\x20messages.\x20A\
    \x20`read_limit`\x20of\x20zero\x20indicates\x20that\n\x20there\x20is\x20\
    no\x20limit,\x20and\x20a\x20negative\x20`read_limit`\x20will\x20cause\
    \x20an\x20error.\n\n\x20If\x20the\x20stream\x20returns\x20fewer\x20bytes\
    \x20than\x20allowed\x20by\x20the\x20`read_limit`\x20and\x20no\n\x20error\
    \x20occurred,\x20the\x20stream\x20includes\x20all\x20data\x20from\x20the\
    \x20`read_offset`\x20to\x20the\n\x20end\x20of\x20the\x20resource.\n\n\r\
    \n\x05\x04\0\x02\x02\x04\x12\x04s\x02j\x18\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03s\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03s\x08\x12\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03s\x15\x16\n2\n\x02\x04\x01\x12\x04w\0}\x01\
    \x1a&\x20Response\x20object\x20for\x20ByteStream.Read.\n\n\n\n\x03\x04\
    \x01\x01\x12\x03w\x08\x14\n\x84\x02\n\x04\x04\x01\x02\0\x12\x03|\x02\x12\
    \x1a\xf6\x01\x20A\x20portion\x20of\x20the\x20data\x20for\x20the\x20resou\
    rce.\x20The\x20service\x20**may**\x20leave\x20`data`\n\x20empty\x20for\
    \x20any\x20given\x20`ReadResponse`.\x20This\x20enables\x20the\x20service\
    \x20to\x20inform\x20the\n\x20client\x20that\x20the\x20request\x20is\x20s\
    till\x20live\x20while\x20it\x20is\x20running\x20an\x20operation\x20to\n\
    \x20generate\x20more\x20data.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04|\x02\
    w\x16\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03|\x02\x07\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03|\x08\x0c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03|\x0f\
    \x11\n4\n\x02\x04\x02\x12\x06\x80\x01\0\x9e\x01\x01\x1a&\x20Request\x20o\
    bject\x20for\x20ByteStream.Write.\n\n\x0b\n\x03\x04\x02\x01\x12\x04\x80\
    \x01\x08\x14\n\xd4\x01\n\x04\x04\x02\x02\0\x12\x04\x84\x01\x02\x1b\x1a\
    \xc5\x01\x20The\x20name\x20of\x20the\x20resource\x20to\x20write.\x20This\
    \x20**must**\x20be\x20set\x20on\x20the\x20first\n\x20`WriteRequest`\x20o\
    f\x20each\x20`Write()`\x20action.\x20If\x20it\x20is\x20set\x20on\x20subs\
    equent\x20calls,\n\x20it\x20**must**\x20match\x20the\x20value\x20of\x20t\
    he\x20first\x20request.\n\n\x0f\n\x05\x04\x02\x02\0\x04\x12\x06\x84\x01\
    \x02\x80\x01\x16\n\r\n\x05\x04\x02\x02\0\x05\x12\x04\x84\x01\x02\x08\n\r\
    \n\x05\x04\x02\x02\0\x01\x12\x04\x84\x01\t\x16\n\r\n\x05\x04\x02\x02\0\
    \x03\x12\x04\x84\x01\x19\x1a\n\xbf\x04\n\x04\x04\x02\x02\x01\x12\x04\x92\
    \x01\x02\x19\x1a\xb0\x04\x20The\x20offset\x20from\x20the\x20beginning\
    \x20of\x20the\x20resource\x20at\x20which\x20the\x20data\x20should\x20be\
    \n\x20written.\x20It\x20is\x20required\x20on\x20all\x20`WriteRequest`s.\
    \n\n\x20In\x20the\x20first\x20`WriteRequest`\x20of\x20a\x20`Write()`\x20\
    action,\x20it\x20indicates\n\x20the\x20initial\x20offset\x20for\x20the\
    \x20`Write()`\x20call.\x20The\x20value\x20**must**\x20be\x20equal\x20to\
    \n\x20the\x20`committed_size`\x20that\x20a\x20call\x20to\x20`QueryWriteS\
    tatus()`\x20would\x20return.\n\n\x20On\x20subsequent\x20calls,\x20this\
    \x20value\x20**must**\x20be\x20set\x20and\x20**must**\x20be\x20equal\x20\
    to\n\x20the\x20sum\x20of\x20the\x20first\x20`write_offset`\x20and\x20the\
    \x20sizes\x20of\x20all\x20`data`\x20bundles\n\x20sent\x20previously\x20o\
    n\x20this\x20stream.\n\n\x20An\x20incorrect\x20value\x20will\x20cause\
    \x20an\x20error.\n\n\x0f\n\x05\x04\x02\x02\x01\x04\x12\x06\x92\x01\x02\
    \x84\x01\x1b\n\r\n\x05\x04\x02\x02\x01\x05\x12\x04\x92\x01\x02\x07\n\r\n\
    \x05\x04\x02\x02\x01\x01\x12\x04\x92\x01\x08\x14\n\r\n\x05\x04\x02\x02\
    \x01\x03\x12\x04\x92\x01\x17\x18\n\xad\x01\n\x04\x04\x02\x02\x02\x12\x04\
    \x97\x01\x02\x18\x1a\x9e\x01\x20If\x20`true`,\x20this\x20indicates\x20th\
    at\x20the\x20write\x20is\x20complete.\x20Sending\x20any\n\x20`WriteReque\
    st`s\x20subsequent\x20to\x20one\x20in\x20which\x20`finish_write`\x20is\
    \x20`true`\x20will\n\x20cause\x20an\x20error.\n\n\x0f\n\x05\x04\x02\x02\
    \x02\x04\x12\x06\x97\x01\x02\x92\x01\x19\n\r\n\x05\x04\x02\x02\x02\x05\
    \x12\x04\x97\x01\x02\x06\n\r\n\x05\x04\x02\x02\x02\x01\x12\x04\x97\x01\
    \x07\x13\n\r\n\x05\x04\x02\x02\x02\x03\x12\x04\x97\x01\x16\x17\n\x84\x02\
    \n\x04\x04\x02\x02\x03\x12\x04\x9d\x01\x02\x12\x1a\xf5\x01\x20A\x20porti\
    on\x20of\x20the\x20data\x20for\x20the\x20resource.\x20The\x20client\x20*\
    *may**\x20leave\x20`data`\n\x20empty\x20for\x20any\x20given\x20`WriteReq\
    uest`.\x20This\x20enables\x20the\x20client\x20to\x20inform\x20the\n\x20s\
    ervice\x20that\x20the\x20request\x20is\x20still\x20live\x20while\x20it\
    \x20is\x20running\x20an\x20operation\x20to\n\x20generate\x20more\x20data\
    .\n\n\x0f\n\x05\x04\x02\x02\x03\x04\x12\x06\x9d\x01\x02\x97\x01\x18\n\r\
    \n\x05\x04\x02\x02\x03\x05\x12\x04\x9d\x01\x02\x07\n\r\n\x05\x04\x02\x02\
    \x03\x01\x12\x04\x9d\x01\x08\x0c\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\
    \x9d\x01\x0f\x11\n5\n\x02\x04\x03\x12\x06\xa1\x01\0\xa4\x01\x01\x1a'\x20\
    Response\x20object\x20for\x20ByteStream.Write.\n\n\x0b\n\x03\x04\x03\x01\
    \x12\x04\xa1\x01\x08\x15\nT\n\x04\x04\x03\x02\0\x12\x04\xa3\x01\x02\x1b\
    \x1aF\x20The\x20number\x20of\x20bytes\x20that\x20have\x20been\x20process\
    ed\x20for\x20the\x20given\x20resource.\n\n\x0f\n\x05\x04\x03\x02\0\x04\
    \x12\x06\xa3\x01\x02\xa1\x01\x17\n\r\n\x05\x04\x03\x02\0\x05\x12\x04\xa3\
    \x01\x02\x07\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\xa3\x01\x08\x16\n\r\n\
    \x05\x04\x03\x02\0\x03\x12\x04\xa3\x01\x19\x1a\n?\n\x02\x04\x04\x12\x06\
    \xa7\x01\0\xaa\x01\x01\x1a1\x20Request\x20object\x20for\x20ByteStream.Qu\
    eryWriteStatus.\n\n\x0b\n\x03\x04\x04\x01\x12\x04\xa7\x01\x08\x1f\nO\n\
    \x04\x04\x04\x02\0\x12\x04\xa9\x01\x02\x1b\x1aA\x20The\x20name\x20of\x20\
    the\x20resource\x20whose\x20write\x20status\x20is\x20being\x20requested.\
    \n\n\x0f\n\x05\x04\x04\x02\0\x04\x12\x06\xa9\x01\x02\xa7\x01!\n\r\n\x05\
    \x04\x04\x02\0\x05\x12\x04\xa9\x01\x02\x08\n\r\n\x05\x04\x04\x02\0\x01\
    \x12\x04\xa9\x01\t\x16\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\xa9\x01\x19\
    \x1a\n@\n\x02\x04\x05\x12\x06\xad\x01\0\xb4\x01\x01\x1a2\x20Response\x20\
    object\x20for\x20ByteStream.QueryWriteStatus.\n\n\x0b\n\x03\x04\x05\x01\
    \x12\x04\xad\x01\x08\x20\nT\n\x04\x04\x05\x02\0\x12\x04\xaf\x01\x02\x1b\
    \x1aF\x20The\x20number\x20of\x20bytes\x20that\x20have\x20been\x20process\
    ed\x20for\x20the\x20given\x20resource.\n\n\x0f\n\x05\x04\x05\x02\0\x04\
    \x12\x06\xaf\x01\x02\xad\x01\"\n\r\n\x05\x04\x05\x02\0\x05\x12\x04\xaf\
    \x01\x02\x07\n\r\n\x05\x04\x05\x02\0\x01\x12\x04\xaf\x01\x08\x16\n\r\n\
    \x05\x04\x05\x02\0\x03\x12\x04\xaf\x01\x19\x1a\n\x9f\x01\n\x04\x04\x05\
    \x02\x01\x12\x04\xb3\x01\x02\x14\x1a\x90\x01\x20`complete`\x20is\x20`tru\
    e`\x20only\x20if\x20the\x20client\x20has\x20sent\x20a\x20`WriteRequest`\
    \x20with\n\x20`finish_write`\x20set\x20to\x20true,\x20and\x20the\x20serv\
    er\x20has\x20processed\x20that\x20request.\n\n\x0f\n\x05\x04\x05\x02\x01\
    \x04\x12\x06\xb3\x01\x02\xaf\x01\x1b\n\r\n\x05\x04\x05\x02\x01\x05\x12\
    \x04\xb3\x01\x02\x06\n\r\n\x05\x04\x05\x02\x01\x01\x12\x04\xb3\x01\x07\
    \x0f\n\r\n\x05\x04\x05\x02\x01\x03\x12\x04\xb3\x01\x12\x13b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}
